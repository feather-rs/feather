use std::borrow::Cow;
use std::cell::RefCell;
use std::io::{ErrorKind, Write};
use std::iter::FromIterator;
use std::sync::Arc;
use std::time::Duration;

use commands::dispatcher::CommandDispatcher;
use commands::node::CommandNode;
use flume::{Receiver, Sender, TryIter};
use parking_lot::Mutex;
use rustyline::completion::Completer;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{
    Cmd, CompletionType, ConditionalEventHandler, Context, Editor, Event, EventContext,
    EventHandler, Helper, KeyCode, KeyEvent, RepeatCount,
};
use slab::Slab;

use common::Game;
use ecs::{Entity, HasResources};
use feather_commands::CommandCtx;

const PROMPT: &str = "/";
const HISTORY_FILE: &str = ".feather_command_history";

pub struct ConsoleInput {
    commands_receiver: Receiver<String>,
    stdout: Receiver<u8>,
    line: Arc<Mutex<String>>,
    tab_completion_sender: Sender<(usize, Vec<String>)>,
    tab_completion_receiver: Receiver<String>,
    command_graph: Sender<Slab<CommandNode>>,
}

impl ConsoleInput {
    pub fn new(stdout: Receiver<u8>) -> ConsoleInput {
        let (command_sender, command_receiver) = flume::unbounded();

        let (tab_sender, tab_receiver) = flume::bounded(1);
        let (tab_sender_2, tab_receiver_2) = flume::bounded(1);

        let (command_graph_sender, command_graph_receiver) = flume::unbounded();

        let line = Arc::new(Mutex::new(String::new()));
        let line1 = line.clone();

        tokio::spawn(async move {
            let mut rl = Editor::<CommandHelper>::new();
            if rl.load_history(HISTORY_FILE).is_err() {
                log::info!("No previous console command history.")
            }
            rl.set_auto_add_history(true);
            rl.set_completion_type(CompletionType::List);
            rl.set_helper(Some(CommandHelper {
                tab_sender,
                tab_receiver: tab_receiver_2,
                command_graph: Default::default(),
                command_graph_receiver,
            }));
            let line = line1;
            rl.bind_sequence(
                Event::Any,
                EventHandler::Conditional(Box::new(AsyncHandler(line.clone()))),
            );
            loop {
                let s = rl.readline(PROMPT);
                match s {
                    Ok(s) => {
                        *line.lock() = String::new();
                        command_sender.send(s).unwrap();
                        rl.append_history(HISTORY_FILE).unwrap();
                    }
                    Err(ReadlineError::Interrupted) => {
                        std::process::exit(0);
                        // TODO shutdown
                    }
                    _ => (),
                };
            }
        });

        ConsoleInput {
            commands_receiver: command_receiver,
            stdout,
            line,
            tab_completion_sender: tab_sender_2,
            tab_completion_receiver: tab_receiver,
            command_graph: command_graph_sender,
        }
    }
    pub fn try_iter(&self) -> TryIter<String> {
        self.commands_receiver.try_iter()
    }
    pub fn flush_stdout(&self) {
        flush_stdout(&self.stdout, &self.line.lock())
    }
    pub fn tab_complete_if_needed(&self, game: &mut Game, console: Entity) {
        while let Ok(line) = self.tab_completion_receiver.try_recv() {
            if let Ok(dispatcher) = game.resources().get::<CommandDispatcher<CommandCtx>>() {
                let dispatcher = &*dispatcher;
                let completions = feather_commands::tab_complete(dispatcher, game, console, &line);
                if !completions.2.is_empty() {
                    self.tab_completion_sender
                        .send((
                            completions.0,
                            completions
                                .2
                                .into_iter()
                                .map(|(completion, _tooltip)| completion)
                                .collect(),
                        ))
                        .unwrap();
                    return;
                }
            }
            self.tab_completion_sender.send((0, vec![])).unwrap();
        }
    }

    pub fn update_command_graph(&self, graph: &CommandDispatcher<CommandCtx>) {
        self.command_graph
            .send(Slab::from_iter(graph.nodes().map(|(i, node)| {
                (
                    i,
                    match node {
                        CommandNode::Root { children } => CommandNode::Root {
                            children: children.clone(),
                        },
                        CommandNode::Literal {
                            execute,
                            name,
                            children,
                            parent,
                        } => CommandNode::Literal {
                            execute: *execute,
                            name: name.clone(),
                            children: children.clone(),
                            parent: *parent,
                        },
                        CommandNode::Argument {
                            execute,
                            name,
                            suggestions_type,
                            parser,
                            children,
                            parent,
                        } => CommandNode::Argument {
                            execute: *execute,
                            name: name.clone(),
                            suggestions_type: suggestions_type.clone(),
                            parser: parser.clone_boxed(),
                            children: children.clone(),
                            parent: *parent,
                        },
                    },
                )
            })))
            .unwrap();
    }
}

pub fn flush_stdout(queue: &Receiver<u8>, line: &str) {
    let mut stdout = std::io::stdout();
    let mut wrote = false;
    for message in queue.try_iter() {
        if !wrote {
            wrote = true;
            stdout.write_all(b"\x1b[2K").unwrap(); // erase line
            stdout.write_all(b"\x1b[1G").unwrap(); // move cursor to the beginning of the line
        }
        stdout.write_all(&[message]).unwrap();
    }
    if wrote {
        stdout.write_all(b"\x1b[1G").unwrap(); // move cursor to the beginning of the line
        stdout.write_all(PROMPT.as_bytes()).unwrap();
        stdout.write_all(line.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
}

struct AsyncHandler(Arc<Mutex<String>>);

impl ConditionalEventHandler for AsyncHandler {
    fn handle(&self, evt: &Event, _: RepeatCount, _: bool, _: &EventContext) -> Option<Cmd> {
        if let Some(KeyEvent(KeyCode::Char(c), _)) = evt.get(0) {
            self.0.lock().push(*c);
            None
        } else {
            None
        }
    }
}

struct CommandHelper {
    tab_sender: Sender<String>,
    tab_receiver: Receiver<(usize, Vec<String>)>,
    /// a temporary copy of the server's command dispatcher (used for faster command highlighting)
    command_graph: RefCell<CommandDispatcher<()>>,
    command_graph_receiver: Receiver<Slab<CommandNode>>,
}

impl Validator for CommandHelper {}

impl Highlighter for CommandHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        const RESET: &str = "\x1B[0m";
        const RED: &str = "\x1B[31;1m";
        const ARGUMENT_COLORS: [&str; 5] = [
            "\x1B[34;1m",
            "\x1B[33;1m",
            "\x1B[32;1m",
            "\x1B[31;5m",
            "\x1B[33m",
        ];

        fn children(node: &CommandNode) -> &Vec<usize> {
            match node {
                CommandNode::Root { children }
                | CommandNode::Literal { children, .. }
                | CommandNode::Argument { children, .. } => children,
            }
        }
        fn matches<'a>(node: &CommandNode, s: &'a str) -> (bool, &'a str) {
            match node {
                CommandNode::Root { .. } => unreachable!(),
                CommandNode::Literal { name, .. } => {
                    if s == name {
                        (true, &s[s.len()..])
                    } else if s.starts_with(&format!("{} ", name)) {
                        (true, &s[name.len() + 1..])
                    } else {
                        (false, "")
                    }
                }
                CommandNode::Argument { parser, .. } => {
                    if let Some((i, _)) = parser.parse(s) {
                        (true, &s[i..])
                    } else {
                        (false, "")
                    }
                }
            }
        }

        while let Ok(nodes) = self.command_graph_receiver.try_recv() {
            *self.command_graph.borrow_mut() = CommandDispatcher::default();
            self.command_graph
                .borrow_mut()
                .add_nodes(nodes.into_iter().map(|(_, node)| node).collect());
        }

        let mut i = 0;
        let mut result = String::new();

        let commands = self.command_graph.borrow();
        let mut command = line;
        let mut node = commands.nodes().next().unwrap().1; // root node is always first
        'next: loop {
            for child in children(node) {
                let n = commands.nodes().nth(*child).unwrap().1;
                if let (true, s) = matches(n, command) {
                    if matches!(n, CommandNode::Argument { .. }) {
                        result += ARGUMENT_COLORS[i];
                        result += &command[0..command.len() - s.len()];
                        i += 1;
                    } else {
                        result += RESET;
                        result += &command[0..command.len() - s.len()];
                    }
                    node = commands.nodes().nth(*child).unwrap().1;
                    command = s;
                    continue 'next;
                }
            }
            if !command.is_empty() {
                result += RED;
                result += command;
            }
            break;
        }
        result += RESET;

        Cow::Owned(result)
    }
    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        line.chars().nth(pos) != Some(' ')
    }
}

impl Hinter for CommandHelper {
    type Hint = String;
}

impl Completer for CommandHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.tab_sender.send(line[..pos].to_string()).unwrap();
        let completions = self
            .tab_receiver
            .recv_timeout(Duration::from_secs(10))
            .map_err(|_| {
                log::warn!("The server didn't respond for tab-completion request in 10 seconds");
                ReadlineError::Io(std::io::Error::new(
                    ErrorKind::TimedOut,
                    "The tab-completion request has timed out",
                ))
            });
        self.tab_receiver.drain();
        completions
    }
}

impl Helper for CommandHelper {}

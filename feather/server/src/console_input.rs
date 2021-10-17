use std::io::{ErrorKind, Write};
use std::sync::Arc;
use std::time::Duration;

use commands::dispatcher::CommandDispatcher;
use flume::{Receiver, Sender, TryIter};
use parking_lot::Mutex;
use rustyline::completion::Completer;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{
    Cmd, ConditionalEventHandler, Context, Editor, Event, EventContext, EventHandler, Helper,
    KeyCode, KeyEvent, RepeatCount,
};

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
}

impl ConsoleInput {
    pub fn new(stdout: Receiver<u8>) -> ConsoleInput {
        let (command_sender, command_receiver) = flume::unbounded();

        let (tab_sender, tab_receiver) = flume::bounded(1);
        let (tab_sender_2, tab_receiver_2) = flume::bounded(1);

        let line = Arc::new(Mutex::new(String::new()));
        let line1 = line.clone();

        tokio::spawn(async move {
            let mut rl = Editor::<CommandHelper>::new();
            if rl.load_history(HISTORY_FILE).is_err() {
                log::info!("No previous console command history.")
            }
            rl.set_auto_add_history(true);
            rl.set_helper(Some(CommandHelper {
                tab_sender,
                tab_receiver: tab_receiver_2,
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
}

impl Validator for CommandHelper {}

impl Highlighter for CommandHelper {}

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

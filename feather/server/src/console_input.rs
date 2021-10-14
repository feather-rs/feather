use std::io::Write;
use std::sync::Arc;

use flume::{Receiver, TryIter};
use parking_lot::Mutex;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{
    Cmd, ConditionalEventHandler, Context, Editor, Event, EventContext, EventHandler, Helper,
    KeyCode, KeyEvent, RepeatCount,
};

use common::Game;
use ecs::{Entity, EntityBuilder, HasResources};
use feather_commands::{CommandCtx, CommandDispatcher};
use quill_common::components::{ChatBox, ChatPreference, Console, Name};

const PROMPT: &str = "/";
const HISTORY_FILE: &str = ".feather_command_history";

pub struct ConsoleInput {
    commands_receiver: Receiver<String>,
    stdout: Receiver<u8>,
    line: Arc<Mutex<String>>,
}

impl ConsoleInput {
    pub fn new(stdout: Receiver<u8>, game: Arc<Mutex<Game>>) -> ConsoleInput {
        // Create the console entity so the console can receive messages
        let mut console = EntityBuilder::new();
        console
            .add(Console)
            .add(Name::new("Console"))
            .add(ChatBox::new(ChatPreference::All));

        // We can use the raw spawn method because
        // the console isn't a "normal" entity.
        let _console = game.lock().ecs.spawn(console.build());

        let (sender, receiver) = flume::unbounded();

        let line = Arc::new(Mutex::new(String::new()));
        let line1 = line.clone();

        tokio::spawn(async move {
            let mut rl = Editor::<CommandHelper>::new();
            rl.load_history(HISTORY_FILE).ok();
            // TODO find a way to get mutable access to `Game` on this thread to pass it to
            // `CommandCtx` to enable tab-completions. Mutex doesn't work because systems,
            // entity spawn callbacks and resources don't implement Send + Sync, and there's
            // a problem with `Resources`.
            //rl.set_helper(Some(CommandHelper { console, game }));
            let line = line1;
            rl.bind_sequence(
                Event::Any,
                EventHandler::Conditional(Box::new(AsyncHandler(line.clone()))),
            );
            loop {
                let s = rl.readline(PROMPT);
                match s {
                    Ok(s) => {
                        {
                            let mut guard = line.lock();
                            *guard = String::new();
                        }
                        sender.send(s).unwrap();
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
            commands_receiver: receiver,
            stdout,
            line,
        }
    }
    pub fn try_iter(&self) -> TryIter<String> {
        self.commands_receiver.try_iter()
    }
    pub fn flush_stdout(&self) {
        flush_stdout(&self.stdout, &self.line.lock())
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

pub struct CommandHelper {
    console: Entity,
    game: Arc<Mutex<Game>>,
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
        let game = &mut *self.game.lock();
        let line = &line[..pos];
        if let Ok(dispatcher) = game.resources().get::<CommandDispatcher<CommandCtx>>() {
            let dispatcher = &*dispatcher;
            let completions = feather_commands::tab_complete(dispatcher, game, self.console, line);
            if !completions.is_empty() {
                let n = line.rfind(' ').unwrap_or(1);
                return Ok((
                    n,
                    completions
                        .into_iter()
                        .map(|(completion, _tooltip)| completion)
                        .collect(),
                ));
            }
        }
        Ok((0, vec![]))
    }
}

impl Helper for CommandHelper {}

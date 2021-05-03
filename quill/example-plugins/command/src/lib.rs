use lieutenant::command::builder::{literal, CommandBuilder};
use quill::{components::Name, CommandContext, Game, Plugin, Setup};

quill::plugin!(SimpleCommand);

struct SimpleCommand;

impl Plugin for SimpleCommand {
    fn enable(_game: &mut Game, setup: &mut Setup<Self>) -> Self {
        setup.add_command(literal("/echo").space().arg::<u32>().on_call(|x: u32| {
            move |_plugin: &mut Self, ctx: &mut CommandContext| {
                match &ctx.caller {
                    quill::Caller::Player(player) => {
                        let name = player.get::<Name>().unwrap();
                        player.send_message(format!("Hi {} your number was {}", name, x));
                    }
                    quill::Caller::Terminal => {
                        println!("Hi terminal, the number was {}", x);
                    }
                }

                42
            }
        }));

        SimpleCommand
    }

    fn disable(self, _game: &mut Game) {}
}

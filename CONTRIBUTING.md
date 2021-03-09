# Contributing to Feather

Feather is an ambitious project, and contributions are always welcome!

If you want to work on the codebase, please keep the following in mind:
* Run `rustfmt` on your code before committing. The CI build will fail if rustfmt detects formatting errors.
* Run [`clippy`](https://github.com/rust-lang/rust-clippy) on your code and fix any warnings it gives. Clippy can detect common mistakes, and as with formatting, the build will fail if there are Clippy warnings.
* Where possible and necessary, please write tests.
* Run `cargo test` before committing to ensure you have not broken anything.

## Notes to your code

Here are things to look at before requesting a merge:

* **Remove your own unused, commented out code:** Remove commented out code used for testing purposes.
* **Tracing actions:** If tracing i.e. a players actions, make sure to include an identifier to the player, don't just trace in general!

# Original code (code from Minecraft)

> 🛑 **Do not use any of code based of Minecraft's source**: Please do not write code that is in any way inspired, based on, or taken from Mojang's work, including but not limited to
the vanilla server and client. Feather is a "clean-room" implementation, meaning that it is written
from scratch without any involvement with proprietary code. By using code from Mojang, the project
would become prone to legal issues.
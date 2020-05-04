mod plugin;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn plugin_init(args: TokenStream, input: TokenStream) -> TokenStream {
    plugin::plugin(args, input)
}

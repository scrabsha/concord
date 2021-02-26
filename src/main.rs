mod init;
mod switch;
pub mod utils;

use std::env;

use seahorse::App;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    App::new("concord")
        .description("Allows to handle multiple Discord accounts on the same system.")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .command(init::command())
        .command(switch::command())
        .run(args)
}

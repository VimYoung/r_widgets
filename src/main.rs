use chunks_rs::GtkCmdLineExt;
use chunks_rs::{utils::load_css, Factory, GtkApp, GtkCmdLine};
use r_widgets::{show_hello, storage};
use std::env;

const STYLE: &str = include_str!("style.css");

fn main() {
    let factory = Factory::new("chunk.factory");

    let chunks = |app: &GtkApp, cmd_line: &GtkCmdLine| {
        let args = cmd_line.arguments();
        if args.len() < 2 {
            eprintln!("No command provided");
            return 1;
        }

        match args[1].to_str().unwrap_or_default() {
            "storage" => {
                storage(app);
                load_css(STYLE);
                0
            }
            "show-hello" => {
                show_hello(app);
                load_css(STYLE);
                0
            }
            arg => {
                eprintln!("Unknown argument: {arg}");
                1
            }
        }
    };

    // It may seem redundant to collect the args a second time, but to avoid type errors, it is necessary.
    let args: Vec<String> = env::args().collect();

    factory.pollute_with_args(chunks, args);
}

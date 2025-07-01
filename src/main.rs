mod cli;
mod generator;
mod pattern;
mod scanner;
mod validator;

fn main() {
    env_logger::init();
    cli::run();
}

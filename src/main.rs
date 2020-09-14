pub mod chunk;
pub mod chunk_type;
pub mod png;
mod args;
mod commands;

type Error = &'static str;
type Result<T> = std::result::Result<T, Error>;

fn main() {
    args::process_args();
}

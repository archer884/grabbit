use clap::Parser;

/// Get media URLs from user posts.
#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// The name of a Reddit user.
    pub user: String,
}

pub fn read() -> Args {
    Parser::parse()
}

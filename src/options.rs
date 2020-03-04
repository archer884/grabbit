use structopt::StructOpt;

/// Get media URLs from user posts.
#[derive(Clone, Debug, StructOpt)]
pub struct Opt {
    /// The name of a Reddit user.
    pub user: String,
}

pub fn read() -> Opt {
    StructOpt::from_args()
}

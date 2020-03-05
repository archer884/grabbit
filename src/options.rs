use structopt::StructOpt;

/// Get media URLs from user posts.
#[derive(Clone, Debug, StructOpt)]
pub struct Opt {
    /// The name of a Reddit user.
    pub user: String,

    /// Print debug information for Post items.
    #[structopt(short = "V", long = "verbose")]
    pub verbose: bool,
}

pub fn read() -> Opt {
    StructOpt::from_args()
}

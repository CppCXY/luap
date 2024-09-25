use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "luap", about = "Lua package manager")]
pub enum CommandOpt {
    Install {
        #[structopt(long)]
        dump_library: bool,
    },
    Check {
        #[structopt(long)]
        dump_library: bool,
    },
    Add {
        package: String,
        github: String,
        #[structopt(long)]
        dev: bool,
        #[structopt(long)]
        hash: Option<String>,
        #[structopt(long)]
        tag: Option<String>,
        #[structopt(long)]
        branch: Option<String>,
    },
    Remove {
        package: String,
        #[structopt(long)]
        dev: bool
    },
    Update {
        package: String,
        #[structopt(long)]
        dev: bool,
        #[structopt(long)]
        hash: Option<String>,
        #[structopt(long)]
        tag: Option<String>,
        #[structopt(long)]
        branch: Option<String>,
    },
    Init
}

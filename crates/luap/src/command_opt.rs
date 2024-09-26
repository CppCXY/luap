use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "luap", about = "Lua package manager")]
pub enum CommandOpt {
    Install {
        #[structopt(long, help = "Dump the library information")]
        dump_library: bool,
    },
    Check {
        #[structopt(long, help = "Dump the library information")]
        dump_library: bool,
    },
    Add {
        #[structopt(help = "Name of the package to add")]
        package: String,
        #[structopt(help = "GitHub repository URL")]
        github: String,
        #[structopt(long, help = "Add the package as a development dependency")]
        dev: bool,
        #[structopt(long, help = "Specific commit hash to use")]
        hash: Option<String>,
        #[structopt(long, help = "Specific tag to use")]
        tag: Option<String>,
        #[structopt(long, help = "Specific branch to use")]
        branch: Option<String>,
    },
    Remove {
        #[structopt(help = "Name of the package to remove")]
        package: String,
        #[structopt(long, help = "Remove the package from development dependencies")]
        dev: bool,
    },
    Update {
        #[structopt(help = "Name of the package to update (optional), if not provided, update all packages")]
        package: Option<String>,
        #[structopt(long, help = "Specific commit hash to use")]
        hash: Option<String>,
        #[structopt(long, help = "Specific tag to use")]
        tag: Option<String>,
        #[structopt(long, help = "Specific branch to use")]
        branch: Option<String>,
    },
    Init,
}
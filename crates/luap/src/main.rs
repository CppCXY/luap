use structopt::StructOpt;

mod command_opt;
mod lock_file;
mod targets;

fn main() {
    let opt = command_opt::CommandOpt::from_args();

    match opt {
        command_opt::CommandOpt::Install { dump_library } => {
            targets::install::install_package(dump_library);
        }
        command_opt::CommandOpt::Check { dump_library } => {
            targets::check::check_package(dump_library);
        }
        command_opt::CommandOpt::Add {
            package,
            github,
            dev,
            hash,
            tag,
            branch,
        } => {
            if dev {
                targets::add::add_dev_package(&package, &github, branch, tag, hash);
            } else {
                targets::add::add_package(&package, &github, branch, tag, hash);
            }
        }
        command_opt::CommandOpt::Remove { package, dev } => {
            if dev {
                targets::remove::remove_dev_package(&package);
            } else {
                targets::remove::remove_package(&package);
            }
        }
        command_opt::CommandOpt::Update {
            package,
            dev,
            hash,
            tag,
            branch,
        } => {
            if dev {
                targets::update::update_dev_package(&package, branch, tag, hash);
            } else {
                targets::update::update_package(&package, branch, tag, hash);
            }
        }
        command_opt::CommandOpt::Init => {
            targets::init::init_package();
        }
    }
}

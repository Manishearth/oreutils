use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "oreutils", about = "Installation manager for various CLI utilities reimagined in Rust", rename_all = "kebab-case")]
enum Opt {
    #[structopt(about = "Install the basic utilities: ripgrep, exa, bat, fd")]
    Install,
    #[structopt(about = "Upgrade any installed tools. Use `oreutils install` to install missing ones.")]
    Upgrade,
    #[structopt(about = "Uninstall all oreutils tools")]
    Uninstall,
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Install => {
            install()
        }
        Opt::Upgrade => {
            upgrade()
        }
        Opt::Uninstall => {
            uninstall()
        }
    }
}


fn install() {
    unimplemented!()
}

fn upgrade() {
    unimplemented!()
}

fn uninstall() {
    unimplemented!()
}


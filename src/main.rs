use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "oreutils",
    about = "Installation manager for various CLI utilities reimagined in Rust",
    rename_all = "kebab-case"
)]
enum Opt {
    #[structopt(about = "Install the basic utilities: ripgrep, exa, bat, fd")]
    Install,
    #[structopt(
        about = "Upgrade any installed tools. Use `oreutils install` to install missing ones."
    )]
    Upgrade,
    #[structopt(about = "Uninstall all oreutils tools")]
    Uninstall,
}

struct Tool {
    name: &'static str,
    package: &'static str,
    cli: &'static str,
}

const TOOLS: &[Tool] = &[
    Tool {
        name: "ripgrep",
        package: "ripgrep",
        cli: "rg",
    },
    Tool {
        name: "exa",
        package: "exa",
        cli: "exa",
    },
    Tool {
        name: "bat",
        package: "bat",
        cli: "bat",
    },
    Tool {
        name: "fd",
        package: "fd-find",
        cli: "fd",
    },
];

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Install => install(),
        Opt::Upgrade => upgrade(),
        Opt::Uninstall => uninstall(),
    }
}

fn install() {
    for tool in TOOLS {
        let exists = which::which(tool.cli);
        if exists.is_ok() {
            println!(
                "Tool {:?} already installed, use `oreutils upgrade` to upgrade",
                tool.name
            );
            continue;
        }
        cargo_install(tool.package, false);
    }
}

fn upgrade() {
    unimplemented!()
}

fn uninstall() {
    unimplemented!()
}

fn cargo_install(pkg: &str, force: bool) {
    let mut cmd = Command::new("cargo");
    if force {
        cmd.args(&["install", "-f", pkg]);
    } else {
        cmd.args(&["install", pkg]);
    }
    cmd.env("RUSTFLAGS", "-Ctarget-cpu=native");
    let res = cmd.spawn();
    if res.is_err() {
        eprintln!("Installing {:?} failed", pkg);
    }
}

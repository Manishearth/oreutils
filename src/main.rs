use regex::Regex;
use semver::Version;
use std::process::Command;
use structopt::StructOpt;

mod fetch;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "oreutils",
    about = "Installation manager for various CLI utilities reimagined in Rust",
    rename_all = "kebab-case"
)]
enum Opt {
    #[structopt(about = "Install the basic utilities: ripgrep, exa, bat, fd")]
    Install {
        #[structopt(help = "Specific tool to install. Omit to install all.")]
        tool: Option<String>,
    },
    #[structopt(
        about = "Upgrade any installed tools. Use `oreutils install` to install missing ones."
    )]
    Upgrade {
        #[structopt(help = "Specific tool to upgrade. Omit to upgrade all.")]
        tool: Option<String>,
    },
    #[structopt(about = "Uninstall all oreutils tools")]
    Uninstall,
}

#[derive(Clone, Copy)]
struct Tool {
    name: &'static str,
    package: &'static str,
    cli: &'static str,
}

impl Tool {
    fn equals(&self, other: &str) -> bool {
        self.name == other || self.package == other || self.cli == other
    }
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
        Opt::Install {tool} => install(tool),
        Opt::Upgrade {tool} => upgrade(tool),
        Opt::Uninstall => uninstall(),
    }
}

fn install(tool: Option<String>) {
    for_each_tool(tool, |tool| {
        let exists = which::which(tool.cli);
        if exists.is_ok() {
            println!(
                "Tool {:?} already installed, use `oreutils upgrade` to upgrade",
                tool.name
            );
            return;
        }
        cargo_install(tool.package, false);
    });
}


fn for_each_tool<F: Fn(&Tool)>(tool: Option<String>, f: F) {
    if let Some(tool) = tool {
        for tool in TOOLS.iter().filter(|x| x.equals(&tool)) {
            f(tool)
        }
    } else {
        for tool in TOOLS.iter()  {
            f(tool)
        }
    };
}

fn upgrade(tool: Option<String>) {
    for_each_tool(tool, |tool| {
        let res = upgrade_tool(tool);
        match res {
            Ok(vers) => println!("Tool {} updated to version {}", tool.name, vers),
            Err(Error::NotFound) => println!(
                "Tool {} not installed, use `oreutils install` to install",
                tool.name
            ),
            Err(Error::VersionBroken(None)) => {
                println!("`{} --version` didn't produce expected output", tool.cli)
            }
            Err(Error::VersionBroken(Some(v))) => println!(
                "`{} --version` didn't produce expected output: could not parse {}",
                tool.cli, v
            ),
            Err(Error::AlreadyUpdated(v)) => println!("Tool {} is already up to date at version {}", tool.name, v),
            Err(Error::CratesFetchError(e)) => println!(
                "Failed to fetch information for crate {} from crates.io: {}",
                tool.name, e
            ),
        }
    });
}

enum Error {
    NotFound,
    VersionBroken(Option<String>),
    CratesFetchError(fetch::FetchError),
    AlreadyUpdated(Version)
}

fn upgrade_tool(tool: &Tool) -> Result<Version, Error> {
    let output = Command::new(tool.cli)
        .args(&["--version"])
        .output()
        .map_err(|_| Error::NotFound)?;
    let output = String::from_utf8(output.stdout).map_err(|_| Error::VersionBroken(None))?;
    let output = output.lines().next().ok_or(Error::VersionBroken(None))?;
    let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    let vers = re
        .find(output)
        .ok_or(Error::VersionBroken(Some(output.into())))?;
    let vers = vers.as_str();
    let vers = Version::parse(vers).map_err(|_| Error::VersionBroken(Some(vers.into())))?;
    let latest_vers =
        fetch::get_latest_version(tool.package).map_err(|e| Error::CratesFetchError(e))?;

    if vers < latest_vers {
        cargo_install(tool.package, true);
        Ok(latest_vers)
    } else {
        Err(Error::AlreadyUpdated(vers))
    }
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
    match res {
        Ok(mut child) => {
            let status = child.wait().expect("Command wasn't running");
            if !status.success() {
                eprintln!("Installing {:?} failed", pkg);
            }
        }
        Err(_) => eprintln!("Cargo didn't start"),
    }
}

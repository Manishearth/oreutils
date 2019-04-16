// This file modified from https://github.com/killercup/cargo-edit/blob/master/src/fetch.rs
// (cargo-edit depends on a bunch of extra stuff I'd like to avoid)

use env_proxy;
use reqwest;
use semver::Version;
use serde_json as json;
use std::fmt;
use std::time::Duration;

use serde_derive::Deserialize;

const REGISTRY_HOST: &str = "https://crates.io";

#[derive(Deserialize)]
struct Versions {
    versions: Vec<CrateVersion>,
}

#[derive(Deserialize)]
struct CrateVersion {
    #[serde(rename = "crate")]
    _name: String,
    #[serde(rename = "num")]
    version: Version,
    yanked: bool,
}

pub enum FetchError {
    NoCrate,
    BadResponse,
    NoVersions,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FetchError::NoCrate => write!(f, "No crate found"),
            FetchError::BadResponse => write!(f, "Bad response"),
            FetchError::NoVersions => write!(f, "Crate has no release versions"),
        }
    }
}

pub fn get_latest_version(crate_name: &str) -> Result<Version, FetchError> {
    Ok(fetch_cratesio(crate_name)?
        .versions
        .into_iter()
        .filter(|v| !v.version.is_prerelease())
        .find(|v| !v.yanked)
        .ok_or(FetchError::NoVersions)?
        .version)
}

fn fetch_cratesio(crate_name: &str) -> Result<Versions, FetchError> {
    let url = format!(
        "{host}/api/v1/crates/{crate_name}",
        host = REGISTRY_HOST,
        crate_name = crate_name
    );

    match get_with_timeout(&url, Duration::from_secs(10)) {
        Ok(response) => Ok(json::from_reader(response).map_err(|_| FetchError::BadResponse)?),
        Err(e) => {
            let not_found_error = e.status() == Some(reqwest::StatusCode::NOT_FOUND);

            if not_found_error {
                Err(FetchError::NoCrate)
            } else {
                Err(FetchError::BadResponse)
            }
        }
    }
}

fn get_with_timeout(url: &str, timeout: Duration) -> reqwest::Result<reqwest::Response> {
    let client = reqwest::ClientBuilder::new()
        .timeout(timeout)
        .proxy(reqwest::Proxy::custom(|url| {
            env_proxy::for_url(url).to_url()
        }))
        .build()?;

    client
        .get(url)
        .send()
        .and_then(|resp| resp.error_for_status())
}

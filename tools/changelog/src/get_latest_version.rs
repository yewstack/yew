use anyhow::Result;
use git2::Repository;
use semver::{Error, Version};

use crate::yew_package::YewPackage;

pub fn get_latest_version(package: &YewPackage) -> Result<Version> {
    let common_tag_pattern = format!("{}-v", package);
    let search_pattern = format!("{}*", common_tag_pattern);

    let mut tags: Vec<Version> = Repository::open_from_env()?
        .tag_names(Some(&search_pattern))?
        .iter()
        .filter_map(|mb_tag| {
            mb_tag.map(|tag| {
                let version = tag.replace(&common_tag_pattern, "");
                Version::parse(&version)
            })
        })
        .collect::<Result<Vec<Version>, Error>>()?;

    tags.sort();
    tags.reverse();

    Ok(tags[0].clone())
}

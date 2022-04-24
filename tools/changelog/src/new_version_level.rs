use semver::Version;
use strum::{Display, EnumString};

#[derive(Debug, Clone, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum NewVersionLevel {
    Patch,
    Minor,
    Major,
}

impl NewVersionLevel {
    pub fn bump(&self, current_version: Version) -> Version {
        match self {
            NewVersionLevel::Patch => Version {
                patch: current_version.patch + 1,
                ..current_version
            },
            NewVersionLevel::Minor => Version {
                minor: current_version.minor + 1,
                patch: 0,
                ..current_version
            },
            NewVersionLevel::Major => Version {
                major: current_version.major + 1,
                minor: 0,
                patch: 0,
                ..current_version
            },
        }
    }
}

//! In core-kit 6.4.0, we introduced a change to delete all configuration-files and services on
//! whenever the migrator runs (https://github.com/bottlerocket-os/bottlerocket-core-kit/pull/456).
//!
//! This migrations ensures that nodes downgrading to versions prior to core-kit 6.4.0 will delete
//! and re-populate these keys.
use migration_helpers::{migrate, Migration, MigrationData, Result};

const PREFIXES_TO_DELETE: &[&str] = &["configuration-files.", "services."];

#[snafu::report]
fn main() -> Result<()> {
    migrate(DeleteConfigsAndServicesOnDowngradeMigration)
}

pub struct DeleteConfigsAndServicesOnDowngradeMigration;

impl Migration for DeleteConfigsAndServicesOnDowngradeMigration {
    fn forward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!("DeleteConfigsAndServicesOnDowngradeMigration has no work to do on upgrade.",);
        Ok(input)
    }

    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        input.data.retain(|key, _| {
            let to_keep = !(PREFIXES_TO_DELETE
                .iter()
                .any(|prefix| key.starts_with(prefix)));
            if !to_keep {
                println!("Removed '{key}'");
            }
            to_keep
        });
        Ok(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn nothing_to_clear() {
        let data = MigrationData {
            data: hashmap! {
                "settings.hello".into() => "there".into(),
                "settings.something.configuration-files".into() => "retain this!".into(),
                "settings.something.services".into() => "and this!".into(),
            },
            metadata: HashMap::new(),
        };
        let result = DeleteConfigsAndServicesOnDowngradeMigration
            .backward(data)
            .unwrap();
        assert_eq!(
            result.data,
            hashmap! {
                "settings.hello".into() => "there".into(),
                "settings.something.configuration-files".into() => "retain this!".into(),
                "settings.something.services".into() => "and this!".into(),
            }
        );
    }

    #[test]
    fn all_clear() {
        let data = MigrationData {
            data: hashmap! {
                "services.delete-this".into() => "yep".into(),
                "configuration-files.delete-this".into() => "this too".into(),
                "configuration-files.another-one".into() => "bye".into(),
                "services.and-this".into() => "au revoir".into(),
            },
            metadata: HashMap::new(),
        };
        let result = DeleteConfigsAndServicesOnDowngradeMigration
            .backward(data)
            .unwrap();
        assert_eq!(result.data, HashMap::new());
    }

    #[test]
    fn delete_some() {
        let data = MigrationData {
            data: hashmap! {
                "services.delete-this".into() => "deleted".into(),
                "configuration-files.and-this".into() => "deleted".into(),
                "settings.but-not-this".into() => "stays".into(),
                "or-this-either.configuration-files".into() => "also-stays".into(),
            },
            metadata: HashMap::new(),
        };
        let result = DeleteConfigsAndServicesOnDowngradeMigration
            .backward(data)
            .unwrap();
        assert_eq!(
            result.data,
            hashmap! {
                "settings.but-not-this".into() => "stays".into(),
                "or-this-either.configuration-files".into() => "also-stays".into(),
            }
        );
    }

    #[test]
    fn dont_touch_the_metadata() {
        let data = MigrationData {
            data: hashmap! {
                "configuration-files.delete".into() => "delete".into(),
                "services.delete".into() => "delete".into(),
                "settings.keep".into() => "keep".into(),
            },
            metadata: hashmap! {
                "configuration-files.delete".into() => hashmap! {
                    "keep".into() => "yep!".into(),
                },
                "services.delete".into() => hashmap! {
                    "keep".into() => "yep!".into(),
                },
                "settings.keep".into() => hashmap! {
                    "keep".into() => "yep!".into(),
                },
            },
        };
        let result = DeleteConfigsAndServicesOnDowngradeMigration
            .backward(data)
            .unwrap();
        assert_eq!(
            result,
            MigrationData {
                data: hashmap! {
                    "settings.keep".into() => "keep".into(),
                },
                metadata: hashmap! {
                    "configuration-files.delete".into() => hashmap! {
                        "keep".into() => "yep!".into(),
                    },
                    "services.delete".into() => hashmap! {
                        "keep".into() => "yep!".into(),
                    },
                    "settings.keep".into() => hashmap! {
                        "keep".into() => "yep!".into(),
                    },
                },
            },
        );
    }
}

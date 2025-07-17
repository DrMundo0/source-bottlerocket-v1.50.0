use migration_helpers::{error, migrate, Migration, MigrationData, Result};
use snafu::OptionExt;

const DEVICE_LIST_STRATEGY_SETTING: &str =
    "settings.kubelet-device-plugins.nvidia.device-list-strategy";

#[snafu::report]
fn main() -> Result<()> {
    migrate(ReplaceDeviceListStrategy)
}

/// We changed the type of the device-list-strategy in the NVIDIA Kubernetes Device
/// plugin API from a string to a list, and accept "cdi-cri" as a valid value
pub struct ReplaceDeviceListStrategy;

impl Migration for ReplaceDeviceListStrategy {
    /// New versions must either have a default for the settings or generate them; we don't need to
    /// do anything.
    fn forward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!("ReplaceDeviceListStrategy has no work to do on upgrade.");
        Ok(input)
    }

    /// Older versions don't know about the setting now accepting both a string, a list and a new accepted value "cdi-cri";
    /// we remove the list option and the "cdi-cri" value so that old versions don't see them and fail deserialization.
    /// (The settings must be defaulted or generated in new versions, and safe to remove.)
    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        let setting = DEVICE_LIST_STRATEGY_SETTING;
        if let Some(data) = input.data.get_mut(setting) {
            match data {
                serde_json::Value::Array(arr) => {
                    let list: Vec<&str> = arr
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Option<Vec<&str>>>()
                        .context(error::ReplaceListContentsSnafu {
                            setting,
                            data: arr.clone(),
                        })?;

                    let new_value = match list.first() {
                        None | Some(&"cdi-cri") => "volume-mounts".to_string(),
                        Some(value) => value.to_string(),
                    };

                    *data = serde_json::Value::String(new_value.to_string());
                }

                serde_json::Value::String(setting_str) => {
                    if setting_str == "cdi-cri" {
                        *data = serde_json::Value::String("volume-mounts".to_string());
                    } else {
                        *data = serde_json::Value::String(setting_str.to_string());
                    }
                }

                _ => error::InvalidSettingTypeSnafu {
                    data: setting.to_string(),
                }
                .fail()?,
            }
        } else {
            println!("Found no '{setting}' to change on downgrade");
        }

        Ok(input)
    }
}

#[cfg(test)]
mod test_replace_list {
    use super::*;
    use crate::{Migration, MigrationData};
    use maplit::hashmap;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn forward_test() {
        let data = MigrationData {
            data: hashmap! {
                DEVICE_LIST_STRATEGY_SETTING.into() => "volume-mounts".into(),
            },
            metadata: HashMap::new(),
        };
        let result = ReplaceDeviceListStrategy.forward(data).unwrap();
        assert_eq!(
            result.data,
            hashmap! {
                DEVICE_LIST_STRATEGY_SETTING.into() => "volume-mounts".into(),
            }
        );
    }

    #[test]
    fn backward_test() {
        let test_cases: Vec<(MigrationData, HashMap<String, Value>)> = vec![
            (
                MigrationData {
                    data: hashmap! {
                        DEVICE_LIST_STRATEGY_SETTING.into() => vec!["cdi-cri"].into(),
                    },
                    metadata: HashMap::new(),
                },
                hashmap! {
                    DEVICE_LIST_STRATEGY_SETTING.into() => "volume-mounts".into(),
                },
            ),
            (
                MigrationData {
                    data: hashmap! {
                        DEVICE_LIST_STRATEGY_SETTING.into() => Vec::<String>::new().into(),
                    },
                    metadata: HashMap::new(),
                },
                hashmap! {
                    DEVICE_LIST_STRATEGY_SETTING.into() => "volume-mounts".into(),
                },
            ),
            (
                MigrationData {
                    data: hashmap! {
                        DEVICE_LIST_STRATEGY_SETTING.into() => vec!["volume-mounts", "envvar"].into(),
                    },
                    metadata: HashMap::new(),
                },
                hashmap! {
                    DEVICE_LIST_STRATEGY_SETTING.into() => "volume-mounts".into(),
                },
            ),
            (
                MigrationData {
                    data: hashmap! {
                        DEVICE_LIST_STRATEGY_SETTING.into() => vec!["envvar", "volume-mounts"].into(),
                    },
                    metadata: HashMap::new(),
                },
                hashmap! {
                    DEVICE_LIST_STRATEGY_SETTING.into() => "envvar".into(),
                },
            ),
            (
                MigrationData {
                    data: hashmap! {
                        DEVICE_LIST_STRATEGY_SETTING.into() => vec!["cdi-cri", "envvar"].into(),
                    },
                    metadata: HashMap::new(),
                },
                hashmap! {
                    DEVICE_LIST_STRATEGY_SETTING.into() => "volume-mounts".into(),
                },
            ),
            (
                MigrationData {
                    data: hashmap! {
                        DEVICE_LIST_STRATEGY_SETTING.into() => vec!["cdi-cri", "envvar", "volume-mounts"].into(),
                    },
                    metadata: HashMap::new(),
                },
                hashmap! {
                    DEVICE_LIST_STRATEGY_SETTING.into() => "volume-mounts".into(),
                },
            ),
            (
                MigrationData {
                    data: hashmap! {
                        DEVICE_LIST_STRATEGY_SETTING.into() => vec!["envvar", "volume-mounts"].into(),
                    },
                    metadata: HashMap::new(),
                },
                hashmap! {
                    DEVICE_LIST_STRATEGY_SETTING.into() => "envvar".into(),
                },
            ),
            (
                MigrationData {
                    data: hashmap! {
                        DEVICE_LIST_STRATEGY_SETTING.into() => vec!["volume-mounts", "envvar"].into(),
                    },
                    metadata: HashMap::new(),
                },
                hashmap! {
                    DEVICE_LIST_STRATEGY_SETTING.into() => "volume-mounts".into(),
                },
            ),
            (
                MigrationData {
                    data: hashmap! {
                        DEVICE_LIST_STRATEGY_SETTING.into() => "cdi-cri".into(),
                    },
                    metadata: HashMap::new(),
                },
                hashmap! {
                    DEVICE_LIST_STRATEGY_SETTING.into() => "volume-mounts".into(),
                },
            ),
            (
                MigrationData {
                    data: hashmap! {
                        DEVICE_LIST_STRATEGY_SETTING.into() => "envvar".into(),
                    },
                    metadata: HashMap::new(),
                },
                hashmap! {
                    DEVICE_LIST_STRATEGY_SETTING.into() => "envvar".into(),
                },
            ),
        ];

        for (input, expected) in test_cases {
            let result = ReplaceDeviceListStrategy.backward(input).unwrap();
            assert_eq!(result.data, expected);
        }
    }
}

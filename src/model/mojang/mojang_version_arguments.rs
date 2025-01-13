use serde::{Deserialize, Serialize};

use crate::platform::PlatformData;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionArguments {
    pub game: Vec<MojangVersionArgumentEntry>,
    pub jvm: Vec<MojangVersionArgumentEntry>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum MojangVersionArgumentEntry {
    String(String),
    RuleBasedArgument(MojangVersionArgumentRuleBasedValue),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionArgumentRuleBasedValue {
    pub value: Option<MojangVersionArgumentRuleValue>,
    pub rules: Vec<MojangVersionArgumentRule>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionArgumentRule {
    pub action: String,
    pub os: Option<MojangVersionArgumentRuleOs>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum MojangVersionArgumentRuleValue {
    String(String),
    Array(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionArgumentRuleOs {
    pub name: Option<String>,
    pub arch: Option<String>,
}

impl MojangVersionArguments {
    pub fn select_arguments(&self, platform_data: &PlatformData) -> (Vec<String>, Vec<String>) {
        let mut game_args = Vec::new();
        let mut jvm_args = Vec::new();

        for arg in &self.game {
            match arg {
                MojangVersionArgumentEntry::String(value) => game_args.push(value.clone()),
                _ => {}
            }
        }

        for arg in &self.jvm {
            match arg {
                MojangVersionArgumentEntry::String(value) => jvm_args.push(value.clone()),
                MojangVersionArgumentEntry::RuleBasedArgument(rule_based_value) => {
                    for rule in &rule_based_value.rules {
                        if let Some(os) = &rule.os {
                            if let Some(name) = &os.name {
                                if name != &platform_data.platform_type.native_id() {
                                    continue;
                                }
                            }
                        }

                        match &rule_based_value.value {
                            Some(MojangVersionArgumentRuleValue::String(value)) => {
                                jvm_args.push(value.clone());
                            }
                            Some(MojangVersionArgumentRuleValue::Array(values)) => {
                                jvm_args.extend(values.clone());
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        (game_args, jvm_args)
    }
}

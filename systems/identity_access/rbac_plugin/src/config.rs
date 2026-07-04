use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacConfig {
    pub model_path: String,
    pub policy_path: String,
}

impl Default for RbacConfig {
    fn default() -> Self {
        Self {
            model_path: "rbac_model.conf".to_owned(),
            policy_path: "rbac_policy.csv".to_owned(),
        }
    }
}

use serde::{Deserialize, Serialize};

pub mod entrypoint;
pub mod processor;

#[derive(Serialize, Deserialize, Debug)]
pub struct TriggerProof {
    pub bump: u8,
    pub execution_id: String,
}

pub const BONSOL_IMAGE_ID: &'static str = "598a430e71a8b4a8f6aecc944e484cb555522f2dc66375b1cbe9ea88790e6b9c";

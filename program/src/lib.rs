use serde::{Deserialize, Serialize};

pub mod entrypoint;
pub mod processor;

#[derive(Serialize, Deserialize, Debug)]
pub struct TriggerProof {
    pub bump: u8,
    pub execution_id: String,
}

pub const BONSOL_IMAGE_ID: &'static str = "d56c18db0f7e17ba3dbb7336ee4a0bebd5e4216d9b9d30133322175cf6c4ac79";

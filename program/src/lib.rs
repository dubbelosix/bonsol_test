use serde::{Deserialize, Serialize};

pub mod entrypoint;
pub mod processor;

#[derive(Serialize, Deserialize, Debug)]
pub struct TriggerProof {
    pub bump: u8,
    pub execution_id: String,
}

pub const BONSOL_IMAGE_ID: &'static str = "a50a57236235f45a610d47417c3489ab097909986a625f74a5c3a9ea4fa01a53";

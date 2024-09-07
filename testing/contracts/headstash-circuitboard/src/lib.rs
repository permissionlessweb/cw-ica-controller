/// Secret Headstash Implementation Workflow:
/// 1. Upload Headstash, Snip120u, Secret-Cw-ICA-Controller, and Secret-Headstash-Cw-ICA-Owner WASM blobs.
/// 2. Instantiate Headstash (each token sent will create a snip120u for use)
/// 3. Set Headstash As Minter.
/// 4. Instantate SCRT-Headstash-Circuit-board.
/// 5. Create ICA owned by protocol
/// 6. 
pub mod contract;
pub mod cosmos_msg;
mod error;
pub mod msg;
pub mod state;
pub mod verify;

pub use crate::error::ContractError;

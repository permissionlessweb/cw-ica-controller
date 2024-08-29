use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_ica_controller::types::state::headstash::HeadstashParams;
use cw_storage_plus::{Item, Map};

pub use contract::ContractState;
pub use ica::{IcaContractState, IcaState};

/// The item used to store the state of the IBC application.
pub const STATE: Item<ContractState> = Item::new("state");
/// The map used to store the state of the cw-ica-controller contracts.
pub const ICA_STATES: Map<u64, IcaContractState> = Map::new("icas");
/// The map used to store the state of the cw-ica-controller contracts.
pub const HEADSTASH_STATES: Map<u64, HeadstashParams> = Map::new("hsp");
/// The item used to store the count of the cw-ica-controller contracts.
pub const ICA_COUNT: Item<u64> = Item::new("ica");
/// The item used to map contract addresses to ICA IDs.
pub const CONTRACT_ADDR_TO_ICA_ID: Map<Addr, u64> = Map::new("catia");
/// The item used to store the stat of the snip25 contracts created
pub const SNIP25_CONTRACTS: Item<Vec<Addr>> = Item::new("snip");

mod contract {

    use super::*;

    /// ContractState is the state of the IBC application.
    #[cw_serde]
    pub struct ContractState {
        /// The code ID of the cw-ica-controller contract.
        pub ica_controller_code_id: u64,
    }

    impl ContractState {
        /// Creates a new ContractState.
        pub fn new(ica_controller_code_id: u64) -> Self {
            Self {
                ica_controller_code_id,
            }
        }
    }
}

mod ica {
    use cw_ica_controller::{ibc::types::metadata::TxEncoding, types::state::ChannelState};

    use super::*;

    /// IcaContractState is the state of the cw-ica-controller contract.
    #[cw_serde]
    pub struct IcaContractState {
        pub contract_addr: Addr,
        pub ica_state: Option<IcaState>,
        pub headstash_params: HeadstashParams,
    }

    /// IcaState is the state of the ICA.
    #[cw_serde]
    pub struct IcaState {
        pub ica_id: u64,
        pub ica_addr: String,
        pub tx_encoding: TxEncoding,
        pub channel_state: ChannelState,
    }

    impl IcaContractState {
        /// Creates a new [`IcaContractState`].
        pub fn new(contract_addr: Addr, headstash_params: HeadstashParams) -> Self {
            Self {
                contract_addr,
                ica_state: None,
                headstash_params,
            }
        }
    }

    impl IcaState {
        /// Creates a new [`IcaState`].
        pub fn new(
            ica_id: u64,
            ica_addr: String,
            tx_encoding: TxEncoding,
            channel_state: ChannelState,
        ) -> Self {
            Self {
                ica_id,
                ica_addr,
                tx_encoding,
                channel_state,
            }
        }
    }
}

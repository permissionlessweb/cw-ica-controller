use cosmwasm_schema::cw_serde;
use cw_storage_plus::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};
use secret_cosmwasm_std::{Addr, DepsMut, Storage, Uint128};

pub use contract::ContractState;
pub use ica::{IcaContractState, IcaState};

pub const INIT_ID: u64 = 71;

/// The item used to store the state of the IBC application.
pub static CONFIG_KEY: &[u8] = b"state";
/// The map used to store the state of the cw-ica-controller contracts.
pub const ICA_STATES: &[u8] = b"ica_states";
/// The item used to store the count of the cw-ica-controller contracts.
pub const ICA_COUNT: &[u8] = b"ica_count";
/// The item used to map contract addresses to ICA IDs.
pub const CONTRACT_ADDR_TO_ICA_ID: &[u8] = b"contract_addr_to_ica_id";

pub fn config_w(storage: &mut dyn Storage) -> Singleton<ContractState> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_r(storage: &dyn Storage) -> ReadonlySingleton<ContractState> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn ica_states_r(storage: &dyn Storage) -> ReadonlyBucket<IcaContractState> {
    bucket_read(storage, ICA_STATES)
}

pub fn ica_states_w(storage: &mut dyn Storage) -> Bucket<IcaContractState> {
    bucket(storage, ICA_STATES)
}

pub fn ica_count_w(storage: &mut dyn Storage) -> Singleton<u64> {
    singleton(storage, CONFIG_KEY)
}

pub fn ica_count_r(storage: &dyn Storage) -> ReadonlySingleton<u64> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn addr_to_ica_id_r(storage: &dyn Storage) -> ReadonlyBucket<u64> {
    bucket_read(storage, ICA_STATES)
}

pub fn addr_to_ica_ir_w(storage: &mut dyn Storage) -> Bucket<u64> {
    bucket(storage, ICA_STATES)
}

mod contract {

    use crate::ContractError;

    use super::*;

    /// ContractState is the state of the IBC application.
    #[cw_serde]
    pub struct ContractState {
        /// The admin of this contract.
        pub admin: Addr,
        /// The code ID of the cw-ica-controller contract.
        pub ica_controller_code_id: u64,
        /// The code ID of the cw-ica-controller contract.
        pub ica_controller_code_hash: String,
        /// {wallet1} {wallet2}
        pub bloom_plaintxt_msg: String,
        /// contract addr of headstash.
        pub headstash_addr: String,
    }

    impl ContractState {
        /// Creates a new ContractState.
        pub fn new(
            admin: Addr,
            ica_controller_code_id: u64,
            ica_controller_code_hash: String,
            bloom_plaintxt_msg: String,
            headstash_addr: String,
        ) -> Self {
            Self {
                admin,
                ica_controller_code_id,
                ica_controller_code_hash,
                bloom_plaintxt_msg,
                headstash_addr,
            }
        }

        /// Checks if the address is the admin
        pub fn verify_admin(&self, sender: impl Into<String>) -> Result<(), ContractError> {
            if self.admin == sender.into() {
                Ok(())
            } else {
                Err(ContractError::Unauthorized {})
            }
        }
        pub fn verify_headstash_bloom(
            &self,
            deps: &DepsMut,
            sender: Addr,
            eth_pubkey: String,
            eth_sig: String,
            bloom_plaintxt: String,
            destination_addr: String,
        ) -> Result<Addr, ContractError> {
            // verify signature comes from eth pubkey
            crate::contract::validation::validate_claim(
                deps,
                sender,
                eth_pubkey.into(),
                eth_sig.into(),
                bloom_plaintxt.into(),
                &destination_addr,
            )?;

            let owner = Addr::unchecked("TODO: return owner addr from eth pubkey");

            Ok(owner)
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
        pub fn new(contract_addr: Addr) -> Self {
            Self {
                contract_addr,
                ica_state: None,
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

#[cw_serde]
pub struct BloomSnip120u {
    pub amount: Uint128,
    pub address: Addr,
}

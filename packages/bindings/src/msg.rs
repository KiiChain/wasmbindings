use crate::types::Metadata;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg, Uint128};

/// KiichainMsg represents a message to the Kiichain wasm implementation
#[cw_serde]
pub enum KiichainMsg {
    TokenFactory(TokenFactoryMsg),
    // Future implementation will go here
    // EVM(EVMMsg),
}

/// Special messages to be supported by any chain that supports token_factory
#[cw_serde]
pub enum TokenFactoryMsg {
    /// CreateDenom creates a new factory denom, of denomination:
    /// factory/{creating contract bech32 address}/{Subdenom}
    /// Subdenom can be of length at most 44 characters, in [0-9a-zA-Z./]
    /// Empty subdenoms are valid.
    /// The (creating contract address, subdenom) pair must be unique.
    /// The created denom's admin is the creating contract address,
    /// but this admin can be changed using the UpdateAdmin binding.
    ///
    /// If you set an initial metadata here, this is equivalent
    /// to calling SetMetadata directly on the returned denom.
    CreateDenom {
        subdenom: String,
        metadata: Option<Metadata>,
    },
    /// ChangeAdmin changes the admin for a factory denom.
    /// Can only be called by the current contract admin.
    /// If the NewAdminAddress is empty, the denom will have no admin.
    ChangeAdmin {
        denom: String,
        new_admin_address: String,
    },
    /// Contracts can mint native tokens for an existing factory denom
    /// that they are the admin of.
    MintTokens {
        denom: String,
        amount: Uint128,
        mint_to_address: String,
    },
    /// Contracts can burn native tokens for an existing factory denom
    /// tshat they are the admin of.
    BurnTokens {
        denom: String,
        amount: Uint128,
        burn_from_address: String,
    },
    /// Contracts can force transfer tokens for an existing factory denom
    /// that they are the admin of.
    ForceTransfer {
        denom: String,
        amount: Uint128,
        from_address: String,
        to_address: String,
    },
    SetMetadata {
        denom: String,
        metadata: Metadata,
    },
}

impl TokenFactoryMsg {
    /// Turn the TokenFactoryMsg into a KiichainMsg
    pub fn into_kiichain_msg(self) -> KiichainMsg {
        KiichainMsg::TokenFactory(self)
    }
}

impl From<TokenFactoryMsg> for CosmosMsg<TokenFactoryMsg> {
    fn from(msg: TokenFactoryMsg) -> CosmosMsg<TokenFactoryMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for KiichainMsg {}

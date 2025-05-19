use crate::types::{Metadata, Params};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

/// KiichainQuery represents a query to the Kiichain wasm implementation
#[cw_serde]
pub enum KiichainQuery {
    TokenFactory(TokenFactoryQuery),
    Evm(EVMQuery),
}

/// TokenFactoryQuery represents a query to the TokenFactory module
#[cw_serde]
#[derive(QueryResponses)]
pub enum TokenFactoryQuery {
    /// Given a subdenom created by the address `creator_addr`
    /// returns the full denom as used by `BankMsg::Send`.
    /// You may call `FullDenom { creator_addr: env.contract.address, subdenom }` to find the denom issued
    /// by the current contract.
    #[returns(FullDenomResponse)]
    FullDenom {
        creator_addr: String,
        subdenom: String,
    },
    /// Returns the metadata set for this denom, if present. May return None.
    /// This will also return metadata for native tokens created outside
    /// of the token factory (like staking tokens)
    #[returns(MetadataResponse)]
    Metadata { denom: String },
    /// Returns info on admin of the denom, only if created/managed via token factory.
    /// Errors if denom doesn't exist or was created by another module.
    #[returns(AdminResponse)]
    Admin { denom: String },
    /// List all denoms that were created by the given creator.
    /// This does not imply all tokens currently managed by the creator.
    /// (Admin may have changed)
    #[returns(DenomsByCreatorResponse)]
    DenomsByCreator { creator: String },
    /// Returns configuration params for TokenFactory modules
    #[returns(ParamsResponse)]
    Params {},
}

/// TokenFactoryQuery implementations
impl TokenFactoryQuery {
    /// Turn the TokenFactoryQuery into a KiichainQuery
    pub fn into_kiichain_query(self) -> KiichainQuery {
        KiichainQuery::TokenFactory(self)
    }
}

/// Implement the custom query trait for KiichainQuery
impl CustomQuery for KiichainQuery {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum EVMQuery {
    /// Executes a query on the EVM module (eth_call)
    #[returns(EVMEthCallResponse)]
    EthCall {
        /// The address of the contract to call
        contract: String,
        /// The data to send to the contract
        data: String,
    },
    /// Fetch the ERC20 information for a given address
    #[returns(Erc20InformationResponse)]
    Erc20Information {
        /// The address of the contract to call
        contract: String,
    },
    /// Fetch the ERC20 balance for a given address
    #[returns(Erc20BalanceResponse)]
    Erc20Balance {
        /// The address of the contract to call
        contract: String,
        /// The address to check the balance for
        address: String,
    },
    /// Fetch the ERC20 allowance for a given address
    #[returns(Erc20AllowanceResponse)]
    Erc20Allowance {
        /// The address of the contract to call
        contract: String,
        /// The address to check the allowance for
        owner: String,
        /// The address to check the allowance for
        spender: String,
    },
}

/// EVMQuery implementations
impl EVMQuery {
    /// Turn the EVMQuery into a KiichainQuery
    pub fn into_kiichain_query(self) -> KiichainQuery {
        KiichainQuery::Evm(self)
    }
}

/// Implement the custom query trait for EVMQuery
impl CustomQuery for EVMQuery {}

/// FullDenomResponse is the full denom query response
#[cw_serde]
pub struct FullDenomResponse {
    pub denom: String,
}

/// MetadataResponse is the metadata query response
#[cw_serde]
pub struct MetadataResponse {
    /// Empty if this was never set for the given denom
    pub metadata: Option<Metadata>,
}

/// AdminResponse is the admin query response
#[cw_serde]
pub struct AdminResponse {
    pub admin: String,
}

/// DenomsByCreatorResponse is the denoms by creator query response
#[cw_serde]
pub struct DenomsByCreatorResponse {
    pub denoms: Vec<String>,
}

/// ParamsResponse is the params query response
#[cw_serde]
pub struct ParamsResponse {
    pub params: Params,
}

#[cw_serde]
pub struct EVMEthCallResponse {
    /// The result of the call in hex format
    pub data: String,
}

#[cw_serde]
pub struct Erc20InformationResponse {
    /// The name of the token
    pub name: String,
    /// The symbol of the token
    pub symbol: String,
    /// The number of decimals of the token
    pub decimals: u8,
}

#[cw_serde]
pub struct Erc20BalanceResponse {
    /// The balance of the token
    pub balance: String,
}

#[cw_serde]
pub struct Erc20AllowanceResponse {
    /// The allowance of the token
    pub allowance: String,
}

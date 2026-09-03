use soroban_sdk::{contractevent, Address, Symbol};

#[contractevent]
pub struct InitializedEvent {
    #[topic]
    pub version: Symbol,
    pub event_version: u32,
    pub admin: Address,
}

#[contractevent]
pub struct PriceUpdatedEvent {
    #[topic]
    pub version: Symbol,
    #[topic]
    pub asset: Address,
    pub event_version: u32,
    pub admin: Address,
    pub price: i128,
}

#[allow(dead_code)]
#[contractevent]
pub struct OracleUpdatedEvent {
    #[topic]
    pub version: Symbol,
    #[topic]
    pub asset: Address,
    pub event_version: u32,
    pub admin: Address,
    pub oracle: Address,
}

#[contractevent]
pub struct PriceInvalidatedEvent {
    #[topic]
    pub version: Symbol,
    #[topic]
    pub asset: Address,
    pub event_version: u32,
    pub admin: Address,
}

#[contractevent]
pub struct StalenessWindowUpdatedEvent {
    #[topic]
    pub version: Symbol,
    #[topic]
    pub admin: Address,
    pub event_version: u32,
    pub max_age_seconds: u64,
}

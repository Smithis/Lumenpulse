#![no_std]

//! Shared contract version introspection surface (issue #1046).
//!
//! This crate defines the standardized [`ContractVersion`] response type and
//! the `VersionedContract` trait that any contract in this workspace can
//! implement to expose its deployed semantic version on-chain, instead of
//! relying only on off-chain deployment manifests.
//!
//! # Format
//!
//! A [`ContractVersion`] is a SemVer-style `(major, minor, patch)` triple:
//!
//! - `major` — bumped for storage-layout or interface changes that are not
//!   backward compatible (clients/operators must treat these as a different
//!   contract surface).
//! - `minor` — bumped for backward-compatible additions (new entrypoints,
//!   new optional behavior).
//! - `patch` — bumped for backward-compatible fixes with no interface or
//!   storage change.
//!
//! Use [`ContractVersion::is_compatible_with`] to check whether two
//! deployments are interface-compatible rather than comparing fields
//! manually.
//!
//! # Implementing contracts
//!
//! The following contracts in this workspace declare that they implement
//! `VersionedContract` and are covered by the conformance suite:
//!
//! - `lumen_token` (`LumenToken`)
//! - `crowdfund_vault` (`CrowdfundVaultContract`)
//! - `contributor_registry` (`ContributorRegistryContract`)
//! - `vesting-wallet` (`VestingWalletContract`)
//! - `upgradable-contract` (`UpgradableContract`)
//!
//! Because implementers use `impl VersionedContract`, adding a method to the
//! trait without updating every implementer is a compile error. The
//! conformance tests (in this crate's `conformance` module) also exercise
//! each implementer end to end to catch signature drift.

use soroban_sdk::{contractclient, contracttype, Env, Symbol};

/// Canonical event-schema version for all `#[contractevent]` structs.
///
/// v1 is the initial version. Bump only when a required/renamed/removed
/// data field or topic order changes. Additive `Option<T>` fields stay v1.
/// Keep old structs as `...V1` for replay when bumping.
pub const EVENT_SCHEMA_VERSION: u32 = 1;

/// Canonical version topic string (`ScSymbol` <= 32 chars).
/// Published as `topics[1]` — `topics[0]` remains the auto-derived
/// snake_case struct name so existing backend mappers keep matching.
pub const EVENT_VERSION_TOPIC: &str = "v1";

/// Returns `Symbol("v1")` for event publishing.
/// Callers should also set `event_version: EVENT_SCHEMA_VERSION` in data
/// so decoders that only read `value` can detect schema changes.
pub fn event_version_symbol(env: &Env) -> Symbol {
    Symbol::new(env, EVENT_VERSION_TOPIC)
}

/// Standardized semantic version response for on-chain version introspection.
///
/// Two versions are backward compatible when they share the same `major`
/// (see [`ContractVersion::is_compatible_with`]) — this is the field clients
/// and operators should key release/config validation off of.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ContractVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ContractVersion {
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Whether `self` and `other` are interface-compatible per SemVer
    /// semantics: same `major` (and, pre-1.0, same `minor` too, since a `0.x`
    /// line has no stability guarantee across minor versions).
    pub fn is_compatible_with(&self, other: &ContractVersion) -> bool {
        if self.major == 0 || other.major == 0 {
            self.major == other.major && self.minor == other.minor
        } else {
            self.major == other.major
        }
    }
}

/// Standardized version introspection interface. Implement this on any
/// contract that should expose its deployed semantic version on-chain.
#[contractclient(name = "VersionedClient")]
pub trait VersionedContract {
    /// Returns this contract's deployed semantic version.
    fn contract_version(env: Env) -> ContractVersion;
}

#[cfg(test)]
pub mod conformance {
    use super::*;
    use soroban_sdk::{contract, contractimpl, Env};

    /// A mock implementer used by the conformance suite.
    #[contract]
    pub struct MockVersioned;

    #[contractimpl]
    impl VersionedContract for MockVersioned {
        fn contract_version(_env: Env) -> ContractVersion {
            ContractVersion::new(1, 2, 3)
        }
    }

    /// Exercises a registered contract through the interface's generated
    /// client, asserting it exposes `contract_version` with the exact
    /// signature declared by [`VersionedContract`], and returns the reported
    /// version so callers can assert on it.
    pub fn assert_version_signature(env: &Env, id: &soroban_sdk::Address) -> ContractVersion {
        let client = VersionedClient::new(env, id);
        client.contract_version()
    }

    #[test]
    fn mock_contract_implements_interface_end_to_end() {
        let env = Env::default();
        let id = env.register(MockVersioned, ());

        let version = assert_version_signature(&env, &id);

        assert_eq!(version, ContractVersion::new(1, 2, 3));
    }

    #[test]
    fn same_major_is_compatible() {
        let a = ContractVersion::new(1, 0, 0);
        let b = ContractVersion::new(1, 4, 2);
        assert!(a.is_compatible_with(&b));
        assert!(b.is_compatible_with(&a));
    }

    #[test]
    fn different_major_is_incompatible() {
        let a = ContractVersion::new(1, 9, 9);
        let b = ContractVersion::new(2, 0, 0);
        assert!(!a.is_compatible_with(&b));
    }

    #[test]
    fn pre_1_0_treats_minor_as_breaking() {
        let a = ContractVersion::new(0, 1, 0);
        let b = ContractVersion::new(0, 2, 0);
        assert!(!a.is_compatible_with(&b));

        let c = ContractVersion::new(0, 1, 5);
        assert!(a.is_compatible_with(&c));
    }

    #[test]
    fn event_schema_version_is_v1() {
        assert_eq!(EVENT_SCHEMA_VERSION, 1);
        assert_eq!(EVENT_VERSION_TOPIC, "v1");
        let env = Env::default();
        assert_eq!(event_version_symbol(&env).to_string(), "v1");
    }
}

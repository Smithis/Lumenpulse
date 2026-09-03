use crate::types::ProjectMetadata;
use soroban_sdk::{contractevent, Address, Env, String, Symbol};

// ── Event Struct Definitions ────────────────────────────────────────────────
// Canonical versioning (issue #1057): `version` topic + `event_version` data
// field on every event. topics[0] stays the auto-derived snake_case name so
// backend `RAW_EVENT_MAP` keeps matching; topics[1] == "v1" is filterable.

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectProposedEvent {
    #[topic]
    pub version: Symbol,
    pub event_version: u32,
    pub project_id: u64,
    pub proposer: Address,
    pub name: String,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoteCastEvent {
    #[topic]
    pub version: Symbol,
    pub event_version: u32,
    pub project_id: u64,
    pub voter: Address,
    pub approve: bool,
    pub voting_power: u64,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectVerifiedEvent {
    #[topic]
    pub version: Symbol,
    pub event_version: u32,
    pub project_id: u64,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectRejectedEvent {
    #[topic]
    pub version: Symbol,
    pub event_version: u32,
    pub project_id: u64,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalExpiredEvent {
    #[topic]
    pub version: Symbol,
    pub event_version: u32,
    pub project_id: u64,
}

// ── Direct Emission Helper Functions ─────────────────────────────────────────

pub fn emit_project_proposed(
    env: &Env,
    project_id: u64,
    proposer: &Address,
    metadata: &ProjectMetadata,
) {
    // Carry the name as-is (issue #1231): the previous implementation copied
    // it into a fixed 32-byte buffer and converted it to a `Symbol`, which
    // panicked for any name that wasn't exactly 32 bytes of
    // `[A-Za-z0-9_]` — i.e. almost every realistic project name, since
    // `ProjectMetadata::name` allows up to 100 arbitrary characters
    // (spaces, punctuation). `propose_project`, this contract's primary
    // entrypoint, could not complete for normal input. `String` has no such
    // restriction and needs no lossy round-trip.
    ProjectProposedEvent {
        version: version_interface::event_version_symbol(env),
        event_version: version_interface::EVENT_SCHEMA_VERSION,
        project_id,
        proposer: proposer.clone(),
        name: metadata.name.clone(),
    }
    .publish(env);
}

pub fn emit_vote_cast(
    env: &Env,
    project_id: u64,
    voter: &Address,
    approve: bool,
    voting_power: u64,
) {
    VoteCastEvent {
        version: version_interface::event_version_symbol(env),
        event_version: version_interface::EVENT_SCHEMA_VERSION,
        project_id,
        voter: voter.clone(),
        approve,
        voting_power,
    }
    .publish(env);
}

pub fn emit_project_verified(env: &Env, project_id: u64) {
    ProjectVerifiedEvent {
        version: version_interface::event_version_symbol(env),
        event_version: version_interface::EVENT_SCHEMA_VERSION,
        project_id,
    }
    .publish(env);
}

pub fn emit_project_rejected(env: &Env, project_id: u64) {
    ProjectRejectedEvent {
        version: version_interface::event_version_symbol(env),
        event_version: version_interface::EVENT_SCHEMA_VERSION,
        project_id,
    }
    .publish(env);
}

pub fn emit_proposal_expired(env: &Env, project_id: u64) {
    ProposalExpiredEvent {
        version: version_interface::event_version_symbol(env),
        event_version: version_interface::EVENT_SCHEMA_VERSION,
        project_id,
    }
    .publish(env);
}

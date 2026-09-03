# Canonical Event Versioning (issue #1057)

Goal: backend and data-processing consumers evolve safely.

## Rule

- Every `#[contractevent]` includes:
  - `#[topic] version: Symbol` — always `Symbol("v1")` today, published as `topics[1]`
  - `event_version: u32` — always `1` today, in data for XDR-only decoders
- `topics[0]` stays the auto-derived snake_case struct name (e.g. `price_updated_event`).
  Never rename the struct to version; backend `RAW_EVENT_MAP` keys on `topics[0]`.
- `v1` = current. Bump to `v2` on required/renamed/removed field or topic reorder.
  Keep old struct as `...V1` for replay. Additive `Option<T>` stays `v1` (minor).

## Producer

```rust
use version_interface::{EVENT_SCHEMA_VERSION, event_version_symbol};

#[contractevent]
pub struct PriceUpdatedEvent {
    #[topic] pub version: Symbol,
    #[topic] pub asset: Address,
    pub event_version: u32,
    pub admin: Address,
    pub price: i128,
}

PriceUpdatedEvent {
    version: event_version_symbol(&env),
    event_version: EVENT_SCHEMA_VERSION,
    .. 
}.publish(&env);
```

Shared source: `apps/onchain/contracts/version-interface/src/lib.rs`
(`EVENT_SCHEMA_VERSION`, `EVENT_VERSION_TOPIC="v1"`).

## Consumer (backend)

- Indexer keeps `topics[0]` mapping — v1 events map exactly as before.
- `extractEventSchemaVersion(topics)` parses `topics[1]` `/^v(\d+)$/`.
- `mapSorobanEventWithVersion(topics)` returns `null` on unsupported version
  so processor routes to DLQ instead of silently mis-decoding.
- `value.event_version` is authoritative when topics are pruned.

## Adopted

- `pricing_adapter` (5 events) + `lumenpulse-curation` (5 events) — SDK 23 workspace.
- Follow-up: `notification_broker` (pins SDK 21.5.1, not in workspace members)
  needs SDK upgrade before it can depend on `version-interface`; vendored copy
  intentionally not added to avoid drift.

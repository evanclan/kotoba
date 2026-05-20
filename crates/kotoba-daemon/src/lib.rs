//! # kotoba-daemon
//!
//! Local HTTP / JSON-RPC API for the Kotoba engine.
//!
//! Status: **planned for v0.3.** This crate is scaffolding only — see
//! `docs/plugin-api.md` for the planned API surface and
//! `docs/roadmap.md#v03--make-it-everywhere` for the milestone.
//!
//! When implemented, this crate will:
//! - Bind to `127.0.0.1:6060` by default
//! - Serve the v1 REST + JSON-RPC API documented in `docs/plugin-api.md`
//! - Authenticate requests via a per-install token in `~/.config/kotoba/daemon.token`
//! - Expose the same engine that backs the CLI

#![warn(missing_docs)]

/// The semver version of `kotoba-daemon`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Stub entry point that returns an explanatory message until the daemon is implemented.
pub fn placeholder_message() -> &'static str {
    "kotoba-daemon is scaffolded but not yet implemented. \
     See docs/plugin-api.md and docs/roadmap.md."
}

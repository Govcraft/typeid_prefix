# ⚠️ DEPRECATED - This repository has been archived

> **This crate has been consolidated into the [mti workspace](https://github.com/GovCraft/mti).**
>
> All future development will occur in the unified workspace repository.

## Migration

The `typeid_prefix` crate is now maintained as part of the `mti` (Magic Type Id) workspace at:

**https://github.com/GovCraft/mti**

The crate continues to be published to crates.io from the new location. Existing dependencies on `typeid_prefix` will continue to work - no changes are required for consumers.

## Why the change?

All three TypeID-related crates (`mti`, `typeid_prefix`, and `typeid_suffix`) are now consolidated into a single Cargo workspace for:

- Unified development and testing
- Consistent versioning and releases
- Simplified maintenance
- Shared CI/CD infrastructure

## Links

- **New Repository**: https://github.com/GovCraft/mti
- **crates.io**: https://crates.io/crates/typeid_prefix (still available)
- **Documentation**: https://docs.rs/typeid_prefix

---

*Original README content preserved below for reference:*

---

# `TypeID` Prefix

[![Crates.io](https://img.shields.io/crates/v/typeid_prefix.svg)](https://crates.io/crates/typeid_prefix)
[![Documentation](https://docs.rs/typeid_prefix/badge.svg)](https://docs.rs/typeid_prefix)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A Rust library that implements robust validation and sanitization for the `prefix` component of [TypeIDs](https://github.com/jetpack-io/typeid), strictly adhering to the rules defined in the official **TypeID Specification**.

TypeIDs are globally unique, type-prefixed identifiers. This crate focuses *only* on the `prefix` part, ensuring it conforms to the specification's requirements for length, character set, and structure.

For a complete TypeID implementation, use the [mti (Magic Type Id) crate](https://crates.io/crates/mti).

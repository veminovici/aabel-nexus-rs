# Simplee > Aabel > Nexus

[![CI][ci-badge]][ci-url]
![GitHub top language][lang-badge]
[![License:MIT][license-badge]][license-url]
![GitHub code size in bytes][size-badge]
![GitHub last commit][last-commit-badge]
![GitHub watchers][watchers-badge]

A set of crates for distributed algorithms. They use an actor model.

## Crates
- The [nexus-ids][nexus-ids-folder] crate implements the actor and session identifiers.
- The [nexus-hashmap][nexus-hashmap-folder] crate implements several extensions for the HashMap.
- The [nexus-lattice][nexus-lattice-folder] crate defines *Join* and *Meet* traits, basic operations in the [lattice](https://en.wikipedia.org/wiki/Join_and_meet) theory.
- The [nexus-counters][nexus-counters-folder] crate implements a distributed counter. In our case it will be used to increment different values.
- The [nexus-session-store][nexus-session-store-folder] crate implements a store for session. Each session can store values for each node.
- The [nexus-message][nexus-message-folder] crate implements serveral extensions for the actix::Message trait.
- The [nexus-observer][nexus-observer-folder] crate implements the Observer actor. The actor counts the number of successful and failed operations during the execution of the run.

## About
> Code designed and written on the beautiful island of [**Saaremaa**][url_estonia], Estonia.

[crates-url]: https://crates.io/crates/aabel-nexus-rs
[ci-badge]: https://github.com/veminovici/aabel-nexus-rs/actions/workflows/ci.yml/badge.svg?branch=main
[ci-url]: https://github.com/veminovici/aabel-nexus-rs/actions/workflows/ci.yml
[lang-badge]: https://img.shields.io/github/languages/top/veminovici/aabel-nexus-rs
[license-badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[license-url]: https://opensource.org/licenses/MIT
[size-badge]: https://img.shields.io/github/languages/code-size/veminovici/aabel-nexus-rs
[last-commit-badge]: https://img.shields.io/github/last-commit/veminovici/aabel-nexus-rs
[watchers-badge]: https://img.shields.io/github/watchers/veminovici/aabel-nexus-rs
[url_estonia]: https://goo.gl/maps/DmB9ewY2R3sPGFnTA
[nexus-ids-folder]: ./nexus-ids/
[nexus-hashmap-folder]: ./nexus-hashmap/
[nexus-lattice-folder]: ./nexus-lattice/
[nexus-counters-folder]: ./nexus-counters/
[nexus-message-folder]: ./nexus-message/
[nexus-session-store-folder]: ./nexus-session-store/
[nexus-observer-folder]: ./nexus-observer/
[def]: ./nexus-hashmap/
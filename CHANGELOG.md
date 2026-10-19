# Changelog

## [0.2.0](https://github.com/tembo/tembo-rs/compare/v0.1.0...v0.2.0) (2026-10-19)


### Features

* **api:** add body field expiresAt on apiKeys.create (+7 more changes) ([40e9940](https://github.com/tembo/tembo-rs/commit/40e994010ca20fe0bb1b04b378c12c16bffab2db))
* **api:** add operation projects.duplicate (+4 more changes) ([df47091](https://github.com/tembo/tembo-rs/commit/df470918f14b1e279252d30485bbc9cd67bdd5ef))
* **api:** add query param range on insights.retrieve (+2 more changes) ([c2946e2](https://github.com/tembo/tembo-rs/commit/c2946e23b804f346ce25b63e1bdd0e027c7aa1c6))
* **api:** initial SDK generation ([41373fd](https://github.com/tembo/tembo-rs/commit/41373fd436c1ad6f070514ed245190107a07f1d5))
* **api:** update SDK surface (3 changes) ([64341c0](https://github.com/tembo/tembo-rs/commit/64341c0885049cfbc93f810a0ede9fb2d67ebffb))


### Bug Fixes

* **models:** allow module_inception for the models resource ([7effd3c](https://github.com/tembo/tembo-rs/commit/7effd3c3c5d483cf08c5b30ac23d8b85a56de3b6))
* **models:** allow module_inception for the models resource ([3866856](https://github.com/tembo/tembo-rs/commit/3866856a0008af0eed3c3f02377842d864f8a281))

## Changelog

All notable changes to `tembo-rs` are documented here. Release
tooling appends a section per released version below.

## Unreleased

- Initial generation of the `tembo-rs` SDK.
- Response-only models are marked `#[non_exhaustive]`, so new response
  fields can be added in future versions without a breaking release;
  request models stay literally constructible.

# Changelog

## [0.10.0](https://github.com/orcher-io/protos/compare/v0.9.0...v0.10.0) (2026-10-19)


### Features

* let a workflow failure carry the calls made before it ([#35](https://github.com/orcher-io/protos/issues/35)) ([6b42c29](https://github.com/orcher-io/protos/commit/6b42c29e1f209f0db88bb8b7d091dbc9dcf44d04))

## [0.9.0](https://github.com/orcher-io/protos/compare/v0.8.0...v0.9.0) (2026-10-15)


### Features

* carry a fresh run's retry policy in the kernel protocol ([#33](https://github.com/orcher-io/protos/issues/33)) ([ced87fd](https://github.com/orcher-io/protos/commit/ced87fd82ac93020e68021f31d1ed8012a043899))

## [0.8.0](https://github.com/orcher-io/protos/compare/v0.7.0...v0.8.0) (2026-10-15)


### ⚠ BREAKING CHANGES

* Resolution gains a field, so Rust code building it with a struct literal that names every field must add it.

### Features

* carry each resolution's place in the run's order ([#31](https://github.com/orcher-io/protos/issues/31)) ([0f23809](https://github.com/orcher-io/protos/commit/0f238092b172115fa1eb6b7e6d52352ffc580b73))

## [0.7.0](https://github.com/orcher-io/protos/compare/v0.6.0...v0.7.0) (2026-10-14)


### ⚠ BREAKING CHANGES

* RestartFresh gains a field, so Rust code building it with a struct literal that names every field must add it.

### Features

* carry a restart's execution timeout in the kernel protocol ([#27](https://github.com/orcher-io/protos/issues/27)) ([d041e25](https://github.com/orcher-io/protos/commit/d041e25153014f525488b9df711ebd5c7a3e5dc0))

## [0.6.0](https://github.com/orcher-io/protos/compare/v0.5.0...v0.6.0) (2026-10-13)


### ⚠ BREAKING CHANGES

* Resolution.what gains a variant, so a Rust match over it that lists every variant must handle ChildStarted.

### Features

* add a ChildStarted resolution to the kernel protocol ([#25](https://github.com/orcher-io/protos/issues/25)) ([b9a7c7f](https://github.com/orcher-io/protos/commit/b9a7c7f2b177759e5f82d073692fa2ec45affcb2))

## [0.5.0](https://github.com/orcher-io/protos/compare/v0.4.0...v0.5.0) (2026-10-13)


### Features

* carry a legacy run's step id on each command ([#21](https://github.com/orcher-io/protos/issues/21)) ([6a0744f](https://github.com/orcher-io/protos/commit/6a0744f6349ce75b0ae7695d98950188c72d54d0))

## [0.4.0](https://github.com/orcher-io/protos/compare/v0.3.0...v0.4.0) (2026-10-12)


### Features

* add the host protocol messages (orcher.host.v1) ([#18](https://github.com/orcher-io/protos/issues/18)) ([cf9c85c](https://github.com/orcher-io/protos/commit/cf9c85c012d6b358f44d0f6bd499e876eee998d1))
* list supported step-id schemes and carry the history lineage ([#19](https://github.com/orcher-io/protos/issues/19)) ([95ab651](https://github.com/orcher-io/protos/commit/95ab651c40d6df4dfb6b434c4ddd6b3962583b33)), closes [#17](https://github.com/orcher-io/protos/issues/17)

## [0.3.0](https://github.com/orcher-io/protos/compare/v0.2.0...v0.3.0) (2026-10-11)


### Features

* add the kernel protocol messages (orcher.kernel.v1) ([#15](https://github.com/orcher-io/protos/issues/15)) ([e789fa0](https://github.com/orcher-io/protos/commit/e789fa0adf8e5354d0842055165a4e691e0f3a2d)), closes [#14](https://github.com/orcher-io/protos/issues/14)

## [0.2.0](https://github.com/orcher-io/protos/compare/v0.1.3...v0.2.0) (2026-10-08)


### Bug Fixes

* release added fields as a minor version, starting with 0.2.0 ([#10](https://github.com/orcher-io/protos/issues/10)) ([9e04b4f](https://github.com/orcher-io/protos/commit/9e04b4f0682e9b57f2af743c648a26c10ea8e5fe))

## [0.1.3](https://github.com/orcher-io/protos/compare/v0.1.2...v0.1.3) (2026-10-08)


### Features

* report the worker protocol version on workflow completion and failure ([#8](https://github.com/orcher-io/protos/issues/8)) ([6064cb5](https://github.com/orcher-io/protos/commit/6064cb532074fb04af999f3d3c5ec72633cf9887))

## [0.1.2](https://github.com/orcher-io/protos/compare/v0.1.1...v0.1.2) (2026-10-08)


### Features

* add a cleanup limit to cancellation and a worker protocol version ([#6](https://github.com/orcher-io/protos/issues/6)) ([83feb6b](https://github.com/orcher-io/protos/commit/83feb6bf307c9e1c16563db3a2670e52d9c2bd55))

## [0.1.1](https://github.com/orcher-io/protos/compare/v0.1.0...v0.1.1) (2026-10-04)


### Bug Fixes

* show the README banner on the package registries ([#3](https://github.com/orcher-io/protos/issues/3)) ([4df4525](https://github.com/orcher-io/protos/commit/4df4525e03d2904aaa5791ceecf051a8da9d9a58))

## Changelog

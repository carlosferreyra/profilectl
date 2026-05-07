# Changelog

All notable changes to dfiles are documented here.

## [0.1.2] — 2026-04-24

### Bug Fixes

- Add system Python flag and Git detection for Windows ([`b39957a`](b39957abb2612e07b5233172e8caead67d01c706))

- Improve repo root detection for cached installations ([`b39f3f0`](b39f3f00787c5a97fa066848be15ecb74d009695))

- Resolve all CI failures ([`de5c6ec`](de5c6ecca5715d05b4d0fa062ffa5741b3759899))

- Add version to internal workspace dep entries for crates.io publishing ([`bf85e82`](bf85e82e128037ab72e4674acfcb51f1a7384c18))

- Publish all workspace crates, align with uv release model ([`ebbddf4`](ebbddf465f1cc4cc41997b0022655ff45565f3e6))

- Correct release.toml for cargo-release workspace behavior ([`6753aa3`](6753aa397032bc7b9365578a35ecf11ca2690af4))

- Remove no-verify from release.toml (unsupported in cargo-release 1.1.2) ([`5e9a920`](5e9a9209315ac525205690b1501439e0d90347eb))

- Restore release.toml hook, add initial CHANGELOG.md ([`9066ef2`](9066ef2483785f7bc6eaa033963bc2401a0a13d8))

- Suppress git-cliff hook on library crates, run only from root ([`3b0be34`](3b0be3474bddba7868b5c8cffd9ae29b750a4edc))


### Chores

- Rename package to 'dfiles' and add PyPI workflow ([`c713f95`](c713f95227336e57ae558fc266140823716bd2d0))

- Drop publish-crates.yml and fix stale dfiles refs ([`7640638`](764063895ffeb1359a6724b813a99ada07b5ffde))

- Release v0.1.1 ([`8329914`](83299145376327d3875ff71c9935073bc1829005))


### Documentation

- Update README with bootstrap and backup instructions ([`b619514`](b619514505e9591b6b8ab196159f636488633426))

- Update README with uvx usage examples and better organization ([`73dc91e`](73dc91e3370c135ef28f7c7deb1c9fb2fee455f0))

- Document cache-optimized CLI with uvx support ([`2e07dcc`](2e07dcc3065eb75140ff2d8f003c667b59370d53))

- Rewrite README for profilectl Rust rewrite ([`776e7e0`](776e7e0eedb0384d9090d3380d50af03a208ef6e))


### Features

- Add git and starship configs, fix manifest paths ([`e11d8f0`](e11d8f04613da972ac521a693ea85534152943ed))

- Implement Typer-based CLI with 5 subcommands ([`d6155ee`](d6155eea5005f6d724f65d5ae5d0579015941658))

- Scaffold Rust workspace as chezmoi successor with profile-based design ([`cc5d5b8`](cc5d5b82753790251debc1b7d6eafa17d7b4013c))

- Rename dfiles → profilectl and wire up cargo-dist release pipeline ([`c547bbd`](c547bbd8ca848f2545a46e939d09767df475e629))


### Refactor

- Overhaul PyPI pipeline and purge stale Python scaffolding ([`323ec19`](323ec19edd4e411cc80abd66dff1b1f7d4c50b25))

- Update repo URLs from dotfiles → profilectl ([`628095b`](628095b590bcbbcaff4687ee57ba8929c701b4e6))




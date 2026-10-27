# fractic-currency-conversion

Currency conversion and historical exchange rates (openexchangerates.org
today, written to be provider-agnostic): `CurrencyConversionUtil` over an
`FxRepository` with cached and HTTP datasources, `RateTable` entity,
`CurrencyConversionCtxView`. Public, provided as-is
(`fractic-io/rust-currency-conversion`, identity `fractic`).

## Read first

- `~/.agents/AGENTS.md` (workspace orientation; loaded automatically by Claude
  Code, read it if you are Codex or Grok) and `~/.agents/guides/rust.md`
  (§2 "library crates" and §4 for the domain / data layering used here).

## Layout

Library with `src/impl/mod.rs` as the gatekeeper: `impl/domain/{entities,
repositories}`, `impl/data/{datasources,models,repositories}` (all
`pub(crate)`), `impl/exports` (what consumers see, re-exported from
`lib.rs`), plus `context.rs`, `errors.rs`, `util.rs` (the public façade).

## Working here

- No flake here: `nix develop /efs/fractic/storyengine/util --command cargo
  test`.
- A new provider is a new `data/datasources/<provider>_datasource.rs` (trait
  + Impl) with its `data/models/<provider>_model.rs`, wired in
  `fx_repository_impl.rs`; the domain and `util.rs` stay unchanged. Only
  domain types leave the crate, through `impl/exports`.
- Commit / PR titles: imperative, trailing period.

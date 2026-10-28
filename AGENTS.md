# fractic-currency-conversion

Currency conversion and historical exchange rates (openexchangerates.org
today, provider-agnostic by design): `CurrencyConversionUtil` over an
`FxRepository` with cached and HTTP datasources, `RateTable` entity,
`CurrencyConversionCtxView`. Library layout per `/efs/fractic/_/rust.md`
§2: `src/impl/mod.rs` gates `impl/domain/{entities,repositories}` and
`impl/data/{datasources,models,repositories}` (all `pub(crate)`),
`impl/exports` is what consumers see, plus `context.rs`, `errors.rs`,
`util.rs` (the public façade).

- A new provider is a new `data/datasources/<provider>_datasource.rs`
  (trait + Impl) with its `data/models/<provider>_model.rs`, wired in
  `fx_repository_impl.rs`; the domain and `util.rs` stay unchanged. Only
  domain types leave the crate, through `impl/exports`.
- Commit / PR titles: imperative, trailing period.

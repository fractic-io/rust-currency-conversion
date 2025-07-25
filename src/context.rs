use fractic_context::define_ctx_view;

define_ctx_view!(
    name: CurrencyConversionCtxView,
    env {},
    secrets {
        OPEN_EXCHANGE_RATES_APP_ID: String,
    },
    deps_overlay {},
    req_impl {}
);

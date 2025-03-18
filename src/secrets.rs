use fractic_env_config::{define_secret_key, define_secrets_config, SecretsConfigEnum};

define_secret_key!(OPEN_EXCHANGE_RATES_APP_ID);

define_secrets_config!(
    OpenExchangeRatesApiSecretsConfig,
    OpenExchangeRatesAppId => OPEN_EXCHANGE_RATES_APP_ID,
);

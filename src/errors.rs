use fractic_server_error::define_internal_error;

define_internal_error!(
    OpenExchangeRatesApiError,
    "Error calling Open Exchange Rates API '{function_name}': {details}.",
    { function_name: &str, details: &str }
);
define_internal_error!(
    CacheIoError,
    "Error reading/writing cache file at: {path}.",
    { path: &str }
);
define_internal_error!(
    CacheSerializationError,
    "Error serializing/deserializing cache file at: {path}.",
    { path: &str }
);

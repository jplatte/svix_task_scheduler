use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    /// The database URL for the postgres database.
    ///
    /// Example: `postgres://user:password@localhost/dbname`.
    pub database_url: String,

    /// The listen address.
    ///
    /// Default: `0.0.0.0:8000`.
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
}

fn default_listen_addr() -> String {
    "0.0.0.0:8000".to_owned()
}

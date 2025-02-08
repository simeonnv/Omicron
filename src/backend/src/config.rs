use argon2::Params;

pub const PORT: u16 = 8443;
pub const LISTENING_ON: &str = "0.0.0.0";

pub const DB_PORT: u16 = 3306;
pub const DB_ADDRESS: &str = "localhost";
pub const DB_NAME: &str = "Omicron";
pub const DB_USERNAME: &str = "root";
pub const DB_PASSWORD: &str = "root";

pub const CERT_PATH: &str = "/etc/letsencrypt/live/epsilonsv.duckdns.org/fullchain.pem";
pub const KEY_PATH: &str  = "/etc/letsencrypt/live/epsilonsv.duckdns.org/privkey.pem";

pub const ARGON2_PARAMS: Result<Params, argon2::Error> = Params::new(
    8192, // Memory cost
    1,    // Iterations
    2,    // Parallelism
    None, // Idk what is this tbh
);


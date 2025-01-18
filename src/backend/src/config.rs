use argon2::Params;

pub const PORT:u16 = 8433;
pub const LISTENING_ON: &str = "0.0.0.0";

pub const DB_PORT:u16 = 3306;
pub const DB_ADDRESS: &str = "epsilonsv.duckdns.org";
pub const DB_NAME: &str = "Omicron";
pub const DB_USERNAME: &str = "root";
pub const DB_PASSWORD: &str = "root";

pub const ARGON2_PARAMS: Result<Params, argon2::Error> = Params::new(
    8192, // Memory cost
    1,    // Reduce iterations
    2,    // Parallelism
    None,
);


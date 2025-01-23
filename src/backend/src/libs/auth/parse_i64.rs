use crate::error::Error;


pub fn parse_i64(input: String, fallback: &'static str) -> Result<i64, Error> {
    let subicron_id = input.parse::<i64>();
    match subicron_id {
        Ok(e) => Ok(e),
        Err(_) => Err(Error::NotFound(fallback.to_owned()))
    }
}
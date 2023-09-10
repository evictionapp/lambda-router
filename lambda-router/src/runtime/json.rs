use serde::Serialize;

use super::error::Error;

/// a simple re-export wrapper of serde_json::to_string for Result<T, E>
pub fn json<T: Serialize, E: Serialize>(result: &Result<T, E>) -> Result<String, Error> {
    Ok(serde_json::to_string(result)?)
}

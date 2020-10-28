
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Req {
    verb: String
}
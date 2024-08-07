use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,  // Decentralized Identifier (DID)
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub issuer: String,
    pub subject: String,
    pub claims: Vec<(String, String)>,  // Key-value pairs for claims
}

mod models;
use models::{User, Credential};
use ring::{rand::SystemRandom, signature::{Ed25519KeyPair, KeyPair}};
use sled::Db;
use serde_json;
use base64::Engine; // Import the Engine trait for base64 encoding

fn register_user(db: &Db, user_id: &str) -> User {
    let rng = SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
    
    // Access the public key correctly
    let public_key = key_pair.public_key().as_ref().to_vec();

    let user = User {
        id: user_id.to_string(),
        public_key: base64::engine::general_purpose::STANDARD.encode(public_key), // Use the new encoding method
    };

    // Store user in the database
    db.insert(user.id.clone(), serde_json::to_vec(&user).unwrap()).unwrap();
    user
}

fn issue_credential(db: &Db, issuer: &str, subject: &str, claims: Vec<(String, String)>) -> Credential {
    let credential = Credential {
        issuer: issuer.to_string(),
        subject: subject.to_string(),
        claims,
    };

    // Store credential in the database
    db.insert(subject.to_string(), serde_json::to_vec(&credential).unwrap()).unwrap();
    credential
}

fn verify_credential(db: &Db, subject: &str) -> Option<Credential> {
    if let Some(cred_bytes) = db.get(subject).unwrap() {
        let credential: Credential = serde_json::from_slice(&cred_bytes).unwrap();
        Some(credential)
    } else {
        None
    }
}

fn main() {
    let db = sled::open("identity_db").unwrap();

    // Register a user
    let user = register_user(&db, "user1");
    println!("Registered User: {:?}", user);

    // Issue a credential
    let claims = vec![("email".to_string(), "user1@example.com".to_string())];
    let credential = issue_credential(&db, &user.id, &user.id, claims);
    println!("Issued Credential: {:?}", credential);

    // Verify the credential
    if let Some(verified_credential) = verify_credential(&db, &user.id) {
        println!("Verified Credential: {:?}", verified_credential);
    } else {
        println!("Credential not found.");
    }
}

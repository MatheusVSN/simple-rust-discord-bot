use std::env;

use firebase_rs::Firebase;

pub fn get_database() -> Firebase {
    let database_url: String = env::var("FIREBASE_URL").expect("Expected a FIREBASE URL");
    Firebase::new(&database_url).unwrap()
}

extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash};

// Hash user password.
pub fn hash_password(password: String) -> String {
	let hashed_password = hash(password, DEFAULT_COST).expect("Fail to hash password!");

	hashed_password
}
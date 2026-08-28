fn main() {
    // Keep sqlx::migrate!() up to date when migration files change.
    println!("cargo:rerun-if-changed=migrations");
}

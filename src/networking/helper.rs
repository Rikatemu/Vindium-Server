use rand::{thread_rng, distributions::Alphanumeric, Rng};

// Generate a random ID for the entity
pub async fn generate_entity_id() -> String {
    return thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect();
}
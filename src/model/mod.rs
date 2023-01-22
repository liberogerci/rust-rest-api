// Importing serialize && deserialize from serde framework
use serde::{Deserialize, Serialize};

// Movie struct definition && serialize/deserialize applying
#[derive(Clone, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub genre: String
}
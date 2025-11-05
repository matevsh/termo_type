pub mod models;
pub mod storage;

pub use models::{Profile, BestScore};
pub use storage::{save_profile, load_profile};

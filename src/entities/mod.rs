#![allow(unused)]
use rand::{random, Rng};

pub mod player;

#[derive(Debug, Clone)]
pub struct BaseStats {
    pub id: i32,
    pub name: String,
    pub health: i32,
}

impl Default for BaseStats {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen_range(1..100001);
            Self {
            id,
            name: format!("en-{}", id),
            health: 275,
        }
    }
}
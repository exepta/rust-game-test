use crate::entities::{BaseStats};

#[derive(Debug, Clone)]
pub struct Player  {
    pub base: BaseStats,
    pub attack: i32,
}
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct BagItem {
    pub id: u64,
    pub weight: f32,
    pub utility: u32
}

impl BagItem {
    pub fn from(id: u64, weight: f32, utility: u32) -> Self {
        return BagItem { id, weight, utility };
    }
}
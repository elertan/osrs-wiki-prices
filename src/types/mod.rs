#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct ItemId(u32);

impl ItemId {
    pub fn new(id: u32) -> Self {
        ItemId(id)
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}
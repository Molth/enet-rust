#[derive(Copy, Clone, Default)]
pub struct ENetSymbol {
    pub value: u8,
    pub count: u8,
    pub under: u16,
    pub left: u16,
    pub right: u16,
    pub symbols: u16,
    pub escapes: u16,
    pub total: u16,
    pub parent: u16,
}

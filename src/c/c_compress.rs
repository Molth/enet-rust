use crate::h_compress::ENetSymbol;

pub const ENET_RANGE_CODER_TOP: u32 = 1 << 24;
pub const ENET_RANGE_CODER_BOTTOM: u32 = 1 << 16;

pub const ENET_CONTEXT_SYMBOL_DELTA: u32 = 3;
pub const ENET_CONTEXT_SYMBOL_MINIMUM: u32 = 1;
pub const ENET_CONTEXT_ESCAPE_MINIMUM: u32 = 1;

pub const ENET_SUBCONTEXT_ORDER: u32 = 2;
pub const ENET_SUBCONTEXT_SYMBOL_DELTA: u32 = 2;
pub const ENET_SUBCONTEXT_ESCAPE_DELTA: u32 = 5;

pub struct ENetRangeCoder {
    pub symbols: [ENetSymbol; 4096],
}

pub fn enet_range_coder_create() -> ENetRangeCoder {
    ENetRangeCoder {
        symbols: [ENetSymbol::default(); 4096],
    }
}

pub fn enet_range_coder_destroy(_: ENetRangeCoder) {}

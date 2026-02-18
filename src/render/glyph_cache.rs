use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GlyphEntry {
    pub symbol: &'static str,
    pub atlas_index: usize,
}

#[derive(Debug, Default, Clone)]
pub struct GlyphCache {
    entries: HashMap<&'static str, GlyphEntry>,
}

impl GlyphCache {
    pub fn register(&mut self, symbol: &'static str) {
        let next_idx = self.entries.len();
        self.entries.entry(symbol).or_insert(GlyphEntry {
            symbol,
            atlas_index: next_idx,
        });
    }

    pub fn warmup_music_symbols(&mut self) {
        for symbol in ["𝄞", "𝄢", "𝄡", "♯", "♭", "●", "○", "𝅭", "𝅮"] {
            self.register(symbol);
        }
    }
}

impl GlyphCache {
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn atlas_checksum(&self) -> usize {
        self.entries
            .values()
            .map(|entry| entry.atlas_index + entry.symbol.len())
            .sum()
    }
}

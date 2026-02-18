use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU64 = AtomicU64::new(1);

/// Identificador estável para entidades de domínio.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(pub u128);

impl Id {
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let c = COUNTER.fetch_add(1, Ordering::Relaxed) as u128;
        Self((now << 16) ^ c)
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        u128::from_str_radix(hex, 16).ok().map(Self)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}
impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

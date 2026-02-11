use anyhow::{Context, Result};

pub struct CANFrame {
    pub timestamp: f64, // parsed timestamp
    pub iface: String,  // interface name
    pub id: u32,        // CAN ID (11/29-bit stored in u32)
    pub data: Vec<u8>,  // payload bytes
    pub raw: String,    // original trimmed line
    pub line_no: usize, // 1-based line number
}

pub fn decode_hex_bytes(s: &str) -> Result<Vec<u8>> {
    if s.is_empty() {
        return Ok(Vec::new()); // "123#"
    }

    // classic CAN: 0..8 bytes, even-length hex
    if s.len() % 2 != 0 {
        anyhow::bail!("hex payload has odd length");
    }
    if s.len() > 16 {
        anyhow::bail!("hex payload too long (max 8 bytes)");
    }

    Ok(hex::decode(s).with_context(|| "invalid hex payload")?)
}

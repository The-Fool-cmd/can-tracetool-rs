use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

struct Frame {
    timestamp: f64, // parsed timestamp
    iface: String,  // interface name
    id: u32,        // CAN ID (11/29-bit stored in u32)
    data: Vec<u8>,  // payload bytes
    raw: String,    // original trimmed line
    line_no: usize, // 1-based line number
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect(); // CLI args

    // can-tracetool status <file>
    if args.len() < 3 {
        eprintln!("Usage: can-tracetool status <file>");
        anyhow::bail!("Invalid arguments");
    }
    if args[1] != "status" {
        eprintln!("Usage: can-tracetool status <file>");
        anyhow::bail!("Invalid arguments");
    }

    let filepath = args
        .get(2)
        .with_context(|| "'status' requires a <file> argument")?;

    let file =
        File::open(filepath).with_context(|| format!("Failed to open input file: {}", filepath))?;

    let mut frames: Vec<Frame> = Vec::new();
    let (valid_lines, invalid_lines, ignored_lines) = parse_file(file, &mut frames)?;

    println!(
        "Valid lines: {}, Invalid lines: {}, Ignored lines: {}",
        valid_lines, invalid_lines, ignored_lines
    );

    Ok(())
}

fn parse_file(mut file: File, frames: &mut Vec<Frame>) -> Result<(i32, i32, i32)> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| "Failed to read file as UTF-8 text")?;

    let mut ignored_lines: i32 = 0;
    let mut invalid_lines: i32 = 0;
    let mut valid_lines: i32 = 0;

    for (idx, line) in contents.lines().enumerate() {
        let s = line.trim();

        // ignore obvious non-data
        if s.is_empty() || s.starts_with('#') {
            ignored_lines += 1;
            continue;
        }

        // expect: "(ts) iface id#data"
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() != 3 {
            invalid_lines += 1;
            continue;
        }

        // strip () from timestamp if present
        let ts_token = tokens[0].trim_matches(|c| c == '(' || c == ')');
        let timestamp: f64 = match ts_token.parse::<f64>() {
            Ok(v) if v.is_finite() => v,
            _ => {
                invalid_lines += 1;
                continue;
            }
        };

        let iface = tokens[1].to_string();
        let id_data = tokens[2];

        // expect "id#data"
        let (id_str, data_str) = match id_data.split_once('#') {
            Some(x) => x,
            None => {
                invalid_lines += 1;
                continue;
            }
        };

        // allow "0x" in ID
        let id_str = id_str.strip_prefix("0x").unwrap_or(id_str);
        let id = match u32::from_str_radix(id_str, 16) {
            Ok(v) if v <= 0x1FFF_FFFF => v, // 29-bit max
            _ => {
                invalid_lines += 1;
                continue;
            }
        };

        let data = match decode_hex_bytes(data_str) {
            Ok(v) => v,
            Err(_) => {
                invalid_lines += 1;
                continue;
            }
        };

        frames.push(Frame {
            timestamp,
            iface,
            id,
            data,
            raw: s.to_string(),
            line_no: idx + 1,
        });

        valid_lines += 1;
    }

    Ok((valid_lines, invalid_lines, ignored_lines))
}

fn decode_hex_bytes(s: &str) -> Result<Vec<u8>> {
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

// Matrix cells carry argv directly; no shell interprets quotes or backslashes.
pub fn encode_matrix_arguments(arguments: &[String]) -> Result<String, String> {
    validate(arguments)?;
    serde_json::to_string(arguments)
        .map(|json| format!("json:{json}"))
        .map_err(|error| format!("encode matrix arguments: {error}"))
}

pub fn decode_matrix_arguments(cell: &str) -> Result<Vec<String>, String> {
    let arguments = if let Some(json) = cell.trim_start().strip_prefix("json:") {
        serde_json::from_str::<Vec<String>>(json)
            .map_err(|error| format!("invalid JSON matrix arguments: {error}"))?
    } else {
        // Preserve the original whitespace-separated matrix format.
        cell.split_whitespace().map(str::to_owned).collect()
    };
    validate(&arguments)?;
    Ok(arguments)
}

fn validate(arguments: &[String]) -> Result<(), String> {
    if arguments.iter().any(|argument| argument.contains('\0')) {
        return Err("matrix arguments cannot contain NUL".to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_roundtrip_preserves_native_arguments() {
        let arguments = ["", "C:\\Program Files\\Spiral\\", "a\"b", "ü 東京", "line\nnext\tcolumn", "$(literal)"]
            .map(str::to_owned).to_vec();
        let cell = encode_matrix_arguments(&arguments).unwrap();
        assert!(!cell.contains(['\n', '\r', '\t']));
        assert_eq!(decode_matrix_arguments(&cell).unwrap(), arguments);
        assert_eq!(decode_matrix_arguments("json:[]").unwrap(), Vec::<String>::new());
    }

    #[test]
    fn legacy_cells_keep_their_existing_tokenization() {
        assert_eq!(decode_matrix_arguments(" --backend  Rust main.spi out.rs ").unwrap(), ["--backend", "Rust", "main.spi", "out.rs"]);
        assert_eq!(decode_matrix_arguments("[literal]").unwrap(), ["[literal]"]);
    }

    #[test]
    fn malformed_json_is_rejected_without_legacy_fallback() {
        for cell in ["json:", "json:{}", "json:[1]", "json:[null]", "json:[\"unterminated]", r#"json:["nul\u0000value"]"#] {
            assert!(decode_matrix_arguments(cell).is_err(), "accepted {cell}");
        }
        assert!(encode_matrix_arguments(&["nul\0value".to_owned()]).is_err());
    }
}

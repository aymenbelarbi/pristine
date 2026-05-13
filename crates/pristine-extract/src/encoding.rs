//! Encoding detection and conversion

/// Detect encoding from byte order mark
pub fn detect_encoding(data: &[u8]) -> Option<&'static encoding_rs::Encoding> {
    if data.len() >= 3 {
        // UTF-8 BOM
        if data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF {
            return Some(encoding_rs::UTF_8);
        }
        // UTF-16 LE BOM
        if data[0] == 0xFF && data[1] == 0xFE {
            return Some(encoding_rs::UTF_16LE);
        }
        // UTF-16 BE BOM
        if data[0] == 0xFE && data[1] == 0xFF {
            return Some(encoding_rs::UTF_16BE);
        }
    }
    None
}

/// Convert bytes to UTF-8 string
pub fn to_utf8(data: &[u8]) -> Result<String, std::string::FromUtf8Error> {
    if let Some(encoding) = detect_encoding(data) {
        let (cow, _, had_errors) = encoding.decode(data);
        if !had_errors {
            Ok(cow.into_owned())
        } else {
            String::from_utf8(data.to_vec())
        }
    } else {
        String::from_utf8(data.to_vec())
    }
}

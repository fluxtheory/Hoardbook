//! JSON Canonicalization Scheme (JCS) — RFC 8785.
//!
//! Produces a deterministic byte representation of a JSON value:
//! - Object keys sorted lexicographically by Unicode code point value
//! - No insignificant whitespace
//! - String escaping follows ES6 / serde_json conventions
//!
//! All our object keys are ASCII, so byte-order comparison is equivalent
//! to Unicode code point order.

use serde_json::Value;

/// Returns the JCS-canonical UTF-8 byte representation of `value`.
/// This is the bytes to sign / verify over.
pub fn canonicalize(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    write_value(value, &mut out);
    out
}

fn write_value(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Null => out.extend_from_slice(b"null"),
        Value::Bool(true) => out.extend_from_slice(b"true"),
        Value::Bool(false) => out.extend_from_slice(b"false"),
        Value::Number(n) => out.extend_from_slice(n.to_string().as_bytes()),
        Value::String(_) => {
            // Delegate to serde_json: it produces the same escaping as ES6 JSON.stringify
            // for all valid UTF-8 strings (escapes U+0000–U+001F, `"`, `\`; everything
            // else is passed through as UTF-8).
            out.extend_from_slice(serde_json::to_string(value).unwrap().as_bytes());
        }
        Value::Array(arr) => {
            out.push(b'[');
            for (i, v) in arr.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                write_value(v, out);
            }
            out.push(b']');
        }
        Value::Object(map) => {
            out.push(b'{');
            // Sort by key — Unicode code point order; ASCII keys are equivalent to byte order.
            let mut entries: Vec<(&String, &Value)> = map.iter().collect();
            entries.sort_by_key(|(k, _)| k.as_str());
            for (i, (k, v)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                // Key as JSON string
                out.extend_from_slice(serde_json::to_string(k).unwrap().as_bytes());
                out.push(b':');
                write_value(v, out);
            }
            out.push(b'}');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn object_keys_sorted() {
        let v = json!({ "z": 1, "a": 2, "m": 3 });
        let canonical = String::from_utf8(canonicalize(&v)).unwrap();
        assert_eq!(canonical, r#"{"a":2,"m":3,"z":1}"#);
    }

    #[test]
    fn nested_object() {
        let v = json!({ "b": { "y": 1, "x": 2 }, "a": true });
        let canonical = String::from_utf8(canonicalize(&v)).unwrap();
        assert_eq!(canonical, r#"{"a":true,"b":{"x":2,"y":1}}"#);
    }

    #[test]
    fn array_preserves_order() {
        let v = json!([3, 1, 2]);
        let canonical = String::from_utf8(canonicalize(&v)).unwrap();
        assert_eq!(canonical, "[3,1,2]");
    }

    #[test]
    fn string_escaping() {
        let v = json!("hello\nworld");
        let canonical = String::from_utf8(canonicalize(&v)).unwrap();
        assert_eq!(canonical, r#""hello\nworld""#);
    }

    #[test]
    fn rfc8785_cross_vector() {
        // Hardcoded expected output from a fixed input.
        // If the JCS implementation drifts (wrong sort, wrong escaping, whitespace inserted),
        // all signatures produced by the old code become unverifiable against this output.
        // Any conforming JCS implementation — in any language — must agree on this exact string.
        let payload = json!({
            "z": false,
            "a": 1,
            "m": [3, 1, 2],
            "nested": { "y": "hello\nworld", "x": null }
        });
        let expected =
            r#"{"a":1,"m":[3,1,2],"nested":{"x":null,"y":"hello\nworld"},"z":false}"#;
        let canonical = String::from_utf8(canonicalize(&payload)).unwrap();
        assert_eq!(canonical, expected);
    }

    #[test]
    fn deterministic_across_insertion_order() {
        // serde_json::Map preserves insertion order by default (IndexMap).
        // JCS must still sort regardless.
        let a = json!({ "z": 1, "a": 2 });
        let b = json!({ "a": 2, "z": 1 });
        assert_eq!(canonicalize(&a), canonicalize(&b));
    }
}

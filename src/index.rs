use http::{header, Request, Response, StatusCode};
use percent_encoding::percent_decode;

const RABBIT0: &str = "🐰";
const RABBIT1: &str = "🐇";

fn str_to_rabbitary(text: &str) -> String {
    text.as_bytes()
        .into_iter()
        .map(|b| format!("{:08b}", b))
        .collect::<String>()
        .replace("0", RABBIT0)
        .replace("1", RABBIT1)
}

fn rabbitary_to_str(rabbitary: &str) -> String {
    let binary_string = rabbitary.replace(RABBIT0, "0").replace(RABBIT1, "1");
    let mut peek_string = binary_string.chars().peekable();
    let mut bytes_vec: Vec<u8> = Vec::new();
    while peek_string.peek().is_some() {
        let chunk: String = peek_string.by_ref().take(8).collect();
        bytes_vec.push(u8::from_str_radix(&chunk[..], 2).unwrap());
    }
    String::from_utf8(bytes_vec).unwrap()
}

fn handler(request: Request<()>) -> http::Result<Response<String>> {
    let response = match request.uri().query() {
        Some(query) => match query.split('&').next() {
            Some(kv) => {
                let mut split_kv = kv.split('=');
                let key = split_kv.next();
                let value = split_kv.next().unwrap();
                let decoded = percent_decode(value.as_bytes()).decode_utf8().unwrap();
                match key {
                    Some(value) if value == "text" => Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                        .body(str_to_rabbitary(&decoded[..])),
                    Some(value) if value == "rabbitary" => Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                        .body(rabbitary_to_str(&decoded[..])),
                    _ => Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body("Could not find either text or rabbitary arguments.".to_string()),
                }
            }
            _ => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Could not find either text or rabbitary arguments.".to_string()),
        },
        _ => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Could not parse query.".to_string()),
    };

    Ok(response.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_translate_text() {
        let text = "hey";
        let rabbitary = str_to_rabbitary(text);
        assert_eq!("🐰🐇🐇🐰🐇🐰🐰🐰🐰🐇🐇🐰🐰🐇🐰🐇🐰🐇🐇🐇🐇🐰🐰🐇", rabbitary);
    }

    #[test]
    fn must_translate_rabbitary() {
        let rabbitary = "🐰🐇🐇🐰🐇🐰🐰🐰🐰🐇🐇🐰🐰🐇🐰🐇🐰🐇🐇🐇🐇🐰🐰🐇";
        let text = rabbitary_to_str(rabbitary);
        assert_eq!("hey", text);
    }
}

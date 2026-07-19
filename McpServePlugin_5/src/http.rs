use crate::*;
use crate::functions::handle_mcp_request;
use std::io::Write;
use std::net::TcpStream;


pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}


pub struct HttpPart {
    pub method: String,
    pub path: String,
    pub content_length: usize,
    pub body_start: usize,
    pub total_len: usize,
}


pub fn try_parse_http_request(buffer: &[u8]) -> Option<HttpPart> {
    let header_end = find_subsequence(buffer, b"\r\n\r\n")?;
    let header_bytes = &buffer[..header_end];
    let header_str = std::str::from_utf8(header_bytes).ok()?;

    let mut lines = header_str.split("\r\n");
    let request_line = lines.next()?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();

    let mut content_length = 0usize;
    for line in lines {
        let lower = line.to_lowercase();
        if let Some(val) = lower.strip_prefix("content-length:") {
            content_length = val.trim().parse::<usize>().unwrap_or(0);
        }
    }

    let body_start = header_end + 4;
    let total_len = body_start + content_length;
    if buffer.len() < total_len {
        return None;
    }

    Some(HttpPart { method, path, content_length, body_start, total_len })
}


pub fn build_http_response(body: &str, status: u16, status_text: &str) -> Vec<u8> {
    let mut resp = Vec::with_capacity(512 + body.len());
    resp.extend_from_slice(format!("HTTP/1.1 {} {}\r\n", status, status_text).as_bytes());
    resp.extend_from_slice(b"Content-Type: application/json\r\n");
    resp.extend_from_slice(format!("Content-Length: {}\r\n", body.len()).as_bytes());
    resp.extend_from_slice(b"Access-Control-Allow-Origin: *\r\n");
    resp.extend_from_slice(b"Access-Control-Allow-Methods: POST, GET, OPTIONS\r\n");
    resp.extend_from_slice(b"Access-Control-Allow-Headers: Content-Type, Accept\r\n");
    resp.extend_from_slice(b"Connection: keep-alive\r\n");
    resp.extend_from_slice(b"\r\n");
    resp.extend_from_slice(body.as_bytes());
    resp
}


pub fn build_options_response() -> Vec<u8> {
    let mut resp = Vec::with_capacity(512);
    resp.extend_from_slice(b"HTTP/1.1 204 No Content\r\n");
    resp.extend_from_slice(b"Access-Control-Allow-Origin: *\r\n");
    resp.extend_from_slice(b"Access-Control-Allow-Methods: POST, GET, OPTIONS\r\n");
    resp.extend_from_slice(b"Access-Control-Allow-Headers: Content-Type, Accept\r\n");
    resp.extend_from_slice(b"Access-Control-Max-Age: 86400\r\n");
    resp.extend_from_slice(b"Connection: keep-alive\r\n");
    resp.extend_from_slice(b"Content-Length: 0\r\n");
    resp.extend_from_slice(b"\r\n");
    resp
}


pub fn process_http_buffer(buffer: &mut Vec<u8>, stream: &mut TcpStream) {
    loop {
        let Some(part) = try_parse_http_request(buffer) else {
            return;
        };

        let body_slice = if part.content_length > 0 {
            &buffer[part.body_start..part.body_start + part.content_length]
        } else {
            &[] as &[u8]
        };

        println!("HTTP {} {}", part.method, part.path);

        let response_bytes = if part.method.eq_ignore_ascii_case("OPTIONS") {
            build_options_response()
        } else if part.method.eq_ignore_ascii_case("POST") && part.path.eq_ignore_ascii_case("/mcp") {
            let rpc_response = if let Ok(body_str) = std::str::from_utf8(body_slice) {
                match serde_json::from_str::<Value>(body_str) {
                    Ok(req) => {
                        println!("********** MCP Request Start **********");
                        println!("{}", serde_json::to_string_pretty(&req).unwrap_or_else(|_| req.to_string()));
                        println!("********** MCP Request End **********");
                        let resp = handle_mcp_request(&req).unwrap_or_else(|e| {
                            json!({
                                "jsonrpc": "2.0",
                                "id": req["id"],
                                "error": {
                                    "code": -32603,
                                    "message": e
                                }
                            })
                        });
                        println!("********** MCP Response Start **********");
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap_or_else(|_| resp.to_string()));
                        println!("********** MCP Response End **********");
                        resp
                    }
                    Err(e) => {
                        let resp = json!({
                            "jsonrpc": "2.0",
                            "id": Value::Null,
                            "error": {
                                "code": -32700,
                                "message": format!("Parse error: {}", e)
                            }
                        });
                        println!("********** MCP Response Start **********");
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap_or_else(|_| resp.to_string()));
                        println!("********** MCP Response End **********");
                        resp
                    }
                }
            } else {
                let resp = json!({
                    "jsonrpc": "2.0",
                    "id": Value::Null,
                    "error": { "code": -32700, "message": "Invalid UTF-8 in body" }
                });
                println!("********** MCP Response Start **********");
                println!("{}", serde_json::to_string_pretty(&resp).unwrap_or_else(|_| resp.to_string()));
                println!("********** MCP Response End **********");
                resp
            };
            build_http_response(&rpc_response.to_string(), 200, "OK")
        } else if part.path.eq_ignore_ascii_case("/health") || part.path.eq_ignore_ascii_case("/") {
            let body = json!({ "status": "ok", "service": "McpServePlugin_1 MCP" }).to_string();
            build_http_response(&body, 200, "OK")
        } else {
            let body = json!({ "error": "Not Found", "path": part.path }).to_string();
            build_http_response(&body, 404, "Not Found")
        };

        let _ = stream.write_all(&response_bytes);
        let _ = stream.flush();

        buffer.drain(0..part.total_len);
        if buffer.is_empty() {
            return;
        }
    }
}

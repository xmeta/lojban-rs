use lojban::{classify_word, friendly_error, parse, tree, word_stats, Rule};
use pest::error::{Error, ErrorVariant, InputLocation, LineColLocation};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Instant;

const INDEX_HTML: &str = include_str!("web_playground/index.html");
const APP_JS: &str = include_str!("web_playground/app.js");
const STYLE_CSS: &str = include_str!("web_playground/style.css");
const MAX_REQUEST_BYTES: usize = 128 * 1024;
const MAX_REGRESSION_CASES: usize = 200;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8787")?;
    println!("Lojban Parser Playground: http://127.0.0.1:8787");
    println!("Press Ctrl+C to stop.");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(error) = handle_connection(&mut stream) {
                    eprintln!("request error: {error}");
                }
            }
            Err(error) => eprintln!("connection error: {error}"),
        }
    }
    Ok(())
}
fn handle_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    let Some(request) = read_request(stream)? else {
        return Ok(());
    };

    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => send_response(stream, "200 OK", "text/html; charset=utf-8", INDEX_HTML),
        ("GET", "/app.js") => {
            send_response(stream, "200 OK", "text/javascript; charset=utf-8", APP_JS)
        }
        ("GET", "/style.css") => {
            send_response(stream, "200 OK", "text/css; charset=utf-8", STYLE_CSS)
        }
        ("POST", "/api/parse") => {
            let input = String::from_utf8_lossy(&request.body);
            let body = parse_response(&input);
            send_response(stream, "200 OK", "application/json; charset=utf-8", &body)
        }
        ("POST", "/api/regression") => {
            let input = String::from_utf8_lossy(&request.body);
            let body = regression_response(&input);
            send_response(stream, "200 OK", "application/json; charset=utf-8", &body)
        }
        _ => send_response(
            stream,
            "404 Not Found",
            "text/plain; charset=utf-8",
            "Not found",
        ),
    }
}
struct Request {
    method: String,
    path: String,
    body: Vec<u8>,
}

fn read_request(stream: &mut TcpStream) -> std::io::Result<Option<Request>> {
    let mut data = Vec::new();
    let mut chunk = [0u8; 4096];
    let header_end;

    loop {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            return Ok(None);
        }
        data.extend_from_slice(&chunk[..read]);
        if data.len() > MAX_REQUEST_BYTES {
            return Ok(None);
        }
        if let Some(pos) = data.windows(4).position(|w| w == b"\r\n\r\n") {
            header_end = pos + 4;
            break;
        }
    }

    let header_text = String::from_utf8_lossy(&data[..header_end]);
    let mut lines = header_text.lines();
    let Some(request_line) = lines.next() else {
        return Ok(None);
    };
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().unwrap_or_default().to_string();
    let path = request_parts.next().unwrap_or("/").to_string();
    let content_length = lines
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        })
        .unwrap_or(0);

    while data.len() < header_end + content_length {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            break;
        }
        data.extend_from_slice(&chunk[..read]);
        if data.len() > MAX_REQUEST_BYTES {
            return Ok(None);
        }
    }

    let body_end = (header_end + content_length).min(data.len());
    Ok(Some(Request {
        method,
        path,
        body: data[header_end..body_end].to_vec(),
    }))
}
fn parse_response(input: &str) -> String {
    let stats = word_stats(input);
    let parse_started = Instant::now();
    let parsed = parse(input);
    let elapsed_ms = parse_started.elapsed().as_secs_f64() * 1000.0;
    let stats_json = format!(
        "{{\"tokens\":{},\"gismu\":{},\"lujvo\":{},\"fuivla\":{},\"cmevla\":{},\"cmavo\":{},\"unknown\":{}}}",
        stats.tokens,
        stats.gismu,
        stats.lujvo,
        stats.fuivla,
        stats.cmevla,
        stats.cmavo,
        stats.unknown
    );

    match parsed {
        Ok(pairs) => {
            let ast = tree::to_json(pairs.clone());
            let pretty = tree::to_json_pretty(pairs.clone());
            let tree_text = tree::to_tree_string(pairs.clone());
            let sexpr = tree::to_sexpr(pairs.clone());
            let leaves = tree::leaf_spans(pairs);
            let leaves_json = leaves
                .iter()
                .map(|leaf| leaf_json(leaf))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{{\"ok\":true,\"elapsed_ms\":{elapsed_ms:.3},\"stats\":{stats_json},\"ast\":{ast},\"pretty\":{},\"tree\":{},\"sexpr\":{},\"leaves\":[{leaves_json}]}}",
                json_string(&pretty),
                json_string(&tree_text),
                json_string(&sexpr)
            )
        }
        Err(error) => {
            let details = error_details_json(&error);
            format!(
                "{{\"ok\":false,\"elapsed_ms\":{elapsed_ms:.3},\"stats\":{stats_json},\"error\":{},\"details\":{details}}}",
                json_string(&friendly_error(&error))
            )
        }
    }
}

fn regression_response(input: &str) -> String {
    let batch_started = Instant::now();
    let mut cases = Vec::new();
    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut processed = 0usize;
    let mut truncated = false;

    for (index, raw_line) in input.lines().enumerate() {
        let text = raw_line.trim_end_matches('\r');
        if text.trim().is_empty() {
            continue;
        }
        if processed >= MAX_REGRESSION_CASES {
            truncated = true;
            break;
        }
        processed += 1;
        let started = Instant::now();
        let parsed = parse(text);
        let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
        match parsed {
            Ok(_) => {
                passed += 1;
                cases.push(format!(
                    "{{\"line\":{},\"text\":{},\"ok\":true,\"elapsed_ms\":{elapsed_ms:.3}}}",
                    index + 1,
                    json_string(text)
                ));
            }
            Err(error) => {
                failed += 1;
                let details = error_details_json(&error);
                cases.push(format!(
                    "{{\"line\":{},\"text\":{},\"ok\":false,\"elapsed_ms\":{elapsed_ms:.3},\"error\":{},\"details\":{details}}}",
                    index + 1,
                    json_string(text),
                    json_string(&friendly_error(&error))
                ));
            }
        }
    }

    let total = passed + failed;
    let elapsed_ms = batch_started.elapsed().as_secs_f64() * 1000.0;
    format!(
        "{{\"total\":{total},\"passed\":{passed},\"failed\":{failed},\"elapsed_ms\":{elapsed_ms:.3},\"truncated\":{truncated},\"cases\":[{}]}}",
        cases.join(",")
    )
}

fn error_details_json(error: &Error<Rule>) -> String {
    let (start, end) = match error.location {
        InputLocation::Pos(pos) => (pos, pos),
        InputLocation::Span((start, end)) => (start, end),
    };
    let (line, column) = match error.line_col {
        LineColLocation::Pos((line, column)) => (line, column),
        LineColLocation::Span((line, column), _) => (line, column),
    };
    let expected = match &error.variant {
        ErrorVariant::ParsingError { positives, .. } => positives
            .iter()
            .take(12)
            .map(|rule| json_string(&format!("{rule:?}")))
            .collect::<Vec<_>>()
            .join(","),
        ErrorVariant::CustomError { .. } => String::new(),
    };
    format!(
        "{{\"start\":{start},\"end\":{end},\"line\":{line},\"column\":{column},\"expected\":[{expected}]}}"
    )
}

fn leaf_json(leaf: &tree::LeafSpan) -> String {
    let bare = leaf.text.trim_start_matches(['.', ',', '!', '?']);
    format!(
        "{{\"rule\":{},\"text\":{},\"start\":{},\"end\":{},\"class\":{}}}",
        json_string(&format!("{:?}", leaf.rule)),
        json_string(&leaf.text),
        leaf.start,
        leaf.end,
        json_string(classify_word(bare))
    )
}
fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < ' ' => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn send_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &str,
) -> std::io::Result<()> {
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nContent-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'none'\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body.as_bytes())?;
    stream.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_api_success_includes_timing_and_outputs() {
        let body = parse_response("mi tavla do");
        assert!(body.contains("\"ok\":true"), "{body}");
        assert!(body.contains("\"elapsed_ms\":"), "{body}");
        assert!(body.contains("\"ast\":"), "{body}");
        assert!(body.contains("\"leaves\":"), "{body}");
    }

    #[test]
    fn parse_api_error_includes_friendly_error() {
        let body = parse_response("qqq");
        assert!(body.contains("\"ok\":false"), "{body}");
        assert!(body.contains("解析エラー"), "{body}");
        assert!(body.contains("\"elapsed_ms\":"), "{body}");
        assert!(body.contains("\"details\":"), "{body}");
        assert!(body.contains("\"expected\":["), "{body}");
    }

    #[test]
    fn regression_api_reports_pass_and_fail_cases() {
        let body = regression_response("mi tavla do\nqqq\n\nle mlatu cu cadzu\n");
        assert!(body.contains("\"total\":3"), "{body}");
        assert!(body.contains("\"passed\":2"), "{body}");
        assert!(body.contains("\"failed\":1"), "{body}");
        assert!(body.contains("\"line\":2"), "{body}");
        assert!(body.contains("\"details\":"), "{body}");
    }

    #[test]
    fn regression_api_enforces_case_limit() {
        let input = std::iter::repeat_n("mi tavla do", MAX_REGRESSION_CASES + 1)
            .collect::<Vec<_>>()
            .join("\n");
        let body = regression_response(&input);
        assert!(body.contains("\"truncated\":true"), "{body}");
        assert!(
            body.contains(&format!("\"total\":{MAX_REGRESSION_CASES}")),
            "{body}"
        );
    }
}

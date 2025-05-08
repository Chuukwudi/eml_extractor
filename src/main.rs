use std::fs;
use mail_parser::{Message, Address, HeaderValue, MessageParser};



fn parse_eml_from_filepath(file_path: &str) -> mail_parser::Message {
    let contents = fs::read(file_path).expect("Could not read the file");
    let message = MessageParser::default()
        .with_address_headers()
        .with_mime_headers()
        .with_date_headers()
        .with_message_ids()
        .with_minimal_headers()
        .parse(&contents);

    message.map_or_else(|| panic!("Failed to parse the message"), Message::into_owned)
}

fn get_details(message: &mail_parser::Message) -> String {
    let addresses: Vec<(&str, Option<&Address>)> = vec![
        ("sender", message.sender()),
        ("to", message.to()),
        ("cc", message.cc()),
        ("bcc", message.bcc()),
        ("from", message.from()),
        ("reply_to", message.reply_to()),
        ("resent_bcc", message.resent_bcc()),
        ("resent_cc", message.resent_cc()),
        ("resent_from", message.resent_from()),
        ("resent_sender", message.resent_sender()),
        ("resent_to", message.resent_to()),
    ];

    let headers: Vec<(&str, &HeaderValue)> = vec![
        ("comments", message.comments()),
        ("in_reply_to", message.in_reply_to()),
        ("keywords", message.keywords()),
        ("list_archive", message.list_archive()),
        ("list_help", message.list_help()),
        ("list_id", message.list_id()),
        ("list_owner", message.list_owner()),
        ("list_post", message.list_post()),
        ("list_subscribe", message.list_subscribe()),
        ("list_unsubscribe", message.list_unsubscribe()),
        ("mime_version", message.mime_version()),
        ("references", message.references()),
        ("return_path", message.return_path()),
    ];

    let strings = vec![
        ("message_id", message.message_id()),
        ("subject", message.subject()),
        ("return_address", message.return_address()),
        ("thread_name", message.thread_name()),
    ];

    let u_sizes = vec![
        ("attachment_count", message.attachment_count()),
        ("text_body_count", message.text_body_count()),
        ("html_body_count", message.html_body_count())
    ];

    let mut map = serde_json::Map::new();
    for (key, address) in addresses {
        if let Some(addr) = address {
            map.insert(key.to_string(), serde_json::to_value(addr).unwrap());
        }
    }

    for (key, value) in headers {
        if !value.is_empty() {
            let value_json =
                serde_json::to_value(value).unwrap_or(serde_json::Value::Array(Vec::new()));
            map.insert(key.to_string(), value_json);
        }
    }

    for (key, value) in strings {
        if let Some(value) = value {
            map.insert(
                key.to_string(),
                serde_json::Value::String(value.to_string()),
            );
        }
    }

    for (key, value) in u_sizes {
        map.insert(key.to_string(), serde_json::Value::Number(value.into()));
    }

    map.insert(
        "date".to_string(),
        serde_json::Value::String(message.date().expect("Date header not found").to_string()),
    );
    serde_json::to_string_pretty(&map).unwrap()
}


fn get_body(message: &mail_parser::Message, ) -> (String, String) {
    let text_body_count = message.text_body_count();
    let html_body_count = message.html_body_count();
    let mut text = String::new();
    let mut html = String::new();
    for n in 0..text_body_count {
        let text_body = message.body_text(n);
        if let Some(txt) = text_body.as_deref() {
            text.push_str(txt);
        }
    }
    for n in 0..html_body_count {
        let html_body = message.body_html(n);
        if let Some(html_) = html_body.as_deref() {
            html.push_str(html_);
        }
    }
    if text.is_empty() {
        text = String::from("No text body found");
    }
    if html.is_empty() {
        html = String::from("No HTML body found");
    }
    (text, html)
}

fn main() {
    let file_path = "/app/data/jk6p1jpnetqnq0v0vd8vba06bath4imgtk091bg1.eml";
    //let file_path = "/app/data/165370460.eml";
    let message = parse_eml_from_filepath(file_path);

    //let output =
    let details = get_details(&message);
    let json_file_path = "/app/data/addresses2.json";
    fs::write(json_file_path, details).expect("Unable to write file");

    let (message_txt, message_html_content) = get_body(&message);

    fs::write("/app/data/message.txt", message_txt.clone()).expect("Unable to write file");
    fs::write("/app/data/message.html", message_html_content.clone()).expect("Unable to write file");
    
    println!("Text Body: {message_txt}");
    println!("HTML Body: {message_html_content}");
}
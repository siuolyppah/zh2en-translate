use std::{
    future::Future,
    io::{stdin, stdout, Write},
};

use chrono::Local;
use rmpv::Value;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

const LOG_FILE: &str = "/tmp/zh2en-rs.log";

async fn append_to_log_file(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .await
        .expect(&format!("open {} fail", LOG_FILE));

    let local_time = Local::now();
    let human_readable_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    file.write_all(format!("[{}]{}\n", human_readable_time, msg).as_bytes())
        .await
        .expect(&format!("write to file \"{}\" fail", LOG_FILE));
}

pub async fn start_request_handle_loop<StringFuture>(map: fn(String) -> StringFuture)
where
    StringFuture: Future<Output = String>,
{
    append_to_log_file("request handle loop start").await;

    loop {
        let mut req = match rmpv::decode::read_value(&mut stdin().lock()).unwrap() {
            Value::Array(input) => input,
            _ => unreachable!(),
        };

        append_to_log_file(&format!("translate input: {:?}", req)).await;

        let input = req[2].to_string();

        req[0] = Value::Integer(1.into());
        req[2] = Value::Nil;
        req[3] = Value::String(map(input).await.into());

        let to_write = Value::Array(req);
        rmpv::encode::write_value(&mut stdout().lock(), &to_write).unwrap();
        stdout().flush().unwrap();

        append_to_log_file(&format!("translate output: {}", to_write)).await;
    }
}

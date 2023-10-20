use std::{
    fs::OpenOptions,
    io::{stdin, stdout, Write},
};

use chrono::Local;
use rmpv::Value;

fn append_to_log_file(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/zh2en-rs.log")
        .unwrap();

    let local_time = Local::now();
    let human_readable_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    writeln!(file, "[{}]{}", human_readable_time, msg).unwrap();
}

pub fn start_request_handle_loop(map: fn(String) -> String) {
    append_to_log_file("maspack-test start");

    loop {
        let mut req = match rmpv::decode::read_value(&mut stdin().lock()).unwrap() {
            Value::Array(input) => input,
            _ => unreachable!(),
        };

        append_to_log_file(&format!("read: {:?}", req));

        let input = req[2].to_string();

        req[0] = Value::Integer(1.into());
        req[2] = Value::Nil;
        req[3] = Value::String(map(input).into());

        let to_write = Value::Array(req);
        rmpv::encode::write_value(&mut stdout().lock(), &to_write).unwrap();
        stdout().flush().unwrap();

        append_to_log_file(&format!("write: {}", to_write));
    }
}

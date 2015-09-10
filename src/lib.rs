#![feature(convert)]

fn repeat_str(string: &str, times: u64) -> String {
    let mut result = "".to_string();
    for _ in 0..times {
        result.push_str(string)
    }
    result
}
pub fn message_box(message: &str, width: u64, wordwrap: bool, _: bool) -> String {
    let mut lines: Vec<&str> = Vec::new();
    let mut real_width = width;
    if !wordwrap {
        let mut max_len: u64 = 0;
        for raw_line in message.split("\n") {
            let line = raw_line.trim_right();
            max_len = std::cmp::max(max_len, line.len() as u64);
            lines.push(line);
        }
        real_width = max_len + 4;
    }
    let mut result = format!(" {}\n", repeat_str("_", real_width - 2));
    for i in 0..lines.len() {
        if lines.len() == 1 {
            result.push_str(format!("< {}{} >\n", lines[i], repeat_str(" ", real_width - 4 - lines[i].len() as u64)).as_str());
        }
        else {
            if i == 0 {
                result.push_str(format!("/ {}{} \\\n", lines[i], repeat_str(" ", real_width - 4 - lines[i].len() as u64)).as_str());
            }
            else if i == lines.len() - 1 {
                result.push_str(format!("\\ {}{} /\n", lines[i], repeat_str(" ", real_width - 4 - lines[i].len() as u64)).as_str());
            }
            else {
                result.push_str(format!("| {}{} |\n", lines[i], repeat_str(" ", real_width - 4 - lines[i].len() as u64)).as_str());
            }
        }
    }
    result.push_str(format!(" {}", repeat_str("-", real_width - 2)).as_str());
    result
}

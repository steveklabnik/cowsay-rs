#![feature(convert)]

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
    let mut result = format!(" {}\n", std::iter::repeat("_").take((real_width - 2) as usize).collect::<String>());
    for i in 0..lines.len() {
        if lines.len() == 1 {
            result.push_str(format!("< {}{} >\n", lines[i], std::iter::repeat(" ").take((real_width - 4 - lines[i].len() as u64) as usize).collect::<String>()).as_str());
        }
        else {
            if i == 0 {
                result.push_str(format!("/ {}{} \\\n", lines[i], std::iter::repeat(" ").take((real_width - 4 - lines[i].len() as u64) as usize).collect::<String>()).as_str());
            }
            else if i == lines.len() - 1 {
                result.push_str(format!("\\ {}{} /\n", lines[i], std::iter::repeat(" ").take((real_width - 4 - lines[i].len() as u64) as usize).collect::<String>()).as_str());
            }
            else {
                result.push_str(format!("| {}{} |\n", lines[i], std::iter::repeat(" ").take((real_width - 4 - lines[i].len() as u64) as usize).collect::<String>()).as_str());
            }
        }
    }
    result.push_str(&format!(" {}", std::iter::repeat("-").take((real_width - 2) as usize).collect::<String>().as_str()));
    result
}

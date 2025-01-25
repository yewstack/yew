use std::io::Write;

use anyhow::Result;

use crate::log_line::LogLine;

pub fn write_log_lines(log_lines: Vec<LogLine>) -> Result<Vec<u8>> {
    let mut logs_list = Vec::default();
    for LogLine {
        message,
        user,
        issue_id,
        user_id,
        ..
    } in log_lines
    {
        writeln!(
            logs_list,
            "- {message}. [[@{user}](https://github.com/{user_id}), [#{issue_id}](https://github.com/yewstack/yew/pull/{issue_id})]",
        )?;
    }
    Ok(logs_list)
}

use anyhow::Result;
use std::io::Write;

use crate::log_line::LogLine;

pub fn write_log_lines(log_lines: Vec<LogLine>) -> Result<Vec<u8>> {
    let mut logs_list = Vec::default();
    for LogLine {
        message,
        user,
        issue_id,
    } in log_lines
    {
        writeln!(
            logs_list,
                "- {message}. [[@{user}](https://github.com/{user}), [#{issue_id}](https://github.com/yewstack/yew/pull/{issue_id})]",
                message = message,
                user = user,
                issue_id = issue_id
            )?;
    }
    Ok(logs_list)
}

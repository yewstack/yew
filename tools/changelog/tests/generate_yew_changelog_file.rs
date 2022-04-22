use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::Result;
use changelog::new_version_level::NewVersionLevel;
use changelog::yew_package::YewPackage;
use changelog::Cli;
use chrono::Utc;

struct FileDeleteOnDrop;

impl Drop for FileDeleteOnDrop {
    fn drop(&mut self) {
        fs::remove_file("tests/test_changelog.md").unwrap();
    }
}

#[test]
fn generate_yew_changelog_file() -> Result<()> {
    // Setup
    let file_delete_on_drop = FileDeleteOnDrop;

    fs::copy("tests/test_base.md", "tests/test_changelog.md")?;

    // Run
    let cli_args = Cli {
        package: YewPackage::from_str("yew").unwrap(),
        new_version_level: NewVersionLevel::Minor,
        from: Some("abeb8bc3f1ffabc8a58bd9ba4430cd091a06335a".to_string()),
        to: "d8ec50150ed27e2835bb1def26d2371a8c2ab750".to_string(),
        changelog_path: "tests/test_changelog.md".to_string(),
        skip_file_write: false,
        skip_get_bump_version: true,
        token: None,
    };

    cli_args.run().unwrap();

    // Check
    let expected = File::open("tests/test_expected.md")?;
    let expected_reader_lines = BufReader::new(expected).lines();

    let after = File::open("tests/test_changelog.md")?;
    let after_reader_lines = BufReader::new(after).lines();

    let lines = expected_reader_lines.zip(after_reader_lines);

    for (i, (expected_line, after_line)) in lines.enumerate() {
        if i == 2 {
            // third line has dynamic things that may break the tests
            let expected_third_line = expected_line?.replace(
                "date_goes_here",
                Utc::now().format("%Y-%m-%d").to_string().as_str(),
            );
            assert_eq!(expected_third_line, after_line?);
        } else {
            assert_eq!(expected_line?, after_line?);
        }
    }

    drop(file_delete_on_drop);

    Ok(())
}

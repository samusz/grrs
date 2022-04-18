use std::error::Error;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) 
-> Result<(), Box<dyn Error>> {
    for line in content.lines() {
        if line.contains(&pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

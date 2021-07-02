/// Convenience macro that parses a line from a [`Command`]'s output.
/// Returns Ok(stdout) if successful, otherwise returns Err(stderr).
#[macro_export]
macro_rules! parse_output {
    ($output:expr) => {
        {
            let output = $output;

            if !output.status.success() {
                return Err(BuildahError::Buildah(parse_output_line(output.stderr)));
            }

            parse_output_line(output.stdout)
        }
    };
}

/// Converts a newline-terminated line of output into a String.
/// Example: b"abc\n" => "abc"
pub fn parse_output_line(bytes: Vec<u8>) -> String {
    let string = String::from_utf8(bytes).unwrap();
    string.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    // TODO: test ^^

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::io;
use textwrap::{wrap, Options};

fn format_input(input: &str) -> String {
    let input = input.trim();

    let wrapped = wrap(&input, Options::new(40))
        .iter()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    let mut output = vec![];

    let max_line_length = wrapped.iter().map(|s| s.len()).max().unwrap_or(0);

    let mut line = String::new();
    line.push_str(" ");
    for _ in 0..max_line_length + 2 {
        line.push_str("-");
    }
    output.push(line);

    match wrapped.len() {
        0 => return String::new(),
        1 => output.push(format!("< {} >", input)),
        _ => {
            // first line
            output.push(format!(
                "/ {}{} \\",
                wrapped[0],
                " ".repeat(max_line_length - wrapped[0].len()) // padding
            ));

            // middle lines
            for line in wrapped.iter().skip(1).take(wrapped.len() - 2) {
                output.push(format!(
                    "| {}{} |",
                    line,
                    " ".repeat(max_line_length - line.len()) // padding
                ));
            }

            // last line
            output.push(format!(
                "\\ {}{} /",
                wrapped[wrapped.len() - 1],
                " ".repeat(max_line_length - wrapped[wrapped.len() - 1].len()) // padding
            ));
        }
    }

    line = String::from(" ");
    for _ in 0..max_line_length + 2 {
        line.push_str("-");
    }
    output.push(line);

    output.join("\n")
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let bubble = format_input(&buffer);

    /*
       \
        \   .~-~.
          /^     ^\
         (__ O O __)
          \ \< >/ /
    */
    let ferris = "       \\
        \\   .~-~.
          /^     ^\\
         (__ O O __)
          \\ \\< >/ /";

    println!("{}", bubble);
    println!("{}", ferris);

    Ok(())
}

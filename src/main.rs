fn main() {
    let mut string = String::from("MATMEX LUCHSHE VSEH");
    println!("{}", process(&mut string));
}

fn process(text: &mut String) -> &str {
    let start = text
        .find(|c: char| !c.is_whitespace())
        .unwrap_or(text.len());
    let end = match text[start..].find(char::is_whitespace) {
        Some(offset) => start + offset,
        None => text.len(),
    };
    text.push('!');
    &text[start..end]
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;

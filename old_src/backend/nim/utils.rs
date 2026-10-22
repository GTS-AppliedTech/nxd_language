pub fn ident(name: &str) -> String {
    name.to_lowercase()
}

pub fn indent_block(block: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    block
        .lines()
        .map(|line| format!("{}{}\n", pad, line.trim_end()))
        .collect()
}

pub fn join_lines(lines: Vec<String>) -> String {
    lines.join("")
}

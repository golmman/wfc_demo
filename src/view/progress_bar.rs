const CURSOR_UP_LEFT: &'static str = "\x1b[1F";
const ERASE_TO_EOL: &'static str = "\x1b[0K";

pub fn start_progress_bar() {
    println!();
}

pub fn print_progress_bar(percent: usize) {
    let p = percent / 2;
    let full = "#".repeat(p);
    let empty = " ".repeat(50 - p);
    println!("{}[{}{}] - {}%", CURSOR_UP_LEFT, full, empty, percent,);
}

pub fn end_progress_bar() {
    println!("{}{}{}", CURSOR_UP_LEFT, ERASE_TO_EOL, CURSOR_UP_LEFT);
}

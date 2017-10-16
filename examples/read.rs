extern crate mouse_readline;

fn main() {
    let result = mouse_readline::read_line("abrr>").unwrap();
    println!("{}", result);
}

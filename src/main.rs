fn main() {
    if let Err(e) = macos_widgets::load_and_print() {
        panic!("{:?}", e)
    }
}

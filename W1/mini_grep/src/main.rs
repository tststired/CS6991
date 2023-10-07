fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;

    if pattern.contains(&pattern_string) {
       println!("The command-line argument is: {pattern}");
    }
}

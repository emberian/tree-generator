fn main() {
    let args = std::os::args();
    let width: int = from_str(args[1].as_slice().trim()).unwrap();
    let trunk = args[2].as_slice().char_at(0);
    let leaf = args[3].as_slice().char_at(0);

    for padding in std::iter::range_inclusive(0, (width - 1) / 2).rev() {
        for _ in range(0i, padding) {
            print!(" ");
        }
        for _ in range(0i, width - (padding * 2)) {
            print!("{}", leaf);
        }
        println!("");
    }

    for padding in range(0i, (width - 3) / 2) {
        print!(" ");
    }

    for _ in range(0i, 3) {
        print!("{}", trunk);
    }

    println!("");
}

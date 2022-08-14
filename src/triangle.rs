pub fn print_triangle(size: u64) {
    for y in 0..size {
        for x in 0..(size*2)-1 {
            if (x >= y && y + x < size*2-1) && (y % 2 == x % 2) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
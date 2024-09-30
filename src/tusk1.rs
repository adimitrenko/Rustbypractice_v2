
pub fn run() {

    const W: u32 = 25;
    const H: u32 = 10;

    for y in 0..H {
        for x in 0..W {

            let is_horisontal = y == 0 || y == H - 1;
            let is_vertical = x == 0 || x == W - 1;

            let c = if is_horisontal || is_vertical {
                "*"
            } else {
                " "
            };
            print!("{}", c );
        }
        println!();
    }
}

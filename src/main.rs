const WIDTH: usize = 64;
const HEIGHT: usize = 48;

fn plot_pixel(buffer: &mut [u8; WIDTH * HEIGHT], x: usize, y: usize, color: u8) {
    buffer[x + y * WIDTH] = color;
}

fn ppm_toString(buffer: &[u8; WIDTH * HEIGHT]) -> String {
    let mut out_data = String::from("P3\n");
    out_data += &format!("{} {}\n", WIDTH, HEIGHT).to_string();
    out_data += &String::from("255\n");
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            out_data += &format!("{} ", buffer[col + col * row]).to_string();
        }
        out_data += &String::from("\n");
    }
    out_data
}

fn main() {
    let mut bitplane: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    plot_pixel(&mut bitplane, 1 as usize, 2 as usize, 128 as u8);
    println!("Hello, world {}!", ppm_toString(&bitplane));
}

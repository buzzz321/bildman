use std::f32::consts::PI;
use std::net::UdpSocket;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn plot_pixel(buffer: &mut [u8; WIDTH * HEIGHT], x: usize, y: usize, color: u8) {
    buffer[x + y * WIDTH] = color;
}

fn ppm_to_string(buffer: &[u8; WIDTH * HEIGHT]) -> String {
    let mut out_data = String::from("P3\n");
    out_data += &format!("{} {}\n", WIDTH, HEIGHT).to_string();
    out_data += &String::from("255\n");
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            out_data += &format!("{} {} {}\n", buffer[col + WIDTH * row], "0", "0").to_string();
        }
    }
    out_data
}

fn send_to_pc(socket: &mut UdpSocket, buffer: &[u8; WIDTH * HEIGHT]) {
    match socket.send(buffer) {
        Ok(_) => (),
        Err(e) => panic!("encountered IO error: {e}"),
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    socket.set_nonblocking(true).unwrap();

    let mut bitplane: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    for degree in 0..360 {
        let x = 15.0 * f32::sin((degree as f32 / 360.0) * 2.0 * PI) + WIDTH as f32 / 2.0;
        let y = 15.0 * f32::cos((degree as f32 / 360.0) * 2.0 * PI) + HEIGHT as f32 / 2.0;
        plot_pixel(&mut bitplane, x as usize, y as usize, 255 as u8);
    }
    println!("{}", ppm_to_string(&bitplane));
    Ok(())
}

use std::f32::consts::PI;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::time::{Duration, Instant};
use std::{thread, time};

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

fn send_to_pc(socket: &UdpSocket, buffer: &[u8; WIDTH * HEIGHT]) {
    socket
        .send(&[1, 2, 3, 4, 5])
        .expect("Not possible to send preamble");
    let ret_val = buffer
        .chunks(1024)
        .for_each(|chunk| match socket.send(chunk) {
            Ok(_) => (),
            Err(e) => panic!("encountered IO error: {e}"),
        });
    //socket
    //    .send(&[5, 4, 3, 2, 1])
    //    .expect("Not possible to send postamble");
    ret_val
}

fn circle(bitplane: &mut [u8; WIDTH * HEIGHT], size: f32, offset_x: f32, offset_y: f32) {
    for degree in 0..360 {
        let x = offset_x + size * f32::cos((degree as f32 / 360.0) * 2.0 * PI);
        let y = offset_y + size * f32::sin((degree as f32 / 360.0) * 2.0 * PI);
        plot_pixel(bitplane, x as usize, y as usize, 255_u8);
    }
}

fn main() -> std::io::Result<()> {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0));
    let socket = UdpSocket::bind(addr)?;
    socket.connect("127.0.0.1:34254")?;
    socket.set_nonblocking(true).unwrap();

    let mut bitplane: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    //println!("{}", ppm_to_string(&bitplane));
    let delay_millis = time::Duration::from_millis(5);
    let SIZE: f32 = 15.0;
    let mut offx: f32 = SIZE; //WIDTH as f32 / 2.0;
    let mut offy: f32 = HEIGHT as f32 / 2.0;
    let mut xdir = 1.0;
    loop {
        bitplane.iter_mut().for_each(|x| *x = 0);

        circle(&mut bitplane, SIZE, offx, offy);
        //plot_pixel(&mut bitplane, offx as usize, offy as usize, 255_u8);
        let start = Instant::now();
        send_to_pc(&socket, &bitplane);
        let stop = Instant::now();
        if stop - start > Duration::from_secs(10) {
            break;
        }
        thread::sleep(delay_millis);
        offx += xdir;
        offy += 0.0;

        if offx >= WIDTH as f32 - SIZE || offx <= SIZE {
            //offx = SIZE + 2.0;
            xdir = -xdir;
        }
    }
    Ok(())
}

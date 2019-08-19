mod ppm;
use ppm::PPM;

fn main() {
    let img = PPM::New(10, 10);
    println!("{}", img.to_string());
}

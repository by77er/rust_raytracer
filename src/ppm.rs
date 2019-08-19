#![allow(dead_code)]

// Represents one pixel within a PPM file, rgb
#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}
impl Pixel {
    pub fn to_string(&self) -> String {
        format!("{} {} {}", &self.r, &self.g, &self.b)
    }
}
#[macro_export]
macro_rules! pxl {
    ($x:expr, $y:expr,  $z:expr) => {
        Pixel {
            r: $x,
            g: $y,
            b: $z
        }
    };
}
// ---

// Represents the contents of a PPM file
pub struct PPM {
    width: u16,
    height: u16,
    data: Vec<Vec<Pixel>>
}
impl PPM {
    pub fn new(width: u16, height: u16) -> PPM {
        PPM {
            width: width,
            height: height,
            data: vec![vec![Pixel { r: 255, g: 255, b: 255};height as usize]; width as usize]
        }
    }
    pub fn get_dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }
    pub fn to_string(&self) -> String {
        let mut s: String;
        s = format!("P3\n{} {}\n255\n", self.width, self.height);
        for x in 0..self.height {
            for y in 0..self.width {
                s.push_str(self.data[y as usize][x as usize].to_string().as_str());
                s.push_str("\n");
            }
        }
        s.pop();
        s
    }
    pub fn set(&mut self, x: u16, y: u16, pixel: Pixel) -> Result<Pixel, String> {
        if (x < self.width) && (y < self.height) {
            let old_pix = self.data[x as usize][y as usize];
            self.data[x as usize][y as usize] = pixel;
            Ok(old_pix)
        } else {
            Err(
                format!("set({}, {}) out of bounds for {}x{} image",
                x, y, self.width, self.height)
            )
        }
    }
}
// ---
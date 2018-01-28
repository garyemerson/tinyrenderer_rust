extern crate image;

mod img;
use img::Img;

fn main() {
    let mut img = Img::new(100, 100);

    for _ in 0..1000000 {
        // line(13, 20, 80, 40, &mut img, (255, 255, 255));

        // line2(13, 20, 80, 40, &mut img, (255, 255, 255));

        // line2(13, 20, 80, 40, &mut img, (255, 255, 255)); 
        // line2(20, 13, 40, 80, &mut img, (255, 0, 0)); 
        // line2(80, 40, 13, 20, &mut img, (255, 0, 0));

        // line3(50, 50, 50, 60, &mut img, (255, 255, 255));
        // line3(50, 50, 50, 40, &mut img, (255, 255, 255));
        // line3(50, 50, 60, 60, &mut img, (255, 255, 255));
        // line3(50, 50, 60, 50, &mut img, (255, 255, 255));
        // line3(50, 50, 40, 50, &mut img, (255, 255, 255));
        // line3(50, 50, 60, 40, &mut img, (255, 255, 255));
        // line3(50, 50, 40, 60, &mut img, (255, 255, 255));
        // line3(50, 50, 40, 40, &mut img, (255, 255, 255));

        // line3(13, 20, 80, 40, &mut img, (255, 255, 255)); 
        // line3(20, 13, 40, 80, &mut img, (255, 0, 0)); 
        // line3(80, 40, 13, 20, &mut img, (255, 0, 0));

        line4(13, 20, 80, 40, &mut img, (255, 255, 255)); 
        line4(20, 13, 40, 80, &mut img, (255, 0, 0)); 
        line4(80, 40, 13, 20, &mut img, (255, 0, 0));
    }

    // img.flip_vertical();
    img.save("output.png");
}

fn line4(mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, img: &mut Img, color: (u8, u8, u8)) {
    let mut steep = false; 
    if ((x0 as i32) - (x1 as i32)).abs() < ((y0 as i32) - (y1 as i32)).abs() { // if the line is steep, we transpose the image 
        let mut temp = x0;
        x0 = y0;
        y0 = temp;

        temp = x1;
        x1 = y1;
        y1 = temp;

        steep = true;
    } 

    if x0 > x1 {
        let mut temp = x1;
        x1 = x0;
        x0 = temp;

        temp = y1;
        y1 = y0;
        y0 = temp;
    }

    let dx = ((x0 as i32) - (x1 as i32)).abs();
    let dy = ((y0 as i32) - (y1 as i32)).abs();
    // let dydx: f64 = (dy as f64) / (dx as f64);
    // let mut error = 0.0;
    let mut y = y0;
    let mut error2: i32 = 0;

    for x in x0..(x1 + 1) {
        if steep {img.put(y as u32, x as u32, color); } else {img.put(x as u32, y as u32, color);}

        // error += dydx;
        // error2 = error * (dx as f64);
        if 2 * (error2 + dy) < dx {
            // error = error + dydx;
            error2 = error2 + dy;
        } else {
            y += 1;
            // error * (dx as f64)
            error2 = error2 + dy - dx;
        }
    }
}

fn line3(mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, img: &mut Img, color: (u8, u8, u8)) {
    let mut steep = false; 
    if ((x0 as i32) - (x1 as i32)).abs() < ((y0 as i32) - (y1 as i32)).abs() { // if the line is steep, we transpose the image 
        let mut temp = x0;
        x0 = y0;
        y0 = temp;

        temp = x1;
        x1 = y1;
        y1 = temp;

        steep = true;
    } 

    if x0 > x1 {
        let mut temp = x1;
        x1 = x0;
        x0 = temp;

        temp = y1;
        y1 = y0;
        y0 = temp;
    }

    let dx = ((x0 as i32) - (x1 as i32)).abs();
    let dy = ((y0 as i32) - (y1 as i32)).abs();
    let derror: f64 = (dy as f64) / (dx as f64);
    let mut error = 0.0;
    let mut y = y0;

    for x in x0..(x1 + 1) {
        if steep {
            img.put(y, x, color);
        } else {
            img.put(x, y, color);
        }

        error += derror;
        if error > 0.5 {
            y = if y1 > y0 { y + 1 } else { y - 1 };
            error -= 1.0;
        }
    }
}

fn line2(mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, img: &mut Img, color: (u8, u8, u8)) {
    let mut steep = false; 
    if ((x0 as i32) - (x1 as i32)).abs() < ((y0 as i32) - (y1 as i32)).abs() { // if the line is steep, we transpose the image 
        let mut temp = x0;
        x0 = y0;
        y0 = temp;

        temp = x1;
        x1 = y1;
        y1 = temp;

        steep = true;
    } 

    if x0 > x1 {
        let mut temp = x1;
        x1 = x0;
        x0 = temp;

        temp = y1;
        y1 = y0;
        y0 = temp;
    }

    for x in x0..(x1 + 1) {
        let t = ((x - x0) as f64) / ((x1 - x0) as f64);
        let y = (y0 as f64) * (1f64 - t) + (y1 as f64) * t;

        if steep {
            img.put(y.round() as u32, x, color);
        } else {
            img.put(x, y.round() as u32, color);
        }
    }
}

fn line(x0: u32, y0: u32, x1: u32, y1: u32, img: &mut Img, color: (u8, u8, u8)) {
    for step in 0..100 {
        let t = (step as f64) / 100f64;
        let x = (x0 as f64) * (1f64 - t) + (x1 as f64) * t;
        let y = (y0 as f64) * (1f64 - t) + (y1 as f64) * t;
        img.put(x.round() as u32, y.round() as u32, color);
    }
}

mod img;
use img::Img;

fn main() {
    let mut img = Img::new(800, 600);
    for x in 0..800 {
        for y in 0..600 {
            img.put(x, y, (0, 146, 90));
        }
    }
    img.save("output.png");
}

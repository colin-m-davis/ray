// https://raytracing.github.io/books/RayTracingInOneWeekend.html

mod vec3;
mod color;
mod ray;

fn main() {
    
    // Image

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render

    println!("P3\n{} {}\n255\n", IMAGE_HEIGHT, IMAGE_WIDTH);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = vec3::Color {
                x: (i as f64) / ((IMAGE_WIDTH-1) as f64),
                y: (j as f64) / ((IMAGE_HEIGHT-1) as f64),
                z: 0.25
            };
            color::write_color(pixel_color);
        }
    }
}

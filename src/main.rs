use rand::{SeedableRng, Rng};
use plotly::{Plot, Scatter, common::Mode};

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

struct Centroid {
    x: f32,
    y: f32,
    x_sum: f32,
    n: u32,
}

impl Centroid {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y, x_sum: 0f32, n: 0u32 }
    }

    fn euclidean_distance(&self, a: &Point) -> f32 {
        ((self.x - a.x).powf(2f32) + (self.y - a.y).powf(2f32)).sqrt()
    }
}

fn generate_random_coordinates(n: u32) -> Vec<Point> {
    let mut coords: Vec<Point> = Vec::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(10);

    for _ in 0..n {
        let x: f32 = rng.gen_range(1..=100) as f32;
        let y: f32 = rng.gen_range(1..=100) as f32;
        coords.push(Point::new(x, y))
    }

    coords
}

fn graph(points: &[Point]) {
    let mut plot = Plot::new();
    let trace = Scatter::new(points.iter().map(|p| p.x).collect(), points.iter().map(|p| p.y).collect()).mode(Mode::Markers);
    plot.add_trace(trace);
    plot.write_html("out.html");
}

fn main() {
    let sample = generate_random_coordinates(50);
    // for (i, x) in sample.iter().enumerate() {
    //     println!("Point {}: ({}, {})", i + 1, x.x, x.y);
    // }
    let point = Centroid::new(50f32, 50f32);
    let distance = point.euclidean_distance(&sample[1]);
    println!("Centroid: ({}, {}) | Point: ({}, {}) | Distance: {}", point.x, point.y, sample[1].x, sample[1].y, distance);
}

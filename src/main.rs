use rand::{Rng, SeedableRng};
use tools::{final_graph, graph, Point};
use std::env;
use colored::Colorize;

// TODO: Make a better final report

const N_POINTS: usize = 50;
const N_CENTROIDS: usize = 5;
const MAX_LIMIT: usize = 500;

#[derive(Debug)]
struct Centroid {
    current: Point,
    x_sum: f32,
    y_sum: f32,
    n: u32,
    previous: Point,
    cluster: Vec<Point>,
}

impl Centroid {
    fn new(x: f32, y: f32) -> Self {
        Self {
            current: Point::new(x, y),
            x_sum: 0f32,
            y_sum: 0f32,
            n: 0u32,
            previous: Point::new(0f32, 0f32),
            cluster: Vec::new(),
        }
    }

    fn new_random() -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(10);
        let x = rng.gen_range(1..=MAX_LIMIT) as f32;
        let y = rng.gen_range(0..=MAX_LIMIT) as f32;
        Centroid::new(x, y)
    }

    fn euclidean_distance(&self, a: &Point) -> f32 {
        ((self.current.x - a.x).powf(2f32) + (self.current.y - a.y).powf(2f32)).sqrt()
    }

    fn add_point(&mut self, x: &f32, y: &f32) {
        self.x_sum += *x;
        self.y_sum += *y;
        self.n += 1;
    }

    fn update_centroid(&mut self) {
        if self.n > 0 {
            self.previous.update(&self.current.x, &self.current.y);
            self.current
                .update(&(self.x_sum / self.n as f32), &(self.y_sum / self.n as f32));
            self.x_sum = 0f32;
            self.y_sum = 0f32;
            self.n = 0;
        }
    }

    fn compare_to_previous(&self) -> bool {
        let delta_x = (self.previous.x - self.current.x).abs();
        let delta_y = (self.previous.y - self.current.y).abs();

        delta_x < 0.001 && delta_y < 0.001
    }
}

fn generate_random_coordinates(n: u32) -> Vec<Point> {
    let mut coords: Vec<Point> = Vec::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(10);

    for _ in 0..n {
        let x: f32 = rng.gen_range(1..=MAX_LIMIT) as f32;
        let y: f32 = rng.gen_range(1..=MAX_LIMIT) as f32;
        coords.push(Point::new(x, y))
    }

    coords
}

fn min_distance(centroids: &[Centroid], point: &Point) -> usize {
    let mut index_of_min = 0usize;
    let mut min_distance = f32::MAX;

    for (i, c) in centroids.iter().enumerate() {
        let d = c.euclidean_distance(point);
        if d < min_distance {
            min_distance = d;
            index_of_min = i;
        }
    }

    index_of_min
}

fn k_mean_clustering(centroids: &mut [Centroid], data: &[Point]) -> u32 {
    let mut iterations = 0;

    loop {
        iterations += 1;

        for point in data {
            let index = min_distance(centroids, point);
            centroids[index].add_point(&point.x, &point.y);
        }

        centroids.iter_mut().for_each(|c| c.update_centroid());

        let mut check = true;
        for c in centroids.iter() {
            if !c.compare_to_previous() {
                check = false;
            }
        }

        if check {
            break;
        }
    }

    for point in data {
        let index = min_distance(centroids, point);
        centroids[index].cluster.push(point.clone());
    }

    iterations
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for s in &args {
        println!("-> {s}");
    }

    let sample = generate_random_coordinates(N_POINTS as u32);
    let mut centroids: Vec<Centroid> = Vec::with_capacity(N_CENTROIDS);
    for _ in 0..N_CENTROIDS {
        centroids.push(Centroid::new_random())
    }

    graph(&sample);

    let n = k_mean_clustering(&mut centroids, &sample);
    let mut clusters: Vec<Vec<Point>> = vec![Vec::new(); N_CENTROIDS];

    for (i, c) in centroids.iter_mut().enumerate() {
        clusters[i].push(c.current.clone());
        for p in &c.cluster {
            clusters[i].push(p.clone());
        }
    }

    final_graph(&clusters);

    println!("{:-<38}", "");
    println!("|  {:<32}  |", "FINAL REPORT".blue().bold());
    println!("|{:-<36}|", "");
    println!("|  {:<32}  |", format!("Number of points: {:>14}", N_POINTS.to_string().bold()));
    println!("|  {:<32}  |", format!("Number of centroids: {:>11}", N_CENTROIDS.to_string().bold()));
    println!("|  {:<32}  |", format!("Number of iterations: {:>10}", n.to_string().bold()));
    println!("|  {:<32}  |", "Range of points:");
    println!("|  {:<32}  |", format!("  Minimum: {:>21}", 0.to_string().bold()));
    println!("|  {:<32}  |", format!("  Maximum: {:>21}", MAX_LIMIT.to_string().bold()));
    println!("|  {:<32}  |", "Centroids:");
    for (i, c) in centroids.iter().enumerate() {
        println!("|  {:<32}  |", format!("  Centroid {}: {:>18}", i + 1, format!("({:.2}, {:.2})", c.current.x, c.current.y).bold()));
    }
    println!("{:-<38}", "");
}

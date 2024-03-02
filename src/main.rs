use rand::{SeedableRng, Rng};
use tools::{graph, final_graph, Point};

// TODO: Take input from the user
// TODO: Remove redundancy in the code
// TODO: Make a better final report
// TODO: Centroid in lib.rs?

const N_CENTROIDS: usize = 3;
const MAX_LIMIT: usize = 500;

#[derive(Debug)]
struct Centroid {
    current: Point,
    x_sum: f32,
    y_sum: f32,
    n: u32,
    previous: Point,
    cluster: Vec<Point>
}

impl Centroid {
    fn new(x: f32, y: f32) -> Self {
        Self { current: Point::new(x, y), x_sum: 0f32, y_sum: 0f32, n: 0u32, previous: Point::new(0f32, 0f32), cluster: Vec::new() }
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
            self.current.update(&(self.x_sum / self.n as f32), &(self.y_sum / self.n as f32));
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


fn k_mean_clustering(centroids: &mut [Centroid], data: &[Point]) -> u32 {
    let mut iterations = 0;

    loop {
        iterations += 1;

        for point in data {
            let mut index_of_min = 0usize;
            let mut min_distance = f32::MAX;

            for (i, c) in centroids.iter_mut().enumerate() {
                let d = c.euclidean_distance(point);
                if d < min_distance {
                    min_distance = d;
                    index_of_min = i;
                }
            }

            centroids[index_of_min].add_point(&point.x, &point.y);
        }

        for c in centroids.iter_mut() {
            c.update_centroid();
        }

        let mut check = true;
        for c in centroids.iter() {
            if !c.compare_to_previous() {
                check = false;
            }
        }

        if check {
            for point in data {
                let mut index_of_min = 0usize;
                let mut min_distance = f32::MAX;

                for (i, c) in centroids.iter_mut().enumerate() {
                    let d = c.euclidean_distance(point);
                    if d < min_distance {
                        min_distance = d;
                        index_of_min = i;
                    }
                }

                centroids[index_of_min].cluster.push(point.clone());
            }
            break;
        }
    }

    iterations
}

fn main() {
    let sample = generate_random_coordinates(50);
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

    // for c in centroids {
    //     println!("{:?}", c);
    // }

    println!("Number of iterations: {}", n);
}

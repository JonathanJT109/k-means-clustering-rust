use rand::{SeedableRng, Rng, distributions};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn update(&mut self, x: &f32, y: &f32) {
        self.x = *x;
        self.y = *y;
    }
}

#[derive(Debug, Copy, Clone)]
struct Centroid {
    current: Point,
    x_sum: f32,
    y_sum: f32,
    n: u32,
    previous: Point,
}

impl Centroid {
    fn new(x: f32, y: f32) -> Self {
        Self { current: Point::new(x, y), x_sum: 0f32, y_sum: 0f32, n: 0u32, previous: Point::new(0f32, 0f32) }
    }

    fn new_random() -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(10);
        let x = rng.gen_range(1..=100) as f32;
        let y = rng.gen_range(0..=100) as f32;
        Self { current: Point::new(x, y), x_sum: 0f32, y_sum: 0f32, n: 0u32, previous: Point::new(0f32, 0f32) }
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
        let delta_y = (self.previous.y - self.current.x).abs();

        (delta_x < 0.0001 && delta_y < 0.0001)
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
            break;
        }
    }

    iterations
}

fn main() {
    let sample = generate_random_coordinates(50);
    // for (i, x) in sample.iter().enumerate() {
    //     println!("Point {}: ({}, {})", i + 1, x.x, x.y);
    // }
    let mut centroids: Vec<Centroid> = Vec::new();
    for _ in 0..3 {
        centroids.push(Centroid::new_random())
    }
    // let distance = point.euclidean_distance(&sample[1]);
    // println!("Centroid: ({}, {}) | Point: ({}, {}) | Distance: {}", point.x, point.y, sample[1].x, sample[1].y, distance);
    let n = k_mean_clustering(&mut centroids, &sample);

    // println!("Centroid: {:?}", point);
    // point.update_centroid();
    // println!("New Centroid: {:#?}", point);
    println!("Number of iterations: {}", n);
    for c in centroids {
        println!("{:?}", c);
    }
}

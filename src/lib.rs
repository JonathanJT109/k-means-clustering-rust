use plotly::{Plot, Scatter, common::{Mode, Marker}};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn update(&mut self, x: &f32, y: &f32) {
        self.x = *x;
        self.y = *y;
    }
}

pub fn graph(points: &[Point]) {
    let mut plot = Plot::new();
    let trace = Scatter::new(points.iter().map(|p| p.x).collect(), points.iter().map(|p| p.y).collect()).mode(Mode::Markers);
    plot.add_trace(trace);
    plot.write_html("sample.html");
}

pub fn final_graph(clusters: &[Vec<Point>]) {
    let mut plot = Plot::new();
    for (i, cluster) in clusters.iter().enumerate() {
        let cluster_name = format!("Cluster {}", i + 1);
        let centroid_name = format!("Centroid {}", i + 1);
        let trace = Scatter::new(cluster[1..].iter().map(|p| p.x).collect(), cluster[1..].iter().map(|p| p.y).collect()).mode(Mode::Markers).name(cluster_name);
        let centroids = Scatter::new(vec![cluster[0].x], vec![cluster[0].y]).mode(Mode::Markers).marker(Marker::new().size_array(vec![15])).name(centroid_name);
        plot.add_trace(trace);
        plot.add_trace(centroids);
    }
    plot.write_html("clusters.html");
}
use plotly::{Plot, Scatter, common::Mode};

struct Point {
    x: f32,
    y: f32,
}
pub fn graph(points: &[Point]) {
    let mut plot = Plot::new();
    let trace = Scatter::new(points.iter().map(|p| p.x).collect(), points.iter().map(|p| p.y).collect()).mode(Mode::Markers);
    plot.add_trace(trace);
    plot.write_html("out.html");
}
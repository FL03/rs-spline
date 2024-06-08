/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rs_spline as rsp;

use anyhow::Result;
use num::traits::{FromPrimitive, Num};
use rsp::BSpline;

fn main() -> Result<()> {
    // Example control points and knot vector
    let points = vec![-1.0, 2.0, 0.0, -1.0];
    let knots = linspace::<f64>(0f64, 6f64, 7);
    let spline = BSpline::new(points, knots)?; // initialize the b-spline
    println!("{:#?}", spline.shape()); // print the shape of the spline
                                       // Sample the spline at a few points
    assert_eq!(dbg!(spline.eval(2.0)), 0.5);
    assert_eq!(dbg!(spline.eval(2.5)), 1.375);

    let samples = linspace(0.0, 5.0, 50);
    assert_eq!(samples.len(), 50);
    for i in [1.1, 1.9] {
        let _ = dbg!(spline.eval(i));
    }

    let a = spline.eval_iter(samples);
    plot_spline("b-spline", a)?;
    Ok(())
}

fn plot_spline(filename: &str, spline: impl IntoIterator<Item = (f64, f64)>) -> Result<()> {
    use plotters::prelude::*;
    let fpath = format!(".artifacts/plots/{filename}.png");
    let root = BitMapBackend::new(&fpath, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("B-Spline", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f64..5f64, -1.5f64..1.5f64)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(10)
        .y_labels(10)
        // We can also change the format of the label text
        .x_label_formatter(&|xi| format!("{:.2}", xi))
        .y_label_formatter(&|yi| format!("{:.2}", yi))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(spline, &BLUE))?;
    root.present()?;
    Ok(())
}

pub fn pad_vec<T: Clone>(vec: &Vec<T>, len: usize) -> Vec<T> {
    let mut res = vec.clone();
    while res.len() < len {
        res.push(res[res.len() - 1].clone());
    }
    res
}

fn linvec<T>(len: usize) -> Vec<T>
where
    T: FromPrimitive,
{
    (0..len).map(|i| T::from_usize(i).unwrap()).collect()
}

fn linspace<T>(from: T, end: T, steps: usize) -> Vec<T>
where
    T: Copy + FromPrimitive + Num,
{
    let n = T::from_usize(steps - 1).unwrap();
    let step = (end - from) / n;
    (0..steps)
        .map(|i| from + step * T::from_usize(i).unwrap())
        .collect()
}

fn msg<T>(t: T, out: T)
where
    T: core::fmt::Debug,
{
    println!("spline({:?}) = {:#?}", t, out);
}

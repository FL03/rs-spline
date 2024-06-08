/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rs_spline as rsp;

use anyhow::Result;
use rsp::BSpline;

pub fn init_tracing() {
    use tracing::Level;
    use tracing_subscriber::EnvFilter;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(Level::DEBUG)
        .with_target(true)
        .without_time()
        .init();
    tracing::info!("Success: initialized the tracing modules...");
}

fn main() -> Result<()> {
    init_tracing();
    // Example control points and knot vector
    let points = vec![-1.0, 2.0, 0.0, -1.0];
    let knots = utils::linspace::<f64>(0f64, 6f64, 7);
    // initialize the b-spline
    let spline = BSpline::new(points, knots)?;
    // print the shape of the spline
    tracing::info!("Shape: {}", spline.shape());
    // Sample the spline at a known points
    assert_eq!(dbg!(spline.eval(2.0)), 0.5);
    assert_eq!(dbg!(spline.eval(2.5)), 1.375);

    let samples = utils::linspace(0.0, 5.0, 50);
    let res = spline.eval_iter(samples);
    tracing::info!("Some Results: {:?}", &res[20..23]);
    plot("b-spline", res)?;
    Ok(())
}

fn plot(filename: &str, spline: impl IntoIterator<Item = (f64, f64)>) -> Result<()> {
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
    tracing::info!("Saved plot to: {fpath}");
    Ok(())
}




#[allow(dead_code)]
mod utils {
    use num::traits::{FromPrimitive, Num, NumRef};

    pub fn linvec<T>(range: core::ops::Range<usize>) -> Vec<T>
    where
        T: FromPrimitive,
    {
        range.map(|i| T::from_usize(i).unwrap()).collect()
    }

    pub fn linspace<T>(from: T, end: T, steps: usize) -> Vec<T>
    where
        T: FromPrimitive + Num + NumRef,
    {
        let n = T::from_usize(steps - 1).unwrap();
        let step = (end - &from) / n;
        (0..steps)
            .map(|i| T::from_usize(i).unwrap() * &step + &from)
            .collect()
    }

    pub fn pad_vec<T: Clone>(vec: &Vec<T>, len: usize) -> Vec<T> {
        let mut res = vec.clone();
        while res.len() < len {
            res.push(res[res.len() - 1].clone());
        }
        res
    }

    pub trait Linspace<T> where T: FromPrimitive + Num + NumRef {
        type Output;

        fn linspace(from: T, end: T, steps: usize) -> Self::Output;
    }

    impl<T> Linspace<T> for Vec<T>
    where
        T: FromPrimitive + Num + NumRef,
    {
        type Output = Vec<T>;
        fn linspace(from: T, end: T, steps: usize) -> Self::Output {
            linspace(from, end, steps)
        }
    }
}

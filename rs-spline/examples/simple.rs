/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rs_spline as rsp;

use rsp::{BSpline, Result};

fn main() -> Result<()> {
    // Example control points and knot vector
    let points = vec![0.0, 1.0, 2.0, 3.0];
    let knots = vec![0.0, 0.0, 0.0, 0.1, 0.2, 0.9, 1.0];
    // initialize the B-spline
    let spline = BSpline::new(points, knots)?; // BSpline::new(degree, points, knots)?;
                                               // generate some data points which may be used to sample the spline
    let data = vec![0.0, 0.5, 1.0, 1.5, 2.9];
    // Evaluate the B-spline at different points
    let res = data
        .iter()
        .copied()
        .map(|t| (t, spline.spline(t)))
        .collect::<Vec<_>>();
    println!(
        "******* B-Spline *******\n\nResults ('t', 'spline(t)'): {:#?}",
        res
    );

    Ok(())
}

pub fn pad_vec<T: Clone>(vec: &Vec<T>, len: usize) -> Vec<T> {
    let mut res = vec.clone();
    while res.len() < len {
        res.push(res[res.len() - 1].clone());
    }
    res
}

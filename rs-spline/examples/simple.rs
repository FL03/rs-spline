/*
    Appellation: simple <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rs_spline as rsp;

use rsp::{BSpline, Result};


macro_rules! spline {
    ($spline:ident($x:expr)) => {
        {
            let tmp = $spline.eval($x);
            println!("spline({:?}) = {:#?}", $x, &tmp);
            tmp
        }
    };
}

fn main() -> Result<()> {
    // Example control points and knot vector
    let points = vec![-1.0, 2.0, 0.0, -1.0];
    let knots = linvec::<f64>(7);
    let spline = BSpline::new(points, knots)?; // initialize the b-spline
    println!("Degree: {:#?}", spline.degree());
    
    let _res = spline!(spline(2.5));

    Ok(())
}

pub fn pad_vec<T: Clone>(vec: &Vec<T>, len: usize) -> Vec<T> {
    let mut res = vec.clone();
    while res.len() < len {
        res.push(res[res.len() - 1].clone());
    }
    res
}

fn linvec<T>(len: usize) -> Vec<T> where T: num::traits::FromPrimitive {
    (0..len).map(|i| T::from_usize(i).unwrap()).collect()
}
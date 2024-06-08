/*
    Appellation: b-spline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rs_spline as rsp;

use rsp::BSpline;

#[test]
fn test_bspline() {
    let points = vec![-1.0, 2.0, 0.0, -1.0];
    let knots = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let spline = BSpline::new(points, knots).unwrap(); // initialize the b-spline

    assert_eq!(spline.degree(), 2);
    // Sample the spline at a few points
    assert_eq!(spline.eval(2.0), 0.5);
    assert_eq!(spline.eval(2.5), 1.375);
}

#[test]
fn test_bspline_iter() {
    // create the control points and knot vector
    let points = vec![-1.0, 2.0, 0.0, -1.0];
    let knots = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    // initialize the b-spline
    let spline = BSpline::new(points, knots).unwrap(); // initialize the b-spline
                                                       // create a list of sample values
    let data = [2.0, 2.5, 3.0, 3.5];
    // specify the expected results
    let exp = vec![0.5, 1.375, 1.0, 0.125];
    let exp_map = data
        .iter()
        .copied()
        .zip(exp.iter().copied())
        .collect::<Vec<_>>();
    // evaluate the spline at the sample points
    assert_eq!(spline.eval_iter(data), exp_map);
}

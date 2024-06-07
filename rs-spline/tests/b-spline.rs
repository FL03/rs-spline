/*
    Appellation: b-spline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rs_spline as rsp;

use rsp::BSpline;

#[test]
fn test_bspline() {
    let points = vec![-1.0, 2.0, 0.0, -1.0];
    let knots = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0,];
    let spline = BSpline::new(points, knots).unwrap(); // initialize the b-spline

    assert_eq!(spline.degree(), 2);
    assert_eq!(spline.eval(2.5), 1.375);

    let samples = vec![2.0, 2.5, 3.0, 3.5];
    let exp = vec![0.5, 1.375, 1.0, 0.125];
    assert_eq!(spline.spline_iter(samples), exp);
}
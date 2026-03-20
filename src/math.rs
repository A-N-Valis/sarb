fn mean(data: &[f64]) -> f64 {
    if data.is_empty() { return 0.0; }

    data.iter().sum::<f64>() / data.len() as f64
}

pub fn calculate_beta(x: &[f64], y: &[f64]) -> f64 {
    assert_eq!(x.len(), y.len(), "prices slices must have equal length");

    let mean_x = mean(x);
    let mean_y = mean(y);

    let (var_x, cov_xy) = x.iter().zip(y.iter())
        .fold((0.0_f64, 0.0_f64), |(vx, co), (&x, &y)|{
            (vx + (x - mean_x).powi(2), co + (x - mean_x) * (y - mean_y))
        });

    if var_x == 0.0 { return 0.0 }

    cov_xy / var_x
}

pub fn calculate_spread(x: &[f64], y: &[f64], beta: f64, out: &mut Vec<f64>) {
    assert_eq!(x.len(), y.len(), "prices slices must have equal length");

    out.clear();
    out.extend(x.iter().zip(y.iter()).map(|(x, y)| y - beta * x));
}
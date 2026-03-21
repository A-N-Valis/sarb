use std::f64::consts::LN_2;

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

pub fn calculate_half_life(spread: &[f64], delta_buf: &mut Vec<f64>) -> f64 {
    let n = spread.len();
    assert!(n >= 2, "spread must have atleast two elements");

    delta_buf.clear();
    delta_buf.extend(spread.windows(2).map(|w| w[1] - w[0]));

    let lagged_spread = &spread[..n-1];

    let gamma = calculate_beta(lagged_spread, delta_buf);

    if gamma >= 0.0 {
        return f64::INFINITY;
    }

    -LN_2 / gamma
}

pub fn calculate_z_score(spread: &[f64]) -> f64 {
    let spread_mean = mean(spread);
    let variance = spread.iter().map(|x| (x - spread_mean).powi(2))
        .sum::<f64>() / spread.len() as f64;
    let sigma = variance.sqrt();

    if sigma == 0.0 { return 0.0; }

    let s_curr = spread[spread.len() - 1];

    (s_curr - spread_mean) / sigma
}
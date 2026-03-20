use crate::{data::PriceWindow, math::{calculate_beta, calculate_half_life, calculate_spread}};

#[derive(Debug)]
pub enum PairState {
    Accumulating,
    Active,
    Unwinding,
    Dead
}

pub struct TradingPair {
    pub state: PairState,
    pub window_x: PriceWindow,
    pub window_y: PriceWindow,
    pub current_beta: f64,
    pub current_half_life: f64,
    pub max_half_life_threshold: f64,
    pub has_open_position: bool
}

impl TradingPair {
    pub fn new(capacity: usize, max_half_life: f64) -> Self {
        Self {
            state: PairState::Accumulating,
            window_x: PriceWindow::new(capacity),
            window_y: PriceWindow::new(capacity),
            current_beta: 0.0,
            current_half_life: 0.0,
            max_half_life_threshold: max_half_life,
            has_open_position: false
        }
    }

    pub fn add_prices(
        &mut self, 
        price_x: f64, 
        price_y: f64, 
        vec_x: &mut Vec<f64>, 
        vec_y: &mut Vec<f64>, 
        spread_vec: &mut Vec<f64>, 
        delta_buf: &mut Vec<f64>
    ) {
        self.window_x.push(price_x);
        self.window_y.push(price_y);

        if self.window_x.is_ready() && self.window_y.is_ready() {
            self.recalibrate(vec_x, vec_y, spread_vec, delta_buf);
        }
    }

    fn recalibrate(&mut self, vec_x: &mut Vec<f64>, vec_y: &mut Vec<f64>, spread_vec: &mut Vec<f64>, delta_buf: &mut Vec<f64>) {
        self.window_x.fill_slice(vec_x);
        self.window_y.fill_slice(vec_y);

        self.current_beta = calculate_beta(&vec_x, &vec_y);
        calculate_spread(vec_x, vec_y, self.current_beta, spread_vec);
        self.current_half_life = calculate_half_life(&spread_vec, delta_buf);

        if self.current_half_life > self.max_half_life_threshold || self.current_half_life == f64::INFINITY {
            if self.has_open_position {
                self.state = PairState::Unwinding
            } else {
                self.state = PairState::Dead
            }
        } else {
            self.state = PairState::Active
        }
    }
}
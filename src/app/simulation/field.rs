use ndarray::{Array, Array1, Ix1, s};
use std::ops::{Index, IndexMut};

use crate::app::simulation::{DIVISIONS, WORLD_SIZE};

const STEP: f32 = 2.0 * WORLD_SIZE / ((DIVISIONS - 1) as f32);

pub struct Field {
    field: Array1<f32>,
    x_points: Array1<f32>,
}

impl Field {
    pub fn new() -> Self {
        Field {
            field: Array::zeros(Ix1(DIVISIONS)),
            x_points: Array::linspace(-WORLD_SIZE, WORLD_SIZE, DIVISIONS),
        }
    }

    fn index_of(&self, x: f32) -> f32 {
        (x + WORLD_SIZE) / STEP
    }

    pub fn values(&self) -> &[f32] {
        self.field.slice(s![..]).to_slice().unwrap()
    }

    pub fn values_mut(&mut self) -> &mut [f32] {
        self.field.slice_mut(s![..]).into_slice().unwrap()
    }

    pub fn value_at(&self, x: f32) -> f32 {
        let get_value = |i: f32| -> f32 { *self.values().get(i as usize).unwrap_or(&0.0) };
        let idx = self.index_of(x);
        //return *field.get(idx.round() as usize).unwrap_or(&0.0);
        let (lower_idx, upper_idx) = (idx.floor(), idx.ceil());
        let (lower, upper) = (get_value(lower_idx), get_value(upper_idx));
        lower * (1.0 - idx + lower_idx) + upper * (idx - lower_idx)
    }

    pub fn set_from_function(&mut self, f: fn(f32, f32) -> f32, t: f32) {
        for i in 0..DIVISIONS {
            self.field[i] = f(self.x_points[i], t);
        }
    }

    pub fn intervals(&self) -> &[f32] {
        self.x_points.slice(s![..]).to_slice().unwrap()
    }

    pub fn add(&mut self, rhs: &Field) {
        self.field += &rhs.field;
    }

    pub fn position_at(&self, idx: usize) -> f32 {
        self.x_points[idx]
    }
}

impl Index<usize> for Field {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values()[index]
    }
}

impl IndexMut<usize> for Field {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values_mut()[index]
    }
}

use crate::vertex::Vertex;

use std::convert::TryInto;
use std::sync::Mutex;

use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct PascalTable {
    tab: Vec<Vec<usize>>,
}

impl PascalTable {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tab: vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
                vec![1, 5, 10, 10, 5, 1],
                vec![1, 6, 15, 20, 15, 6, 1],
            ],
        }
    }

    pub fn get(&mut self, n: usize, k: usize) -> usize {
        while n >= self.tab.len() {
            let mut new_row = vec![1; self.tab.len() + 1];
            let prev = self.tab.len() - 1;
            for i in 1..self.tab.len() {
                new_row[i] = self.tab[prev][i - 1] + self.tab[prev][i];
            }

            self.tab.push(new_row);
        }

        self.tab[n][k]
    }
}

lazy_static! {
    static ref PASCAL_TABLE: Mutex<PascalTable> = Mutex::new(PascalTable::new());
}

fn binomial(n: usize, k: usize) -> usize {
    PASCAL_TABLE.lock().unwrap().get(n, k)
}

#[derive(Debug, Clone, Default)]
pub struct Bezier {
    steps: usize,
    points: Vec<Vertex>,
}

impl Bezier {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_steps(self, steps: usize) -> Self {
        Self { steps, ..self }
    }

    pub fn with_points(self, points: &[Vertex]) -> Self {
        Self {
            points: Vec::from(points),
            ..self
        }
    }

    pub fn push_point(&mut self, vertex: Vertex) {
        println!("New point: {:?}", vertex);
        self.points.push(vertex)
    }

    fn single(&self, t: f32) -> Vertex {
        let mut x_sum = 0.;
        let mut y_sum = 0.;
        let n = self.points.len() - 1;

        for (k, v) in self.points.iter().enumerate() {
            let poly = binomial(n, k) as f32
                * (1. - t).powi((n - k).try_into().unwrap())
                * t.powi(k.try_into().unwrap());

            x_sum += v.position[0] * poly;
            y_sum += v.position[1] * poly;
        }

        Vertex::new(x_sum, y_sum)
    }

    pub fn interpolate(&self) -> Option<Vec<Vertex>> {
        if self.steps == 0 || self.points.len() < 3 {
            return None;
        }

        let mut vertex = Vec::with_capacity(self.steps);

        let inc = 1. / self.steps as f32;
        let mut t = 0.;

        while t <= 1. {
            vertex.push(self.single(t));
            t += inc;
        }

        Some(vertex)
    }
}

use super::energy_matrix::EnergyMatrix;

struct Entry {
    cost: u32,
    prev: Option<usize>,
}

fn min2(sp: &Vec<Entry>, idx1: usize, idx2: usize) -> usize {
    if sp[idx1].cost < sp[idx2].cost {
        idx1
    } else {
        idx2
    }
}

fn min3(sp: &Vec<Entry>, idx1: usize, idx2: usize, idx3: usize) -> usize {
    if sp[idx1].cost < sp[idx2].cost && sp[idx1].cost < sp[idx3].cost {
        idx1
    } else if sp[idx2].cost < sp[idx1].cost && sp[idx2].cost < sp[idx3].cost {
        idx2
    } else {
        idx3
    }
}

pub struct CostMatrix {
    rows: Vec<Vec<Entry>>,
}

impl CostMatrix {
    pub fn new(em: &EnergyMatrix) -> Self {
        let width = em.width();
        let height = em.height();
        let mut rows = Vec::with_capacity(height);

        let mut first_row = Vec::with_capacity(width);
        for x in 0..width {
            first_row.push(Entry {
                cost: em.get_energy(x, 0),
                prev: None,
            })
        }
        rows.push(first_row);

        let img_end_idx = width - 1;
        for y in 1..height {
            let mut new_row: Vec<Entry> = Vec::with_capacity(width);
            for x in 0..width {
                let prev_row = &rows[y - 1];
                let prev_min_idx: usize = if x == 0 {
                    min2(prev_row, x, x + 1)
                } else if x == img_end_idx {
                    min2(prev_row, x - 1, x)
                } else {
                    min3(prev_row, x - 1, x, x + 1)
                };

                let cost = em.get_energy(x, y) + rows[y - 1][prev_min_idx].cost;
                new_row.push(Entry {
                    cost,
                    prev: Some(prev_min_idx),
                })
            }
            rows.push(new_row);
        }

        Self { rows }
    }

    pub fn min_cost_seam(&self) -> Vec<usize> {
        let last_row_min_cost_idx = self
            .rows
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cost.cmp(&b.cost))
            .unwrap()
            .0;

        let mut coordinates = vec![last_row_min_cost_idx];
        let mut c = last_row_min_cost_idx;
        for y in (1..self.rows.len()).rev() {
            c = self.rows[y][c].prev.unwrap();
            coordinates.push(c);
        }

        coordinates.into_iter().rev().collect()
    }
}

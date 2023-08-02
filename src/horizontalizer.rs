use std::ops::Range;

use crate::{rectangle::Rectangle, point::Point};

#[derive(Clone)]
pub struct Batch {
    pub m_origin: Point,
    pub m_indices: Range<usize>,
}

impl Batch {
    pub fn new(f_origin: Point, f_indices: Range<usize>) -> Self {
        Self {
            m_origin: f_origin,
            m_indices: f_indices,
        }
    }
}

pub struct Horizontalizer<'a> {
    m_vertical_rectangles: &'a Vec<Rectangle>,
    pub m_horizontal_rectangles: Vec<Rectangle>,
    m_batches: Vec<Batch>,
    m_sorted_indices_by_height: Vec<usize>
}

impl <'a>Horizontalizer<'a> {
    pub fn new(f_vertical_rectangles: &'a Vec<Rectangle>) -> Self {
        Self {
            m_vertical_rectangles: f_vertical_rectangles,
            m_horizontal_rectangles: vec![],
            m_batches: vec![],
            m_sorted_indices_by_height: vec![],
        }
    }

    pub fn run(&mut self) {
        // Init
        let idx = 0;
        let init_origin = Point::default();
        // Put the indices of all vertical rectangles into the init_batch
        let init_batch = Batch::new(
            init_origin,
            idx..self.m_vertical_rectangles.len(),
        );
        self.m_batches.push(init_batch.clone());
        self.set_index_order();

        while !self.m_sorted_indices_by_height.is_empty() {

            let min_idx = self.get_next_min_height_index_and_remove_prev();

            // In which batch am I?
            let mut current_batch: Batch = init_batch.clone();
            let mut current_batch_idx = None;
            for (idx, batch) in self.m_batches.iter().enumerate() {
                if batch.m_indices.contains(&min_idx) {
                    current_batch = batch.clone();
                    current_batch_idx = Some(idx);
                }
            }

            let bottom_right_x = self.m_vertical_rectangles[current_batch.m_indices.clone().last().unwrap()].m_bottom_right.m_x;
            let bottom_right_y = self.m_vertical_rectangles[min_idx].m_bottom_right.m_y;
            self.create_and_add_new_rectangle(
                &current_batch.m_origin,
                &Point::new(bottom_right_x, bottom_right_y));

            self.create_new_batch_if_needed(&current_batch, min_idx);

            // Remove previously found batch
            self.m_batches.remove(current_batch_idx.unwrap());

        }
    }

    /// Iterate over all vertical rectangles and get the indices sorted by height.
    /// Goes from min. height to max. height.
    fn set_index_order(&mut self){
        let mut indices = (0..self.m_vertical_rectangles.len()).collect::<Vec<_>>();
        indices.sort_by_key(|&i| &self.m_vertical_rectangles[i].m_bottom_right.m_y);
        self.m_sorted_indices_by_height = indices;
    }

    /// Returns the next min. height index and removes the already used one.
    fn get_next_min_height_index_and_remove_prev(&mut self) -> usize {
        let next_idx = self.m_sorted_indices_by_height.first().unwrap().clone();
        self.m_sorted_indices_by_height.remove(0);
        next_idx
    }

    /// Creates and adds a new rectangle.
    fn create_and_add_new_rectangle(&mut self, f_top_left: &Point, f_bottom_right: &Point){
        self.m_horizontal_rectangles.push(Rectangle::new(f_top_left, f_bottom_right));
    }

    /// Create new batch if needed
    fn create_new_batch_if_needed(&mut self, f_current_batch: &Batch, f_min_idx: usize) {
        if f_current_batch.m_indices.start < f_min_idx {
            // Left batch
            let left_origin_x = self.m_vertical_rectangles[f_current_batch.m_indices.start].m_top_left.m_x;
            let left_origin_y = self.m_vertical_rectangles[f_min_idx].m_bottom_right.m_y;
            let left_origin = Point::new(left_origin_x, left_origin_y);
            let left_batch = Batch::new(
                left_origin,
                f_current_batch.m_indices.start..f_min_idx,
            );

            self.m_batches.push(left_batch);
        }

        if f_current_batch.m_indices.end-1 > f_min_idx {
            // Right batch
            let right_origin = self.m_vertical_rectangles[f_min_idx].m_bottom_right;
            let right_batch = Batch::new(
                right_origin,
                f_min_idx+1..f_current_batch.m_indices.end,
            );

            self.m_batches.push(right_batch);
        }
    }
}
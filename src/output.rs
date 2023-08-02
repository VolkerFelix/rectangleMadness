use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufWriter;

#[derive(Serialize, Deserialize)]
pub struct RectangleOutputFormat {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Output {
    numRects: usize,
    sourceRectangles: Vec<RectangleOutputFormat>,
    targetRectangles: Vec<RectangleOutputFormat>,
}

impl Output {
    pub fn new(f_num_rects: usize, f_source_rectangles: Vec<RectangleOutputFormat>, f_target_rectangles: Vec<RectangleOutputFormat>) -> Self {
        Self {
            numRects: f_num_rects,
            sourceRectangles: f_source_rectangles,
            targetRectangles: f_target_rectangles
        }
    }

    pub fn save_result(&self) {
        let output = File::create("output.json").unwrap();
        let mut writer = BufWriter::new(output);
        serde_json::to_writer_pretty(&mut writer, &self).unwrap();
    }
}
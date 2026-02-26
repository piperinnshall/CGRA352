use anyhow::Result;
use opencv::{
    core::{self, Mat, Vector},
    imgcodecs,
};

fn main() {
    let source_img = imgcodecs::imread("car.png", imgcodecs::IMREAD_UNCHANGED)?;
}

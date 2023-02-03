// use anyhow::anyhow;
use anyhow::Result;
// use image::RgbImage;
// use ndarray::{ArrayView3};
use opencv::{self as cv, prelude::*};


fn main() -> Result<()> {
    let img_source = cv::imgcodecs::imread("./assets/fgo-1.jpg", cv::imgcodecs::IMREAD_COLOR)?;
    let img_templt = cv::imgcodecs::imread("./assets/fgo-t.png", cv::imgcodecs::IMREAD_COLOR)?;
    let mut img_dst = cv::core::Mat::default();
    let mask = cv::core::Mat::default();

    // let img_output = &img_source.clone();

    let method = cv::imgproc::TM_CCOEFF;
    cv::imgproc::match_template(&img_source, &img_templt, &mut img_dst, method, &mask)?;
    cv::imgcodecs::imwrite("./out.png", &img_dst, &cv::core::Vector::default())?;
    Ok(())
}
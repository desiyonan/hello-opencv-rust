use std::fs;
use anyhow::Result;
// use image::RgbImage;
// use ndarray::{ArrayView3};
use opencv::{
    self as cv, 
    core::Point as Point,
    prelude::*,
};


fn main() -> Result<()> {
    fs::create_dir_all("out")?;
    let img_source = cv::imgcodecs::imread("./assets/fgo-1.png", cv::imgcodecs::IMREAD_COLOR)?;
    let img_templt = cv::imgcodecs::imread("./assets/fgo-t.png", cv::imgcodecs::IMREAD_COLOR)?;
    let mut img_output = img_source.clone();
    let mut res = cv::core::Mat::default();
    let mask = cv::core::Mat::default();


    let method = cv::imgproc::TM_CCOEFF_NORMED;
    cv::imgproc::match_template(&img_source, &img_templt, &mut res, method, &mask)?;
    cv::imgcodecs::imwrite("./out/tmp.png", &res, &cv::core::Vector::default())?;
        
    let mut min_loc = Point::new(0,0);
    let mut max_loc = Point::new(0,0);
    cv::core::min_max_loc(&res, 
        None,
        None,
        Some(&mut min_loc),
        Some(&mut max_loc),
        &mask)?;

    let top_left = max_loc;
    let btm_right = top_left + Point::from_size(img_templt.size()?);

    // print!("top:{:#?}, btm:{:#?}", top_left, btm_right);

    cv::imgproc::rectangle(
        &mut img_output,
        cv::core::Rect::from_points(top_left, btm_right),
        cv::core::VecN([255., 0., 0., 0.]),
        -1,
        cv::imgproc::LINE_8,
        0,
    )?;
    cv::imgcodecs::imwrite("./out/res.png", &img_output, &cv::core::Vector::default())?;
    Ok(())
}
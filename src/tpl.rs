use anyhow::anyhow;
use anyhow::Result;
use image::RgbImage;
use opencv::{
    self as cv, 
    prelude::*,
    imgcodecs::ImreadModes as ImreadModes
};


fn main() {
    let img_source = cv::imgcodecs::imread("./assets/fgo-1.jpg", cv::imgcodecs::IMREAD_COLOR);
    let img_templt = cv::imgcodecs::imread("./assets/fgo-t.jpg", cv::imgcodecs::IMREAD_COLOR);

    let img_output = img_source::clone();

    let method = cv::imgproc::TM_CCOEFF;
    let res:ToOutputArray;
    cv::imgproc::match_template(img, template, method, &res);
}
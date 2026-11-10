use libvips::{VipsImage, ops};

use crate::utils::Error;
/*
pub fn mask_image2(original: &VipsImage, mask: &VipsImage) -> Result<VipsImage, Error> {
    Ok(ops::composite2(mask, original, ops::BlendMode::Atop)?)
} */

fn fix_bands(img: &VipsImage, img2: &VipsImage) -> Result<VipsImage, Error> {
    let missing = img2.get_bands() - img.get_bands();

    if missing <= 0 {
        return Ok(img.clone());
    }

    Ok(ops::bandjoin_const(
        img,
        &mut vec![255.0; missing as usize],
    )?)
}

pub fn mask_image(original: &VipsImage, mask: &VipsImage) -> Result<VipsImage, Error> {
    let original_alpha = ops::extract_band(original, 3)?;
    let alpha = ops::linear(
        &ops::multiply(&original_alpha, &mask)?,
        &mut [1.0 / 255.0],
        &mut [0.0],
    )?;
    let rgb = ops::extract_band_with_opts(original, 0, &ops::ExtractBandOptions { n: 3 })?;

    Ok(ops::bandjoin(&mut [rgb, alpha])?)
}

pub fn linear_without_the_fucking_alpha(
    img: &VipsImage,
    a: &mut [f64],
    b: &mut [f64],
) -> Result<VipsImage, Error> {
    let rgb = ops::extract_band_with_opts(
        img,
        0,
        &ops::ExtractBandOptions {
            n: 3,
            ..Default::default()
        },
    )?;

    let rgb = ops::linear(&rgb, a, b)?;
    let alpha = ops::extract_band(img, 3)?;

    Ok(ops::bandjoin(&mut [rgb, alpha])?)
}

pub fn overlay(base: &VipsImage, overlay: &VipsImage) -> Result<VipsImage, Error> {
    let base = fix_bands(base, &overlay)?;
    let overlay = overlay;

    Ok(ops::composite2(&base, &overlay, ops::BlendMode::Over)?)
}

use libvips::{VipsImage, ops};

use crate::utils::Error;

fn fix_bands(img: &VipsImage, amount: i32, fill: f64) -> Result<VipsImage, Error> {
    let bands = img.get_bands();

    if bands == amount {
        return Ok(img.clone());
    }

    if bands > amount {
        return Ok(ops::extract_band_with_opts(
            img,
            0,
            &ops::ExtractBandOptions {
                n: amount,
                ..Default::default()
            },
        )?);
    }

    Ok(ops::bandjoin_const(
        img,
        &mut vec![fill; (amount - bands) as usize],
    )?)
}

pub fn mask_image(original: &VipsImage, mask: &VipsImage) -> Result<VipsImage, Error> {
    let original = fix_bands(original, 4, 255.0)?;
    let mask = fix_bands(mask, 4, 255.0)?;

    let rgb = ops::extract_band_with_opts(
        &original,
        0,
        &ops::ExtractBandOptions {
            n: 3,
            ..Default::default()
        },
    )?;

    let original_alpha = ops::extract_band(&original, 3)?;
    let mask_alpha = ops::extract_band(&mask, 3)?;

    let alpha = ops::multiply(&original_alpha, &mask_alpha)?;
    let alpha = ops::linear(&alpha, &mut [1.0 / 255.0], &mut [0.0])?;

    Ok(ops::bandjoin(&mut [rgb, alpha])?)
}

pub fn linear_without_the_fucking_alpha(
    img: &VipsImage,
    a: &mut [f64],
    b: &mut [f64],
) -> Result<VipsImage, Error> {
    let img = fix_bands(img, 4, 255.0)?;

    let rgb = ops::extract_band_with_opts(
        &img,
        0,
        &ops::ExtractBandOptions {
            n: 3,
            ..Default::default()
        },
    )?;

    let rgb = ops::linear(&rgb, a, b)?;
    let alpha = ops::extract_band(&img, 3)?;

    Ok(ops::bandjoin(&mut [rgb, alpha])?)
}

pub fn overlay(base: &VipsImage, overlay: &VipsImage) -> Result<VipsImage, Error> {
    let base = fix_bands(base, 4, 255.0)?;
    let overlay = fix_bands(overlay, 4, 255.0)?;

    Ok(ops::composite2(&base, &overlay, ops::BlendMode::Over)?)
}

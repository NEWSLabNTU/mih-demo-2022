use once_cell::sync::Lazy;
use palette::{FromColor, Hsv, RgbHue, Srgb};
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash, Hasher},
};

const DEFAULT_SATURATION: f64 = 1.0;
const DEFAULT_VALUE: f64 = 1.0;
static DEFAULT_BUILD_HASHER: Lazy<RandomState> = Lazy::new(|| RandomState::default());

/// Samples a RGB array from the hash of input value.
pub fn sample_rgb<T>(value: &T) -> [f64; 3]
where
    T: Hash,
{
    sample_rgb_with_sl_and_hasher(
        value,
        DEFAULT_SATURATION,
        DEFAULT_VALUE,
        &*DEFAULT_BUILD_HASHER,
    )
}

/// Samples a RGB array with fixed saturation and value and a
/// custom hasher from the hash of input value.
pub fn sample_rgb_with_sl_and_hasher<T, S>(
    seed: &T,
    saturation: f64,
    value: f64,
    build_hasher: &S,
) -> [f64; 3]
where
    T: Hash,
    S: BuildHasher,
{
    // Compute the hash of the input seed.
    let hash = {
        let mut hasher = build_hasher.build_hasher();
        seed.hash(&mut hasher);
        hasher.finish()
    };

    // Pick a color in HSV space.
    let hsv = Hsv::new(
        RgbHue::from_degrees((hash.wrapping_mul(79) % 360) as f64),
        saturation,
        value,
    );

    // Convert the HSV color to RGB space.
    let (r, g, b) = Srgb::from_color(hsv).into_components();
    [r, g, b]
}

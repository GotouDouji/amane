use crate::Color;

/// Basic Material 3 roles generated with CIELAB LCh, not the MCU HCT model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Md3ColorScheme {
    pub seed: Color,
    pub is_dark: bool,
    pub primary: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,
    pub secondary: Color,
    pub on_secondary: Color,
    pub secondary_container: Color,
    pub on_secondary_container: Color,
    pub tertiary: Color,
    pub on_tertiary: Color,
    pub tertiary_container: Color,
    pub on_tertiary_container: Color,
    pub error: Color,
    pub on_error: Color,
    pub error_container: Color,
    pub on_error_container: Color,
    pub background: Color,
    pub on_background: Color,
    pub surface: Color,
    pub on_surface: Color,
    pub surface_variant: Color,
    pub on_surface_variant: Color,
    pub surface_dim: Color,
    pub surface_bright: Color,
    pub surface_container_lowest: Color,
    pub surface_container_low: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
    pub outline: Color,
    pub outline_variant: Color,
    pub inverse_surface: Color,
    pub inverse_on_surface: Color,
    pub inverse_primary: Color,
    pub shadow: Color,
    pub scrim: Color,
    pub surface_tint: Color,
}

#[derive(Clone, Copy)]
struct Lch {
    hue: f64,
    chroma: f64,
}

impl Md3ColorScheme {
    pub fn from_seed(seed: Color, is_dark: bool) -> Self {
        let seed_lch = to_lch(seed);
        let primary = Lch {
            hue: seed_lch.hue,
            chroma: 36.0,
        };
        let secondary = Lch {
            hue: seed_lch.hue,
            chroma: 16.0,
        };
        let tertiary = Lch {
            hue: wrap_degrees(seed_lch.hue + 60.0),
            chroma: 24.0,
        };
        let neutral = Lch {
            hue: seed_lch.hue,
            chroma: 6.0,
        };
        let neutral_variant = Lch {
            hue: seed_lch.hue,
            chroma: 8.0,
        };
        let error = Lch {
            hue: 25.0,
            chroma: 84.0,
        };

        let (accent, on_accent, accent_container, on_accent_container) = if is_dark {
            (80.0, 20.0, 30.0, 90.0)
        } else {
            (40.0, 100.0, 90.0, 10.0)
        };
        let (background_tone, on_background_tone) = if is_dark {
            (6.0, 90.0)
        } else {
            (99.0, 10.0)
        };
        let (surface_dim_tone, surface_bright_tone, containers) = if is_dark {
            (6.0, 24.0, [4.0, 10.0, 12.0, 17.0, 22.0])
        } else {
            (87.0, 98.0, [100.0, 96.0, 94.0, 92.0, 90.0])
        };
        let (on_surface_tone, surface_variant_tone, on_surface_variant_tone) = if is_dark {
            (90.0, 30.0, 80.0)
        } else {
            (10.0, 90.0, 30.0)
        };
        let (outline_tone, outline_variant_tone) = if is_dark {
            (60.0, 30.0)
        } else {
            (50.0, 80.0)
        };
        let (inverse_surface_tone, inverse_on_surface_tone, inverse_primary_tone) = if is_dark {
            (90.0, 20.0, 40.0)
        } else {
            (20.0, 95.0, 80.0)
        };

        let primary_color = tone(primary, accent);
        let surface = tone(neutral, background_tone);

        Self {
            seed,
            is_dark,
            primary: primary_color,
            on_primary: tone(primary, on_accent),
            primary_container: tone(primary, accent_container),
            on_primary_container: tone(primary, on_accent_container),
            secondary: tone(secondary, accent),
            on_secondary: tone(secondary, on_accent),
            secondary_container: tone(secondary, accent_container),
            on_secondary_container: tone(secondary, on_accent_container),
            tertiary: tone(tertiary, accent),
            on_tertiary: tone(tertiary, on_accent),
            tertiary_container: tone(tertiary, accent_container),
            on_tertiary_container: tone(tertiary, on_accent_container),
            error: tone(error, accent),
            on_error: tone(error, on_accent),
            error_container: tone(error, accent_container),
            on_error_container: tone(error, on_accent_container),
            background: surface,
            on_background: tone(neutral, on_background_tone),
            surface,
            on_surface: tone(neutral, on_surface_tone),
            surface_variant: tone(neutral_variant, surface_variant_tone),
            on_surface_variant: tone(neutral_variant, on_surface_variant_tone),
            surface_dim: tone(neutral, surface_dim_tone),
            surface_bright: tone(neutral, surface_bright_tone),
            surface_container_lowest: tone(neutral, containers[0]),
            surface_container_low: tone(neutral, containers[1]),
            surface_container: tone(neutral, containers[2]),
            surface_container_high: tone(neutral, containers[3]),
            surface_container_highest: tone(neutral, containers[4]),
            outline: tone(neutral_variant, outline_tone),
            outline_variant: tone(neutral_variant, outline_variant_tone),
            inverse_surface: tone(neutral, inverse_surface_tone),
            inverse_on_surface: tone(neutral, inverse_on_surface_tone),
            inverse_primary: tone(primary, inverse_primary_tone),
            shadow: Color::BLACK,
            scrim: Color::BLACK,
            surface_tint: primary_color,
        }
    }
}

impl Default for Md3ColorScheme {
    fn default() -> Self {
        Self::from_seed(Color::rgb(0x89, 0xb4, 0xfa), true)
    }
}

fn tone(palette: Lch, tone: f64) -> Color {
    from_lab(
        tone,
        palette.chroma * palette.hue.to_radians().cos(),
        palette.chroma * palette.hue.to_radians().sin(),
    )
}

fn to_lch(color: Color) -> Lch {
    let red = linear(f64::from(color.red()) / 255.0);
    let green = linear(f64::from(color.green()) / 255.0);
    let blue = linear(f64::from(color.blue()) / 255.0);

    let x = (red * 0.412_456_4 + green * 0.357_576_1 + blue * 0.180_437_5) / 0.950_47;
    let y = red * 0.212_672_9 + green * 0.715_152_2 + blue * 0.072_175;
    let z = (red * 0.019_333_9 + green * 0.119_192 + blue * 0.950_304_1) / 1.088_83;

    let fx = lab_f(x);
    let fy = lab_f(y);
    let fz = lab_f(z);
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);

    Lch {
        hue: wrap_degrees(b.atan2(a).to_degrees()),
        chroma: a.hypot(b),
    }
}

fn from_lab(lightness: f64, a: f64, b: f64) -> Color {
    let fy = (lightness + 16.0) / 116.0;
    let fx = fy + a / 500.0;
    let fz = fy - b / 200.0;
    let x = 0.950_47 * lab_f_inverse(fx);
    let y = lab_f_inverse(fy);
    let z = 1.088_83 * lab_f_inverse(fz);

    let red = x * 3.240_454_2 + y * -1.537_138_5 + z * -0.498_531_4;
    let green = x * -0.969_266 + y * 1.876_010_8 + z * 0.041_556;
    let blue = x * 0.055_643_4 + y * -0.204_025_9 + z * 1.057_225_2;

    Color::rgb(encoded(red), encoded(green), encoded(blue))
}

fn linear(channel: f64) -> f64 {
    if channel <= 0.040_45 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

fn encoded(channel: f64) -> u8 {
    let channel = channel.clamp(0.0, 1.0);
    let encoded = if channel <= 0.003_130_8 {
        channel * 12.92
    } else {
        1.055 * channel.powf(1.0 / 2.4) - 0.055
    };

    (encoded * 255.0).round() as u8
}

fn lab_f(value: f64) -> f64 {
    const EPSILON: f64 = 216.0 / 24_389.0;
    const KAPPA: f64 = 24_389.0 / 27.0;

    if value > EPSILON {
        value.cbrt()
    } else {
        (KAPPA * value + 16.0) / 116.0
    }
}

fn lab_f_inverse(value: f64) -> f64 {
    const EPSILON: f64 = 216.0 / 24_389.0;
    const KAPPA: f64 = 24_389.0 / 27.0;
    let cube = value * value * value;

    if cube > EPSILON {
        cube
    } else {
        (116.0 * value - 16.0) / KAPPA
    }
}

fn wrap_degrees(degrees: f64) -> f64 {
    degrees.rem_euclid(360.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_light_and_dark_roles_from_seed() {
        let seed = Color::rgb(0x67, 0x50, 0xa4);
        let light = Md3ColorScheme::from_seed(seed, false);
        let dark = Md3ColorScheme::from_seed(seed, true);

        assert_eq!(light.seed, seed);
        assert!(!light.is_dark);
        assert!(dark.is_dark);
        assert_ne!(light.surface, dark.surface);
        assert_ne!(light.primary, dark.primary);
        assert_eq!(light.primary.alpha(), 255);
        assert_eq!(dark.primary.alpha(), 255);
    }

    #[test]
    fn tone_zero_and_one_hundred_reach_neutral_extremes() {
        let neutral = Lch {
            hue: 0.0,
            chroma: 0.0,
        };

        assert_eq!(tone(neutral, 0.0), Color::BLACK);
        assert_eq!(tone(neutral, 100.0), Color::WHITE);
    }
}
//! Portable transcendental functions (A1.5u, D22).
//!
//! The standard library's `f32::tanh`, `exp` and `ln` call the platform's
//! libm, whose results differ in their last bits between glibc versions:
//! glibc 2.39 and 2.44 disagree on `tanhf` for 5.5% of a 2.16-million-input
//! probe, by up to 2 ulp. Scores, learned leaves and features built on them
//! then differ from machine to machine, and a game replayed on another
//! machine can diverge at a near-tie. These functions make no platform
//! transcendental call: they use IEEE 754 arithmetic on `f64` (add, subtract,
//! multiply, divide), bit manipulation, and `round`, whose result Rust
//! specifies exactly, and round once to `f32`, so every supported platform
//! computes the same bits. On the probe grids of the tests below they agree
//! with `f64` references to within an `f32` ulp; that is a tested property,
//! not an exhaustive guarantee. What matters is that they are the same
//! everywhere.

const LN2: f64 = std::f64::consts::LN_2;
// ln 2 split so that `k * LN2_HI` is exact for every `k` used here (fdlibm's split).
const LN2_HI: f64 = 6.931_471_803_691_238_164_90e-1;
const LN2_LO: f64 = 1.908_214_929_270_587_700_02e-10;

/// e^x: `k = round(x / ln 2)`, `r = x - k ln 2` with `|r| <= ln 2 / 2`, a
/// Taylor series in `r`, scaled by 2^k built from its bits.
fn exp64(x: f64) -> f64 {
    if x.is_nan() {
        return x;
    }
    if x > 709.0 {
        return f64::INFINITY;
    }
    if x < -708.0 {
        return 0.0;
    }
    let k = (x / LN2).round();
    let r = (x - k * LN2_HI) - k * LN2_LO;
    // |r| < 0.347, so the tail after r^17 / 17! is below 1e-20.
    let (mut sum, mut term) = (1.0, 1.0);
    for n in 1..=17 {
        term *= r / n as f64;
        sum += term;
    }
    // k is in [-1021, 1023] here, so 2^k is a normal f64.
    sum * f64::from_bits(((k as i64 + 1023) as u64) << 52)
}

/// ln x: `x = m 2^e` with `m` in `[sqrt(1/2), sqrt(2))`, then
/// `ln m = 2 atanh(s)` with `s = (m - 1) / (m + 1)`, `|s| < 0.172`.
fn ln64(x: f64) -> f64 {
    if x.is_nan() || x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return f64::NEG_INFINITY;
    }
    if x.is_infinite() {
        return x;
    }
    let bits = x.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i64;
    // Every input is a widened f32, so never a subnormal f64.
    debug_assert!(biased != 0, "ln64 of a subnormal");
    let mut e = biased - 1023;
    let mut m = f64::from_bits((bits & ((1u64 << 52) - 1)) | (1023u64 << 52));
    if m > std::f64::consts::SQRT_2 {
        m /= 2.0;
        e += 1;
    }
    let s = (m - 1.0) / (m + 1.0);
    let s2 = s * s;
    // s^25 < 1e-19.
    let (mut sum, mut p) = (0.0, s);
    for n in 0..=12 {
        sum += p / (2 * n + 1) as f64;
        p *= s2;
    }
    2.0 * sum + e as f64 * LN2
}

/// e^x, the same on every machine.
pub fn exp(x: f32) -> f32 {
    exp64(x as f64) as f32
}

/// ln x, the same on every machine.
pub fn ln(x: f32) -> f32 {
    ln64(x as f64) as f32
}

/// tanh x, the same on every machine.
pub fn tanh(x: f32) -> f32 {
    let x = x as f64;
    if x.is_nan() {
        return f32::NAN;
    }
    let a = x.abs();
    let t = if a < 1e-5 {
        // tanh a = a - a^3/3 + ..., within 4e-11 of a here.
        a
    } else if a > 20.0 {
        1.0
    } else {
        let e = exp64(2.0 * a);
        (e - 1.0) / (e + 1.0)
    };
    t.copysign(x) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Inputs spanning what the callers pass: material differences over 30,
    /// network pre-activations, probabilities and sizes, at strides that
    /// cover each binade.
    fn grid() -> Vec<f32> {
        let mut xs: Vec<f32> = (-360..=360).map(|i| ((i as f64 / 4.0) / 30.0) as f32).collect();
        let top = 20.0f32.to_bits();
        xs.extend((0..top).step_by(4099).flat_map(|b| [f32::from_bits(b), -f32::from_bits(b)]));
        xs
    }

    fn ulps(a: f32, b: f32) -> i64 {
        (a.to_bits() as i32 as i64 - b.to_bits() as i32 as i64).abs()
    }

    /// Within an f32 ulp of an f64 reference on the probe grid.
    #[test]
    fn portable_functions_are_accurate() {
        for x in grid() {
            assert!(ulps(tanh(x), (x as f64).tanh() as f32) <= 1, "tanh({x})");
            if x.abs() < 80.0 {
                assert!(ulps(exp(x), (x as f64).exp() as f32) <= 1, "exp({x})");
            }
            if x > 0.0 {
                assert!(ulps(ln(x), (x as f64).ln() as f32) <= 1, "ln({x})");
            }
        }
    }

    #[test]
    fn portable_functions_handle_their_edges() {
        assert_eq!(tanh(0.0), 0.0);
        assert_eq!(tanh(-1.5), -tanh(1.5));
        assert_eq!((tanh(f32::INFINITY), tanh(f32::NEG_INFINITY)), (1.0, -1.0));
        assert_eq!((exp(0.0), ln(1.0)), (1.0, 0.0));
        assert_eq!((exp(f32::NEG_INFINITY), exp(200.0)), (0.0, f32::INFINITY));
        assert_eq!(ln(0.0), f32::NEG_INFINITY);
        assert!(ln(-1.0).is_nan() && exp(f32::NAN).is_nan() && tanh(f32::NAN).is_nan());
    }

    /// No platform transcendental call is left in this crate's code outside
    /// its tests, whether written as a method (`x.tanh()`) or a path
    /// (`f32::tanh(x)`): each would make scores, leaves or features differ
    /// across machines again (A1.5u). `powi`, whose precision Rust leaves
    /// unspecified, counts too.
    #[test]
    fn no_platform_transcendentals_outside_tests() {
        fn sources(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            for e in std::fs::read_dir(dir).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() { sources(&p, out) } else if p.extension().is_some_and(|x| x == "rs") { out.push(p) }
            }
        }
        const NAMES: &[&str] = &["tanh", "exp", "exp2", "exp_m1", "ln", "ln_1p", "log", "log2",
            "log10", "powf", "powi", "sin", "cos", "tan", "sin_cos", "asin", "acos", "atan", "atan2",
            "sinh", "cosh", "asinh", "acosh", "atanh", "cbrt", "hypot", "gamma", "ln_gamma"];
        let calls: Vec<String> = NAMES.iter()
            .flat_map(|n| [format!(".{n}("), format!("f32::{n}("), format!("f64::{n}(")])
            .collect();
        let mut files = Vec::new();
        sources(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut files);
        let mut found = Vec::new();
        for f in &files {
            let text = std::fs::read_to_string(f).unwrap();
            let code = text.split("#[cfg(test)]").next().unwrap_or("");
            for (i, line) in code.lines().enumerate() {
                let line = line.split("//").next().unwrap_or("");
                if calls.iter().any(|c| line.contains(c.as_str())) {
                    found.push(format!("{}:{}: {}", f.display(), i + 1, line.trim()));
                }
            }
        }
        assert!(files.len() > 10, "the crate's sources were found");
        assert!(found.is_empty(), "platform transcendental calls outside tests; use crate::portable:\n{}", found.join("\n"));
    }

    /// The outputs' bits are pinned: these functions make no platform
    /// transcendental call, so the same bits hold on every machine, and a
    /// change to them fails here.
    #[test]
    fn portable_function_bits_are_pinned() {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut eat = |v: f32| for b in v.to_bits().to_le_bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x0100_0000_01b3);
        };
        for x in grid() {
            eat(tanh(x));
            eat(exp(x.clamp(-80.0, 80.0)));
            eat(ln(x.abs() + 1e-3));
        }
        assert_eq!(h, 0xfbee_06d0_058c_a62c, "pinned bits {h:#018x}");
    }
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand_chacha::{rand_core::{RngCore, SeedableRng}, ChaCha8Rng};
use core::f32;

const PI_OVER_4: f32 = f32::consts::PI / 4.0;

#[inline(always)]
pub fn fast_atan(x: f32) -> f32 {
    return (PI_OVER_4 * x) - (x * (x.abs() - 1.0) * (0.2447 + 0.663*x.abs())); 
}

const A: f32 = 0.0776509570923569;
const B: f32 = -0.287434475393028;
const C: f32 = PI_OVER_4 - A - B;

#[inline(always)]
pub fn fast_atan_alt(x: f32) -> f32 {
    let squared = x*x;
    //return x - (squared*x/3f32) + squared*squared*x/5f32 - (squared*squared*squared*x/7f32)
    return ((A*squared + B) * squared + C) * x;
}

#[inline(always)]
fn landon_clip(input: f32, drive: f32) -> f32
{
    let input_drive: f32 = input * drive;
    return 2.0 / 3.14 * input_drive.atan();
}

#[inline(always)]
fn fast_atan_less_accurate(input: f32, drive: f32) -> f32
{
    let input_drive: f32 = input * drive;
    return 2.0 / 3.14 * fast_atan(input_drive);
}

#[inline(always)]
fn fast_atan_more_accurate(input: f32, drive: f32) -> f32
{
    let input_drive: f32 = input * drive;
    return 2.0 / 3.14 * fast_atan_alt(input_drive);
}

const SAMPLE_RATE: usize = 44100;

fn maybe_negative_one(rng: &mut ChaCha8Rng) -> f32 {
    if rng.next_u32() > (i32::MAX as u32) {
        return -1f32
    }
    1f32
}

fn bench(c: &mut Criterion) {
    let mut input_data = [[0f32; 3]; SAMPLE_RATE];
    let mut rng = ChaCha8Rng::from_seed([0u8; 32]);
    for [l, r, drive] in input_data.iter_mut() {
        *l = (rng.next_u32() as f32) / (u32::MAX as f32) * maybe_negative_one(&mut rng);
        *r = (rng.next_u32() as f32) / (u32::MAX as f32) * maybe_negative_one(&mut rng);
        *drive = (rng.next_u32() as f32) / (u32::MAX as f32);
    }

    c.bench_function("landon_clip", |b| b.iter(|| {
        for [l, r, drive] in input_data {
            black_box(landon_clip(l, drive));
            black_box(landon_clip(r, drive));
        }
    }));

    c.bench_function("clip_with_fast_more_accurate_atan", |b| b.iter(|| {
        for [l, r, drive] in input_data {
            black_box(fast_atan_more_accurate(l, drive));
            black_box(fast_atan_more_accurate(r, drive));
        }
    }));

    c.bench_function("clip_with_fast_less_accurate_atan", |b| b.iter(|| {
        for [l, r, drive] in input_data {
            black_box(fast_atan_less_accurate(l, drive));
            black_box(fast_atan_less_accurate(r, drive));
        }
    }));
}

criterion_group!(benches, bench);
criterion_main!(benches);
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand_chacha::{rand_core::{RngCore, SeedableRng}, ChaCha8Rng};
extern crate rust;
use rust::{fast_atan_less_accurate, fast_atan_more_accurate, landon_clip};
use core::f32;

const SAMPLE_RATE: usize = 44100;

/// pseudorandomly returns -1 or 1
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
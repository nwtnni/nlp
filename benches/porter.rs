#[macro_use]
extern crate criterion;
extern crate nlp;

use nlp::stem::porter::Porter;
use criterion::Criterion;

const DICTIONARY: &'static str = include_str!("../resources/dict.txt");

fn porter(c: &mut Criterion) {
    c.bench_function("porter-dict", |b| b.iter(|| {
        for word in DICTIONARY.split("\n").map(|word| word.trim_right()).take(100) {
            Porter::stem(word).is_ok();
        }
    }));
}

criterion_group!(benches, porter);
criterion_main!(benches);

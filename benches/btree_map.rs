#![feature(test)]

extern crate test;

use std::collections::BTreeMap;
use test::Bencher;

#[bench]
fn bench(b: &mut Bencher) {
    let mut map = BTreeMap::new();
    b.iter(|| {
        map.insert(0u8, "something");
        map.insert(1, "something");
        let _ = map.remove(&0);
        map.insert(2, "something");
        map.insert(3, "something");
        map.insert(4, "something");
        let _ = map.remove(&3);
        map.insert(5, "something");
        let _ = map.remove(&2);
        let _ = map.remove(&1);
        let _ = map.remove(&4);
        let _ = map.remove(&5);
    });
}

#[bench]
fn lookup(b: &mut Bencher) {
    let mut map = BTreeMap::new();
    for i in 0i32..100 {
        map.insert(i, "something");
    }
    b.iter(|| {
        test::black_box(map[&20i32]);
    });
}

#[bench]
fn insert_delete(b: &mut Bencher) {
    let mut map = BTreeMap::new();

    for i in 0..100 {
        map.insert(i, "something");
    }

    map.remove(&10);
    map.remove(&50);
    map.remove(&20);

    b.iter(|| {
        map.insert(20, test::black_box("something"));
        test::black_box(map.remove(&test::black_box(20)));
    });
}

#![feature(test)]

extern crate stash;
extern crate test;

use test::Bencher;
use std::collections::HashMap;

use stash::Stash;

#[bench]
fn bench_hash_map(b: &mut Bencher) {
    let mut map = HashMap::with_capacity(6);
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
fn bench_stash(b: &mut Bencher) {
    let mut stash = Stash::with_capacity(6);
    b.iter(|| {
        let t1 = stash.put("something");
        let t2 = stash.put("something");
        let _ = stash.take(t1);
        let t3 = stash.put("something");
        let t4 = stash.put("something");
        let t5 = stash.put("something");
        let _ = stash.take(t4);
        let t6 = stash.put("something");
        let _ = stash.take(t3);
        let _ = stash.take(t2);
        let _ = stash.take(t5);
        let _ = stash.take(t6);
    });
}

#[bench]
fn bench_box(b: &mut Bencher) {
    b.iter(|| {
        let b1 = Box::new("something");
        let b2 = Box::new("something");
        drop(b1);
        let b3 = Box::new("something");
        let b4 = Box::new("something");
        let b5 = Box::new("something");
        drop(b4);
        let b6 = Box::new("something");
        drop(b3);
        drop(b2);
        drop(b5);
        drop(b6);
    });
}

#[bench]
fn bench_stash_init(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..1000 {
            test::black_box(Stash::<u64>::new());
        }
    });
}

#[bench]
fn bench_stash_access(b: &mut Bencher) {
    let mut stash = Stash::new();
    let mut tickets = Vec::with_capacity(100);
    for _ in 0..100 {
        tickets.push(stash.put("something"));
    }
    let ref t = tickets[20];
    b.iter(|| {
        test::black_box(&stash[*t]);
    });
}

#[bench]
fn bench_hash_map_access(b: &mut Bencher) {
    let mut map = HashMap::with_capacity(100);
    for i in 0i32..100 {
        map.insert(i, "something");
    }
    b.iter(|| {
        test::black_box(map[&20i32]);
    });
}


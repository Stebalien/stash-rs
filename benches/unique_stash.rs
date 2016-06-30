#![feature(test)]

extern crate stash;
extern crate test;

use test::Bencher;
use std::iter;

use stash::{UniqueStash, Tag};

#[bench]
fn bench(b: &mut Bencher) {
    let mut stash = UniqueStash::new();
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
fn lookup(b: &mut Bencher) {
    let mut stash = UniqueStash::new();
    let mut tickets = Vec::new();
    for _ in 0..100 {
        tickets.push(stash.put("something"));
    }
    let ref t = tickets[20];
    b.iter(|| {
        test::black_box(&stash[*t]);
    });
}

#[bench]
fn insert_delete(b: &mut Bencher) {
    let mut stash = UniqueStash::new();

    let tags: Vec<Tag> = stash
        .extend(iter::repeat("something").take(100))
        .collect();

    stash.take(tags[10]);
    stash.take(tags[50]);
    stash.take(tags[20]);

    b.iter(|| {
        let tag = stash.put(test::black_box("something"));
        test::black_box(stash.take(test::black_box(tag)));
    });
}

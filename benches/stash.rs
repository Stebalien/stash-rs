#![feature(test)]

extern crate stash;
extern crate test;

use test::Bencher;

use stash::Stash;

#[bench]
fn put_and_take(b: &mut Bencher) {
    let mut stash: Stash<&str, usize> = Stash::with_capacity(6);
    b.iter(|| {
        let t1 = stash.put("something");
        let t2 = stash.put("something");
        let _  = test::black_box(stash.take(t1).unwrap());
        let t3 = stash.put("something");
        let t4 = stash.put("something");
        let t5 = stash.put("something");
        let _  = test::black_box(stash.take(t4).unwrap());
        let t6 = stash.put("something");
        let _  = test::black_box(stash.take(t3).unwrap());
        let _  = test::black_box(stash.take(t2).unwrap());
        let _  = test::black_box(stash.take(t5).unwrap());
        let _  = test::black_box(stash.take(t6).unwrap());
    });
}

#[bench]
fn put_and_take_unchecked(b: &mut Bencher) {
    let mut stash: Stash<&str, usize> = Stash::with_capacity(6);
    b.iter(|| {
        let t1 = stash.put("something");
        let t2 = stash.put("something");
        let _  = unsafe{ test::black_box(stash.take_unchecked(t1)) };
        let t3 = stash.put("something");
        let t4 = stash.put("something");
        let t5 = stash.put("something");
        let _  = unsafe{ test::black_box(stash.take_unchecked(t4)) };
        let t6 = stash.put("something");
        let _  = unsafe{ test::black_box(stash.take_unchecked(t3)) };
        let _  = unsafe{ test::black_box(stash.take_unchecked(t2)) };
        let _  = unsafe{ test::black_box(stash.take_unchecked(t5)) };
        let _  = unsafe{ test::black_box(stash.take_unchecked(t6)) };
    });
}

#[bench]
fn put_and_take_block(b: &mut Bencher) {
    let mut stash: Stash<&str, usize> = Stash::with_capacity(10);
    b.iter(|| {
        let t0 = stash.put("something");
        let t1 = stash.put("something");
        let t2 = stash.put("something");
        let t3 = stash.put("something");
        let t4 = stash.put("something");
        let t5 = stash.put("something");
        let t6 = stash.put("something");
        let t7 = stash.put("something");
        let t8 = stash.put("something");
        let t9 = stash.put("something");
        let _ = test::black_box(stash.take(t1).unwrap());
        let _ = test::black_box(stash.take(t4).unwrap());
        let _ = test::black_box(stash.take(t3).unwrap());
        let _ = test::black_box(stash.take(t2).unwrap());
        let _ = test::black_box(stash.take(t5).unwrap());
        let _ = test::black_box(stash.take(t6).unwrap());
        let _ = test::black_box(stash.take(t0).unwrap());
        let _ = test::black_box(stash.take(t8).unwrap());
        let _ = test::black_box(stash.take(t9).unwrap());
        let _ = test::black_box(stash.take(t7).unwrap());
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = test::black_box(stash.take(t4).unwrap());
        let _ = test::black_box(stash.take(t5).unwrap());
        let _ = test::black_box(stash.take(t3).unwrap());
        let _ = test::black_box(stash.take(t1).unwrap());
        let _ = test::black_box(stash.take(t9).unwrap());
        let _ = test::black_box(stash.take(t2).unwrap());
        let _ = test::black_box(stash.take(t6).unwrap());
        let _ = test::black_box(stash.take(t7).unwrap());
        let _ = test::black_box(stash.take(t8).unwrap());
        let _ = test::black_box(stash.take(t0).unwrap());
    });
}

#[bench]
fn put_and_take_unchecked_block(b: &mut Bencher) {
    let mut stash: Stash<&str, usize> = Stash::with_capacity(10);
    b.iter(|| {
        let t0 = stash.put("something");
        let t1 = stash.put("something");
        let t2 = stash.put("something");
        let t3 = stash.put("something");
        let t4 = stash.put("something");
        let t5 = stash.put("something");
        let t6 = stash.put("something");
        let t7 = stash.put("something");
        let t8 = stash.put("something");
        let t9 = stash.put("something");
        let _ = unsafe{ test::black_box(stash.take_unchecked(t1)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t4)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t3)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t2)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t5)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t6)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t0)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t8)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t9)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t7)) };
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = stash.put("something");
        let _ = unsafe{ test::black_box(stash.take_unchecked(t4)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t5)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t3)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t1)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t9)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t2)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t6)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t7)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t8)) };
        let _ = unsafe{ test::black_box(stash.take_unchecked(t0)) };
    });
}

fn setup<'a>() -> (Stash<&'a str>, Vec<usize>) {
    let n = 100;
    let mut stash = Stash::with_capacity(n);
    let mut tickets = Vec::with_capacity(n);
    for _ in 0..n {
        tickets.push(stash.put("foo"));
    }
    (stash, tickets)
}

#[bench]
fn get(b: &mut Bencher) {
    let (stash, tickets) = setup();
    b.iter(|| {
        for &t in tickets.iter() {
            test::black_box(stash.get(t).unwrap());
        }
    });
}

#[bench]
fn get_unchecked(b: &mut Bencher) {
    let (stash, tickets) = setup();
    b.iter(|| {
        for &t in tickets.iter() {
            test::black_box(unsafe{ stash.get_unchecked(t) });
        }
    });
}

#[bench]
fn get_mut(b: &mut Bencher) {
    let (mut stash, tickets) = setup();
    b.iter(|| {
        for &t in tickets.iter() {
            test::black_box(stash.get_mut(t).unwrap());
        }
    });
}

#[bench]
fn get_unchecked_mut(b: &mut Bencher) {
    let (mut stash, tickets) = setup();
    b.iter(|| {
        for &t in tickets.iter() {
            test::black_box(unsafe{ stash.get_unchecked_mut(t) });
        }
    });
}

#[bench]
fn ops_index(b: &mut Bencher) {
    let (stash, tickets) = setup();
    b.iter(|| {
        for &i in tickets.iter() {
            test::black_box(stash[i]);
        }
    });
}

#[bench]
fn iter_sparse(b: &mut Bencher) {
    let (mut stash, tickets) = setup();
    stash.put("something");
    for t in tickets {
        stash.take(t);
    }
    b.iter(|| {
        test::black_box(test::black_box(&stash).iter().next().unwrap());
    });
}

#[bench]
fn iter(b: &mut Bencher) {
    let (stash, _) = setup();
    b.iter(|| {
        for i in test::black_box(&stash) {
            test::black_box(i);
        }
    });
}

#[bench]
fn insert_delete(b: &mut Bencher) {
    let (mut stash, _) = setup();

    stash.take(10);
    stash.take(50);
    stash.take(20);

    b.iter(|| {
        let index = stash.put(test::black_box("something"));
        test::black_box(stash.take(test::black_box(index)));
    });
}

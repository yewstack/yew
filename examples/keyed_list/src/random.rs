use rand::distributions::Bernoulli;
use rand::Rng;

/// `0 <= p <= 1`
pub fn chance(p: f64) -> bool {
    let d = Bernoulli::new(p).unwrap();
    rand::thread_rng().sample(d)
}

/// half-open: [min, max)
pub fn range_exclusive(min: usize, max: usize) -> usize {
    let len: usize = rand::thread_rng().gen();
    len % (max - min) + min
}

/// Choose two distinct indices `(a, b)` such that `a < b`.
pub fn choose_two_distinct_indices<T>(items: &[T]) -> Option<(usize, usize)> {
    match items.len() {
        0 | 1 => None,
        2 => Some((0, 1)),
        n => {
            let first = range_exclusive(0, n);
            // find another index that isn't `first`
            let second = loop {
                let i = range_exclusive(0, n);
                // this must be true at some point because there are at least three items
                if i != first {
                    break i;
                }
            };

            // make sure that `a < b`
            if first > second {
                Some((second, first))
            } else {
                Some((first, second))
            }
        }
    }
}

pub fn choose_two_distinct_mut<T>(items: &mut [T]) -> Option<(&mut T, &mut T)> {
    let (lo, hi) = choose_two_distinct_indices(items)?;

    // a = `items[0..hi]` which contains `lo` because `lo < hi`
    // b = `items[hi..]` where `items[hi] == b[0]`
    let (a, b) = items.split_at_mut(hi);
    Some((&mut a[lo], &mut b[0]))
}

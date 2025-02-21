use rand::distr::Bernoulli;
use rand::Rng;

/// `0 <= p <= 1`
pub fn chance(p: f64) -> bool {
    let d = Bernoulli::new(p).unwrap();
    rand::rng().sample(d)
}

/// half-open: [min, max)
pub fn range_exclusive(min: usize, max: usize) -> usize {
    rand::rng().random_range(min..max)
}

pub fn choose_two_distinct_mut<T>(items: &mut [T]) -> Option<(&mut T, &mut T)> {
    let (lo, hi) = {
        // Choose two distinct indices `(a, b)` such that `a < b`.
        match items.len() {
            0 | 1 => return None,
            _ => {
                let indexes = rand::seq::index::sample(&mut rand::rng(), items.len(), 2);
                let (a, b) = (indexes.index(0), indexes.index(1));
                if a < b {
                    (a, b)
                } else {
                    (b, a)
                }
            }
        }
    };

    // a = `items[0..hi]` which contains `lo` because `lo < hi`
    // b = `items[hi..]` where `items[hi] == b[0]`
    let (a, b) = items.split_at_mut(hi);
    Some((&mut a[lo], &mut b[0]))
}

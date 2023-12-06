// Copyright 2023 Alex Jago <abjago.net>
// Released under the MIT or Apache-2.0 licenses, at your option.

use core::ops::Range;

/** A trait for splitting [`Range`]s and maybe other things too.

Usage:
```
# use range_partition::Split;
let foo = 0..10;
let bar = 3..6;

assert_eq!(foo.split(&bar), (Some(0..3), Some(3..6), Some(6..10)));
```
Named "split" rather than "partition" because the latter is an iterator method available on Range by default.
**/
pub trait Split {
    /// Split `self` by `other` into up to three parts:
    /// * `.0` : `self < other`
    /// * `.1`: intersection of `self` and `other`
    /// * `.2`: `self > other`
    fn split(&self, other: &Self) -> (Option<Self>, Option<Self>, Option<Self>)
    where
        Self: Sized;
}

impl<T> Split for Range<T>
where
    T: Sized + Ord + Copy,
{
    fn split(&self, other: &Self) -> (Option<Self>, Option<Self>, Option<Self>) {
        let mut below = None;
        let mut inter = None;
        let mut above = None;
        if self.start < other.start {
            // below exists
            below = Some(self.start..self.end.min(other.start));
            if other.start < self.end && self.end <= other.end {
                // inter but no above
                inter = Some(other.start..self.end);
            }
            if other.end < self.end {
                // inter and above
                inter = Some(other.clone());
                above = Some(other.end..self.end);
            }
        } else if other.contains(&self.start) {
            // no below
            inter = Some(self.start..self.end.min(other.end));
            if other.end < self.end {
                // also above
                above = Some(other.end..self.end);
            }
        } else {
            // above only
            above = Some(self.clone());
        }

        (below, inter, above)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: Range<usize> = 0..5;
    const B: Range<usize> = 4..10;
    const C: Range<usize> = 6..8;

    #[test]
    fn a_le_b() {
        assert_eq!(A.split(&B), (Some(0..4), Some(4..5), None))
    }

    #[test]
    fn b_ge_a() {
        assert_eq!(B.split(&A), (None, Some(4..5), Some(5..10)))
    }

    #[test]
    fn a_lt_c() {
        assert_eq!(A.split(&C), (Some(0..5), None, None))
    }

    #[test]
    fn a_eq_a() {
        assert_eq!(A.split(&A), (None, Some(0..5), None))
    }

    #[test]
    fn b_contains_c() {
        assert_eq!(B.split(&C), (Some(4..6), Some(6..8), Some(8..10)))
    }

    #[test]
    fn c_within_b() {
        assert_eq!(C.split(&B), (None, Some(6..8), None))
    }

    #[test]
    fn c_gt_a() {
        assert_eq!(C.split(&A), (None, None, Some(6..8)))
    }
}

/*
assert_eq!(range_split(&a, &b), (Some(0..4), Some(4..5), None));
        assert_eq!(range_split(&a, &c), (Some(0..5), None, None));
        assert_eq!(range_split(&c, &b), (None, Some(6..8), None));
        assert_eq!(range_split(&c, &a), (None, None, Some(6..8)));
        assert_eq!(
            range_split(&b, &c),
            (Some(4..6), Some(6..8), Some(8..10))
        );
        assert_eq!(range_split(&b, &a), (None, Some(4..5), Some(5..10)))
*/

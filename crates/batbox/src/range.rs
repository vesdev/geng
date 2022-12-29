//! Extra utilities for working with ranges
use super::*;

pub mod prelude {
    //! Items intended to always be available. Reexported from [crate::prelude]

    #[doc(no_inline)]
    pub use super::*;

    #[doc(no_inline)]
    pub use std::ops::{
        Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
    };
}

/// Same as [RangeBounds] but without exclusive bounds
pub trait FixedRangeBounds<T: ?Sized> {
    /// Start index bound
    fn start_bound(&self) -> FixedBound<&T>;
    /// End index bound
    fn end_bound(&self) -> FixedBound<&T>;
}

/// Same as [Bound] but without exclusive bounds
pub enum FixedBound<T> {
    Included(T),
    Unbounded,
}

impl<T> FixedRangeBounds<T> for RangeInclusive<T> {
    fn start_bound(&self) -> FixedBound<&T> {
        FixedBound::Included(self.start())
    }
    fn end_bound(&self) -> FixedBound<&T> {
        FixedBound::Included(self.end())
    }
}

impl<T> FixedRangeBounds<T> for RangeFull {
    fn start_bound(&self) -> FixedBound<&T> {
        FixedBound::Unbounded
    }
    fn end_bound(&self) -> FixedBound<&T> {
        FixedBound::Unbounded
    }
}

impl<T> FixedRangeBounds<T> for RangeFrom<T> {
    fn start_bound(&self) -> FixedBound<&T> {
        FixedBound::Included(&self.start)
    }
    fn end_bound(&self) -> FixedBound<&T> {
        FixedBound::Unbounded
    }
}

impl<T> FixedRangeBounds<T> for RangeToInclusive<T> {
    fn start_bound(&self) -> FixedBound<&T> {
        FixedBound::Unbounded
    }
    fn end_bound(&self) -> FixedBound<&T> {
        FixedBound::Included(&self.end)
    }
}

// Convert any range into a `start..end` [Range] as if used for slicing
pub fn index_range<R>(len: usize, range: R) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    Range {
        start: match range.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        },
        end: match range.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => len,
        },
    }
}

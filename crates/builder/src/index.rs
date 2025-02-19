use std::fmt::Debug;
use std::iter::{Step, Sum};
use std::sync::atomic::Ordering;

use atoi::FromRadix10;

pub trait Idx:
    Copy
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Mul<Output = Self>
    + Ord
    + Debug
    + Send
    + Sum
    + Sync
    + Sized
    + Step
    + 'static
{
    type Atomic: AtomicIdx<Inner = Self>;

    fn new(idx: usize) -> Self;

    fn zero() -> Self;

    fn index(self) -> usize;

    fn atomic(self) -> Self::Atomic;

    fn parse(bytes: &[u8]) -> (Self, usize);
}

pub trait AtomicIdx: Send + Sync {
    type Inner: Idx<Atomic = Self>;

    fn load(&self, order: Ordering) -> Self::Inner;

    fn fetch_add(&self, val: Self::Inner, order: Ordering) -> Self::Inner;

    fn get_and_increment(&self, order: Ordering) -> Self::Inner;

    fn zero() -> Self;

    fn into_inner(self) -> Self::Inner;
}

macro_rules! impl_idx {
    ($TYPE:ty,$ATOMIC_TYPE:ident) => {
        use std::sync::atomic::$ATOMIC_TYPE;

        impl Idx for $TYPE {
            type Atomic = $ATOMIC_TYPE;

            #[inline]
            fn new(idx: usize) -> Self {
                assert!(idx <= <$TYPE>::MAX as usize);
                idx as $TYPE
            }

            #[inline]
            fn zero() -> Self {
                0
            }

            #[inline]
            fn index(self) -> usize {
                self as usize
            }

            #[inline]
            fn atomic(self) -> $ATOMIC_TYPE {
                <$ATOMIC_TYPE>::new(self)
            }

            #[inline]
            fn parse(bytes: &[u8]) -> (Self, usize) {
                FromRadix10::from_radix_10(bytes)
            }
        }

        impl AtomicIdx for $ATOMIC_TYPE {
            type Inner = $TYPE;

            #[inline]
            fn load(&self, order: Ordering) -> Self::Inner {
                self.load(order)
            }

            #[inline]
            fn fetch_add(&self, val: $TYPE, order: Ordering) -> Self::Inner {
                self.fetch_add(val, order)
            }

            #[inline]
            fn get_and_increment(&self, order: Ordering) -> Self::Inner {
                self.fetch_add(1, order)
            }

            #[inline]
            fn zero() -> Self {
                <$ATOMIC_TYPE>::new(0)
            }

            #[inline]
            fn into_inner(self) -> Self::Inner {
                self.into_inner()
            }
        }
    };
}

impl_idx!(u8, AtomicU8);
impl_idx!(u16, AtomicU16);
impl_idx!(u32, AtomicU32);
impl_idx!(u64, AtomicU64);
impl_idx!(usize, AtomicUsize);

impl_idx!(i8, AtomicI8);
impl_idx!(i16, AtomicI16);
impl_idx!(i32, AtomicI32);
impl_idx!(i64, AtomicI64);
impl_idx!(isize, AtomicIsize);

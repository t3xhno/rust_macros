use paste;

#[macro_export]
macro_rules! avec {
    ($($e:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::new();
        $(vs.push($e);)*
        vs
    }};
    ($e:expr; $count:expr) => {{
        let mut vs = Vec::new();
        let x = $e;
        for _ in 0..$count {
            vs.push(x.clone());
        }
        vs
    }};
}

trait MaxVal {
    fn max_val() -> Self;
}

macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxVal for $t {
            fn max_val() -> Self {
                <$t>::MAX
            }
        }
    };
}

max_impl!(u32);
max_impl!(i32);
max_impl!(u64);
max_impl!(i64);

macro_rules! test_max_val {
    ($t:ty) => {
        paste::item! {
            #[test]
            fn [< $t _max >] () {
                assert_eq!(<$t>::MAX, <$t>::max_val());
            }
        }
    };
}

test_max_val!(u32);
test_max_val!(i32);
test_max_val!(u64);
test_max_val!(i64);

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn one_element() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn multiple_elements_trailing_comma() {
    let x: Vec<u32> = avec![1, 2, 3, 4, 5, 6,];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 6);
    assert_eq!(x[0], 1);
    assert_eq!(x[4], 5);
}

#[test]
fn semicolon_works() {
    let x: Vec<u32> = avec![42; 12];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 12);
    assert_eq!(x[0], 42);
    assert_eq!(x[9], 42);
}
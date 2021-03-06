use paste;

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

macro_rules! impl_and_test {
    ($($t:ty),+) => {
        $(
            max_impl!($t);
            test_max_val!($t);
        )+
    };
}

impl_and_test!(u32, i32, u64, i64);

#[macro_export]
macro_rules! avec {
    ($($e:expr),*) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::count![@COUNT; $($e),*]);
        $(vs.push($e);)*
        vs
    }};

    ($($e:expr,)*) => {{
        $crate::avec![$($e),*];
    }};

    ($e:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vec.resize($count, $e);
        vs
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    //
    // Converts the $e to slice, and uses the unit slice [()]
    // implementation of the len function to count the
    // number of elements!
    //
    (@COUNT; $($e:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $e]),*])
    };

    (@SUBST; $e:expr) => { () };
}

#[macro_export]
macro_rules! sanity_check {
    ($x:expr, $e:expr) => {
        assert!(!$x.is_empty());
        assert_eq!($x.len(), $e.len());
        for i in 0..$x.len() {
            assert_eq!($x[i], $e[i]);
        }
    };
}

#[macro_export]
macro_rules! generate_tests {
    ($($n:ident"lukanoob"$e:expr);+ $(;)?) => {
        paste::item! {
            $(
                #[test]
                fn [< $n >] () {
                    sanity_check!(avec!($e), [$e]);
                }
            )+
        }
    };
}

generate_tests!(
    empty_vec"lukanoob"(); /* Unclear what happened, but it passes... */
    one_ele"lukanoob"[42];
    multiple_ele"lukanoob"[1, 2, 3, 4, 5, 6];
    trailing_comma"lukanoob"[1, 2, 3, 4, 5, 6,];
    semicolon_ele"lukanoob"[42; 12]
);

/// ```compile_fail
/// let x: Vec<u32> = macros::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;

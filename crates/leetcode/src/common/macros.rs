#[macro_export]
#[doc(hidden)]
macro_rules! __test_impl {
    // 同时测试多个没有输入参数的功能函数
    (
        $name:ident {
        $($func:path => $expected:expr);+ $(;)?
    }) => {
        #[test]
        fn $name() {
            $(
                assert_eq!($func(), $expected);
            )+
        }
    };

    // 测试一个功能函数
    (
        $name:ident {
        $func:path;
        $($($input:expr),+ => $expected:expr);+ $(;)?
    }) => {
        #[test]
        fn $name() {
            $(
                assert_eq!($func($($input),+),$expected);
            )+
        }
    };

    // 同时测试两个功能函数
    (
        $name:ident {
        $func1:path;
        $func2:path;
        $($($input:expr),+ => $expected:expr);+ $(;)?
    }) => {
        #[test]
        fn $name() {
            $(
                assert_eq!($func1($($input),+),$expected);
                assert_eq!($func2($($input),+),$expected);
            )+
        }
    };

    // 同时测试三个功能函数
    (
        $name:ident {
        $func1:path;
        $func2:path;
        $func3:path;
        $($($input:expr),+ => $expected:expr);+ $(;)?
    }) => {
        #[test]
        fn $name() {
            $(
                assert_eq!($func1($($input),+),$expected);
                assert_eq!($func2($($input),+),$expected);
                assert_eq!($func3($($input),+),$expected);
            )+
        }
    }
}

/// # Examples
///
/// ```
/// pub struct Solution;
///
/// impl Solution {
///     pub fn add(lhs:i32, rhs:i32) -> i32 {
///         lhs + rhs
///     }
///
///     pub fn add_another(lhs:i32, rhs:i32) -> i32 {
///         lhs + rhs
///     }
///
///     pub fn sub(lhs:i32, rhs:i32) -> i32 {
///         lhs - rhs
///     }
///
///     pub fn get_pi() -> f64 {
///         3.1415926
///     }
///
///     pub fn get_gravity_constant() -> f64 {
///         9.8
///     }
/// }
///
/// #[cfg(test)]
/// mod tests {
///      use leetcode::test;
///      use super::*;
///
///     test!(
///         test_add {         // 测试函数名
///             Solution::add; // 待测函数，可以有多个,共享下列所有测试用例
///             Solution::add_another;
///
///             1, 2 => 3;     // 测试用例
///             -1, 1 => 0;
///         };
///
///         test_sub {
///             Solution::sub;
///
///             10, 5 => 5;
///             0, 1 => -1;
///         };
///
///         test_get_pi {
///             Solution::get_pi => 3.1415926;
///             Solution::get_gravity_constant => 9.8
///         }
///     );
/// }
/// ```
#[macro_export]
macro_rules! test {
    (
        $(
            $name:ident {
                $($inner:tt)+
            }
        );+ $(;)?
    ) => {
        $(
            $crate::__test_impl! {
                $name {
                    $($inner)+
                }
            }
        )+
    };
}

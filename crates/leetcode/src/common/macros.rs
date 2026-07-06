#[macro_export]
macro_rules! test {
    // 同时测试多个没有输入参数的功能函数
    (
        $name:ident{
        $($func:path => $expected:expr);* $(;)?
    }) => {
        #[test]
        fn $name() {
            $(
                assert_eq!($func(), $expected);
            )*
        }
    };

    // 测试一个功能函数
    (
        $name:ident{
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
        $name:ident{
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
        $name:ident{
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

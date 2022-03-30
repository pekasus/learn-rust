#[macro_export]
macro_rules! hashmap {
    ( $( $k:expr => $v:expr),+ $(,)? ) => {
        {
            let mut output = ::std::collections::HashMap::new();
            $(
                output.insert($k, $v);
            )*
        output
        }
    };
    () => {
        ::std::collections::HashMap::new()
    }
}

// hashmap!('a' => 3, 'b' => 11, 'z' => 32)

// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

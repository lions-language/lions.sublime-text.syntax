pub mod lmod1;

pub fn libf1() {
    println!("libf1");
}

/*BEGIN*/fn unused() {
//          ^^^^^^WARN(>=1.41.0-beta) function is never used
//          ^^^^^^NOTE(>=1.41.0-beta) #[warn(dead_code)]
//       ^^^^^^^^^^^WARN(>=1.22.0,<1.41.0-beta) function is never used
//       ^^^^^^^^^^^NOTE(>=1.22.0,<1.41.0-beta) #[warn(dead_code)]
}/*END*/
// ~WARN(<1.22.0) function is never used
// ~NOTE(<1.22.0,>=1.17.0) #[warn(dead_code)]

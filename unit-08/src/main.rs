// use std::fs::File;
// use std::io::{ErrorKind, Read};
// use std::io::Error;
//
// fn main() {
//     // {
//     //     panic!("hahaha")
//     // }
//
//     // {
//     //     let v = vec![1,2,3];
//     //     println!("{}", v[5])
//     // }
//
//     // {
//     //     let f:u32 = File::open("abc.txt");
//     // }
//
//     // {
//     //     println!("========================Result===================");
//     //     let file_name = "abc.txt";
//     //     let f = match  File::open(file_name){
//     //         Ok(file) => file,
//     //         Err(err) => match  err.kind(){
//     //             ErrorKind::NotFound => match File::create(file_name){
//     //                 Ok(fc) => fc,
//     //                 Err(e) => panic!("create file {:?}", e)
//     //             }
//     //             other_err => panic!("other err {:?}", other_err)
//     //         }
//     //     };
//     //
//     // }
//
//
//     // {
//     //     println!("========================unwrap和expect===================");
//     //     let f = File::open("efg.txt").unwrap(); // 自动触发panic
//     //     let f = File::open("efg.txt").expect("efg err"); // 自动触发panic并附带信息
//     // }
//
//     {
//         println!("========================err 传播===================");
//         fn read_from_file()->   Result<String, Error>{
//             let mut s = String::new();
//             // ?用于自动处理错误输出，有错误就中断return   ? 只适用于返回值为Result的函数
//             File::open("abc.txt")?.read_to_string(&mut s)?;
//             Ok(s)
//         }
//         let s =         read_from_file();
//         println!("err {:?}", s.err())
//     }
//
// }


use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("def.txt");
    // main 的返回类型是()
    //  Box 表示所有错误
    Ok(())
}
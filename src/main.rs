// // // use open::that;
// // use std::process::Command;
// // fn main() {
// //     // 打开网页
// //     // that("https://cn.vitejs.dev/").expect("Failed to open browser");
// //     // println!("Hello, world!");
// //     // 根据操作系统执行不同的命令
// //     if cfg!(target_os = "windows") {
// //         Command::new("explorer")
// //             .arg(".")
// //             .spawn()
// //             .expect("Failed to open file manager on Windows");
// //     } else if cfg!(target_os = "linux") {
// //         Command::new("xdg-open")
// //             .arg(".")
// //             .spawn()
// //             .expect("Failed to open file manager on Linux");
// //     } else if cfg!(target_os = "macos") {
// //         Command::new("open")
// //             .arg(".")
// //             .spawn()
// //             .expect("Failed to open file manager on macOS");
// //     } else {
// //         eprintln!("Unsupported operating system");
// //     }
// //     // 根据操作系统执行不同的命令 打开终端
// //     // if cfg!(target_os = "windows") {
// //     //     Command::new("cmd")
// //     //         .arg("/c")
// //     //         .arg("start")
// //     //         .arg("cmd.exe")
// //     //         .spawn()
// //     //         .expect("Failed to open terminal on Windows");
// //     // } else if cfg!(target_os = "linux") {
// //     //     Command::new("gnome-terminal")
// //     //         .arg("--")
// //     //         .arg("bash")
// //     //         .arg("-c")
// //     //         .arg("echo Hello, World!")
// //     //         .spawn()
// //     //         .expect("Failed to open terminal on Linux");
// //     // } else if cfg!(target_os = "macos") {
// //     //     Command::new("open")
// //     //         .arg("-a")
// //     //         .arg("Terminal")
// //     //         .arg("echo Hello, World!")
// //     //         .spawn()
// //     //         .expect("Failed to open terminal on macOS");
// //     // } else {
// //     //     eprintln!("Unsupported operating system");
// //     // }
// // }
// // // 第一步，先通过rust把数据保存到指定目录下面，然后通过读取数据的方式去读取数据
// // // 第二步，在页面上面显示数据，并且可以进行修改
// // // 第三步，实现数据的保存和读取功能
// // // 第四步，实现数据的修改功能
// // // 第五步，实现数据的删除功能
// // // 第六步，实现数据的搜索功能
// // // 第七步，根据搜索功能去匹配数据，如果有数据就直接打开浏览器，如果没有可以提示用户没有找到数据
// // // 第八步，根据搜索内容去匹配数据，如果是打开终端就直接打开终端，如果是打开文件管理器就打开文件管理器,
// // // 还要支持数据导入功能
// fn main() {
//     // let value:i32 = 5;
//     // // 模式匹配 跟 js 中的 switch 类似
//     // match value {
//     //     1 => println!("one"),
//     //     2 => println!("two"),
//     //     3 | 4 => println!("three or four"),
//     //     5 => println!("five"),
//     //     _ => println!("something else"),
//     // }
//     // let fruit: &str = "apple";
//     // match fruit {
//     //     "apple" => println!("An apple a day keeps the doctor away"),
//     //     "banana" => println!("The banana is yellow"),
//     //     _ => println!("I don't know what fruit that is"),
//     // }
//     let x = 5; // 默认是不能改变的，使用这个还可以进行重复覆盖，在覆盖的同时，还可以修改类型
//     println!("x的值是:{}", x);
//     let mut y = 6; // 声明可变的变量
//     println!("y的值是:{}", y);
//     y = 7; // 修改可变的变量
//     println!("y的值是:{}", y);
//     const MAX_AGE: u32 = 20; // 常量  常量在创建的时候，必须要标注类型
//     println!("MAX_AGE的值是:{}", MAX_AGE);
//     // let mut x = 5; // 默认是不能改变的
//     let str: u32 = "42".parse().expect("非数值类型"); // 字符串转数字
//     println!("str的值是:{}", str);
//     // 整数类型
//     let a: i8 = 127; // 8位有符号整数
//     println!("a的值是:{}", a);
//     let b: i16 = -32768; // 16位有符号整数
//     println!("b的值是:{}", b);
//     let c: i32 = 2147483647; // 32位有符号整数
//     println!("c的值是:{}", c);
//     let d: i64 = -9223372036854775808; // 64位有符号整数
//     println!("d的值是:{}", d);
//     // 无符号整数
//     let e: u8 = 255; // 8位无符号整数
//     println!("e的值是:{}", e);
//     let f: u16 = 65535; // 16位无符号整数
//     println!("f的值是:{}", f);
//     let g: u32 = 4294967295; // 32位无符号整数
//     println!("g的值是:{}", g);
//     let h: u64 = 18446744073709551615; // 64位无符号整数
//     println!("h的值是:{}", h);
//     //  无符号跟有符号的区别是一个有负数，一个没有负数
//     // isize 跟 usize 跟平台相关，64位系统上是64位，32位系统上是32位

//     // 浮点数类型
//     let i: f32 = 3.14; // 32位浮点数 单精度
//     println!("i的值是:{}", i);
//     let j: f64 = 2.71828; // 64位浮点数 双精度浮点数
//     println!("j的值是:{}", j);
//     // 布尔类型
//     let k: bool = true; // 布尔类型
//     println!("k的值是:{}", k);
//     // 字符类型  char类型使用「单引号」指定，字符串使用「双引号」指定。
//     let l: char = 'x'; // 单个字符
//     println!("l的值是:{}", l);
//     let m: &str = "hello"; // 字符串
//     println!("m的值是:{}", m);
//     // 元组类型  元组类型可以包含不同类型的数据 ,不能再声明后对其进行修改
//     let n: (i32, f64, &str) = (1, 2.3, "hello"); // 元组
//     println!("n的值是:{:#?}", n);
//     // 对元组进行结构

//     let (aa, bb, cc) = (1, 2.3, "hello");
//     println!("aa的值是:{}", aa);
//     println!("bb的值是:{}", bb);
//     println!("cc的值是:{}", cc);

//     // 数组类型 数组类型的长度是固定的，一旦声明就不能随意修改
//     let o: [i32; 3] = [1, 2, 3]; // 数组
//     println!("o的值是:{:#?}", o);
//     // 正常访问数组内容
//     println!("o[0]的值是:{}", o[0]);
//     let index = 2;
//     let item = o[index];
//     println!("item的值是:{}", item); // 这种在编译的时候没有问题，但是在运行的时候会报错
//                                      // 数组越界访问

//     // // 指针类型
//     // let p: *const i32 = &x; // 指向i32的指针
//     // println!("p的值是:{}", p);
//     // let q: *mut i32 = &mut y; // 指向可变i32的指针
//     // println!("q的值是:{}", q);

//     let sun = add(10, 20);
//     println!("sun的值是:{:?}", sun);
//     loop_api()
// }

// // 函数的写法 ,必须要写每一个参数的类型，如果有返回数据，也要写成返回类型
// fn add(a: i32, b: i32) -> i32 {
//     if a == 5 {
//         // 条件判断，必须要满足一个bool类型
//         println!("满足要求")
//     } else {
//         println!("不满足要求")
//     }
//     return a + b;
// }

// fn loop_api() {
//     let a = [1, 2, 3, 4, 5];
//     for i in a.iter() {
//         println!("i的值是:{}", i)
//     }

//     // loop {
//     //     println!("重复执行")
//     // }
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let _f = File::open("hello.txt");

//     let _f = match _f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("创建文件失败: {:?}", e),
//             },
//             other_error => panic!("打开文件失败: {:?}", other_error),
//         },
//     };
//     println!("{:?}",_f);
// }

// use anyhow::{Result};
// use clap::Parser;
// use std::fs;
// use std::path::PathBuf;
// use std::env;
// /// 在文件系统中搜索指定的文件。
// #[derive(Parser)]
// struct Cli {
//     /// 要查找的文件名
//     filename: String,
// }

// fn main() -> Result<()> {
//     let args = Cli::parse();
//     let home_dir = env::var("USERPROFILE")?; // 对于 Windows
//     println!("home_dir: {}", home_dir);
//     // let home_dir = env::var("HOME")?; // 对于 Unix-like 系统
//     search_file(&args.filename, &PathBuf::from("D:\\"))?;
//     Ok(())
// }

// fn search_file(filename: &str, dir: &PathBuf) -> Result<()> {
//     println!("开始");
//     if dir.is_dir() {
//         for entry in fs::read_dir(dir)? {
//             let entry = entry?;
//             let path = entry.path();
//             if path.is_dir() {
//                 search_file(filename, &path)?;
//             } else if path.is_file() && path.file_name().unwrap() == filename {
//                 println!("文件 {} 找到，路径: {:?}", filename, path);
//                 return Ok(());
//             }
//         }
//     }
//     Ok(())
// }



use anyhow::{Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// 在文件系统中搜索指定的文件。
#[derive(Parser)]
struct Cli {
    /// 要查找的文件名
    filename: String,
    /// 搜索深度
    #[clap(short, long, default_value = "3")]
    depth: usize,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    search_file(&args.filename, &PathBuf::from("D:\\"), args.depth)?;
    Ok(())
}

fn search_file(filename: &str, dir: &PathBuf, depth: usize) -> Result<()> {
    if depth == 0 {
        return Ok(());
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                search_file(filename, &path, depth - 1)?;
            } else if path.is_file() && path.file_name().unwrap() == filename {
                println!("文件 {} 找到，路径: {:?}", filename, path);
                return Ok(());
            }
        }
    }
    Ok(())
}

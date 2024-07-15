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

// use anyhow::{Result};
// use clap::Parser;
// use std::fs;
// use std::path::PathBuf;

// /// 在文件系统中搜索指定的文件。
// #[derive(Parser)]
// struct Cli {
//     /// 要查找的文件名
//     filename: String,
//     /// 搜索深度
//     #[clap(short, long, default_value = "3")]
//     depth: usize,
// }

// fn main() -> Result<()> {
//     let args = Cli::parse();
//     search_file(&args.filename, &PathBuf::from("D:\\"), args.depth)?;
//     Ok(())
// }

// fn search_file(filename: &str, dir: &PathBuf, depth: usize) -> Result<()> {
//     if depth == 0 {
//         return Ok(());
//     }

//     if dir.is_dir() {
//         for entry in fs::read_dir(dir)? {
//             let entry = entry?;
//             let path = entry.path();
//             if path.is_dir() {
//                 search_file(filename, &path, depth - 1)?;
//             } else if path.is_file() && path.file_name().unwrap() == filename {
//                 println!("文件 {} 找到，路径: {:?}", filename, path);
//                 return Ok(());
//             }
//         }
//     }
//     Ok(())
// }
// use std::fs::{read_dir, remove_dir_all, remove_file};
// use std::fs::remove_dir_all;
// use std::io;
// use std::path::Path;

// fn delete_dir_contents(dir_path: &Path) -> io::Result<()> {
//     if let Ok(entries) = read_dir(dir_path) {
//         for entry in entries {
//             if let Ok(entry) = entry {
//                 let entry_path = entry.path();
//                 if entry.file_type()?.is_file() {
//                     remove_file(&entry_path)?;
//                     println!("Deleted file: {:?}", entry_path);
//                 } else if entry.file_type()?.is_dir()
//                     && entry.path() != Path::new(".")
//                     && entry.path() != Path::new("..")
//                 {
//                     // 递归删除子目录
//                     delete_dir_contents(&entry_path)?;
//                     // 注意：在删除文件后再删除空目录
//                     remove_dir_all(&entry_path)?;
//                     println!("Deleted directory: {:?}", entry_path);
//                 }
//             }
//         }
//     }
//     Ok(())
// }

// fn main() {
//     let dir_path = Path::new("D:\\\\zhuomian\\test\\rustdemoapi\\project_name\\cd"); // 修改为你要删除的目录路径，这里使用当前目录作为示例
//     // if let Err(e) = delete_dir_contents(dir_path) {
//     //     eprintln!("Error deleting directory contents: {}", e);
//     // }
//     if let Err(e) = remove_dir_all(&dir_path) {
//         eprintln!("Error deleting directory: {}", e);
//     } else {
//         println!("Directory deleted successfully.");

//     }

// }
use std::env;
// // use std::fs::remove_dir_all;
// use std::fs;
// use std::time::Instant;

// fn main() {
//     // 获取命令行参数
//     let args: Vec<String> = env::args().collect();

//     // 检查是否有足够的参数
//     if args.len() > 1 {
//         // 获取第二个参数
//         let second_arg = &args[1];
//         println!("The second argument is: {}", second_arg);

//         // 记录开始时间
//         let start_time = Instant::now();

//         // 获取文件的实际大小，而非硬盘上的大小
//         let len = fs::metadata(&second_arg).unwrap().len();
//         println!("len: {}", len);
//         // // 删除目录
//         // if let Err(e) = remove_dir_all(&second_arg) {
//         //     eprintln!("Error deleting directory: {}", e);
//         // } else {
//         //     println!("Directory deleted successfully.");
//         // }

//         // 计算并打印执行时间
//         let duration = start_time.elapsed();
//         println!("Time elapsed in deleting directory is: {:?}", duration);
//     } else {
//         println!("No second argument provided.");
//     }
// }
use std::fs::read_dir;
use std::io;
use std::path::Path;

// // 递归地计算目录的大小
fn dir_size(path: &Path) -> io::Result<u64> {
    let mut total_size = 0u64;
    // print!("{:?}---\n", path);
    // 先判断是不是文件

    if path.is_file() {
        return Ok(path.metadata()?.len());
    }
    // 尝试读取目录中的所有条目
    if let Ok(entries) = read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if let Ok(entry_type) = entry_path.metadata() {
                    // 如果是文件，则累加其大小
                    if entry_type.is_file() {
                        total_size += entry_type.len();
                        // print!("是否是文件{:?},是{:?} \n",entry_path.is_file(),entry_path);
                        // print!("{},{:?} ", entry_type.len(),entry_path); // 打印文件大小
                    }
                    // 如果是目录，则递归计算其大小
                    else if entry_type.is_dir() {
                        total_size += dir_size(&entry_path)?;
                    }
                    // 可以选择忽略其他类型的文件系统条目，如符号链接等
                }
            }
        }
    }

    // 注意：这里没有处理 read_dir 可能返回的错误，因为我们已经用 if let Ok(...) 处理了
    // 如果 read_dir 失败，函数将直接返回 Err 而不进入循环体
    // print!("total_size:{},path:{:?} \n", total_size, path);
    Ok(total_size)
}

// fn main() -> io::Result<()> {
//     // 假设我们要计算当前目录（.）的大小
//     let current_dir = Path::new(".");
//     match dir_size(current_dir) {
//         Ok(size) => {
//             // 将字节转换为MB或GB
//             let size_mb = size as f64 / 1024.0 / 1024.0; // 转换为MB
//             let size_gb = size as f64 / 1024.0 / 1024.0 / 1024.0; // 转换为GB

//             // 根据大小选择合适的单位进行打印
//             if size_gb >= 1.0 {
//                 println!("The size of the current directory is: {:.2} GB, {}", size_gb, current_dir.display(),);
//             } else {
//                 println!("The size of the current directory is: {:.2} MB, {}", size_mb, current_dir.display(),);
//             }
//         },
//         Err(e) => eprintln!("Error calculating directory size: {}", e),
//     }

//     Ok(())
// }
use std::fs;
// use std::io;
// use std::path::Path;
use prettytable::{row, Cell, Row, Table};
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // 假设我们要列出当前目录下的内容
    // let mut current_dir = ".";
    if args.len() > 1 {
        //         // 获取第二个参数
        //         let second_arg = &args[1];
        //         println!("The second argument is: {}", second_arg);
        let current_dir = &args[1];
        //         // 记录开始时间
        //         let start_time = Instant::now();

        //         // 获取文件的实际大小，而非硬盘上的大小
        //         let len = fs::metadata(&second_arg).unwrap().len();
        //         println!("len: {}", len);
        // 读取当前目录下的所有条目（文件和子目录）
        // 判断是否是文件还是文件夹

        // if let Ok(metadata) = fs::metadata(current_dir) {
        //     if metadata.is_dir() {
        //         println!("{} is a directory", current_dir);
        //     } else if metadata.is_file() {
        //         println!("{} is a file", current_dir);
        //     } else {
        //         println!("{} is something else", current_dir);
        //     }
        // } else {
        //     println!("{} does not exist", current_dir);
        // }
        let entries = fs::read_dir(current_dir)?;
        let mut table = Table::new();
        table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
        // 遍历这些条目
        for entry in entries {
            let entry = entry?;

            let name = entry.file_name().to_str().unwrap_or_default().to_string(); // 使用to_string_lossy来处理非UTF-8文件名
            let path = entry.path();
            let size_str: String;

            match dir_size(&path) {
                Ok(size) => {
                    // 将字节转换为MB或GB ,如果不到MB就直接以KB的形式打印
                    if size < 1024 {
                        size_str = format!("{:?} KB", size as f64);
                    } else if size < 1024 * 1024 {
                        size_str = format!("{:.2}  MB", size as f64 / 1024.0);
                        // println!("名称:{:?},大小{} MB", name, size / 1024);
                    } else if size < 1024 * 1024 * 1024 {
                        size_str = format!("{:.2}  GB", size as f64 / 1024.0 / 1024.0);
                        // println!("名称:{:?},大小{} GB", name, size / 1024 / 1024);
                    } else {
                        size_str = format!("{:.2}  TB", size as f64 / 1024.0 / 1024.0 / 1024.0);
                        // println!("名称:{:?},大小{} TB", name, size / 1024 / 1024 / 1024);
                    }
                    // print!("名称:{:?},大小{} \n", name, size_str);
                    table.add_row(Row::new(vec![Cell::new(&name), Cell::new(&size_str)]));
                }
                Err(e) => eprintln!("Error calculating directory size: {}", e),
            }
        }
        table.printstd();

    }

    Ok(())
}
// use std::{fs::File, io::BufRead, io::BufReader};

// use clap::Parser;

// /// search for a pattern in a file and display the lines that contain it
// #[derive(Parser)]
// struct Cli {
//     /// the pattern to look for
//     pattern: String,
//     /// the path to the file to read
//     path: std::path::PathBuf,
// }

// fn main() {
//     let args = Cli::parse();
//     let f = File::open(&args.path).expect("could not open file");
//     let reader = BufReader::new(f);
//     reader.lines().for_each(|line| {
//         if let Ok(line) = line {
//             if line.contains(&args.pattern) {
//                 println!("{}", line);
//             }
//         }
//     });
// }

// use anyhow::{Context, Result};
// use clap::Parser;

// /// my cli
// #[derive(Parser)]
// struct Cli {
//     /// my pattern
//     pattern: String,
//     /// path to search
//     path: std::path::PathBuf,
// }

// fn main() -> Result<()> {
//     let args = Cli::parse();
//     let content = std::fs::read_to_string(&args.path)
//         .with_context(|| format!("could not read file {:?}", &args.path))?;
//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line)
//         }
//     }
//     Ok(())
// }

// use std::thread;
// use std::time::Duration;
// use std::{cmp::min, fmt::Write};

// use indicatif::{ProgressBar, ProgressState, ProgressStyle};

// fn main() {
//     let mut downloaded = 0;
//     let total_size = 111010101;
//     // 创建进度条
//     let pb = ProgressBar::new(total_size);
//     // 设置进度条样式
//     pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
//         .unwrap()
//         .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
//         .progress_chars("#>-"));

//     // 模拟下载
//     while downloaded < total_size {
//         let new = min(downloaded + 1, total_size);
//         downloaded = new;
//         pb.set_position(new);
//         thread::sleep(Duration::from_millis(12));
//     }
//     // 下载完成
//     pb.finish_with_message("downloaded");
// }


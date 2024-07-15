use prettytable::{row, Cell, Row, Table};
use std::env;
use std::fs;
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
    Ok(total_size)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let current_dir = &args[1];
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
                    let s = 1024 * 1024 * 1024;
                    let b = s < size;
                    print!("{:?} < {:?},{:?}\n", size, s, b);
                    // 将字节转换为MB或GB ,如果不到MB就直接以KB的形式打印
                    if size < 1024 {
                        size_str = format!("{:?} KB", size as f64);
                        print!("名称:{:?},大小{} KB", name, size)
                    } else if size < 1024 * 1024 {
                        size_str = format!("{:.2}  MB", size as f64 / 1024.0);
                        println!("名称:{:?},大小{} MB", name, size / 1024);
                    } else if size < 1024 * 1024 * 1024 {
                        size_str = format!("{:.2}  GB", size as f64 / 1024.0 / 1024.0);
                        println!("名称:{:?},大小{} GB", name, size / 1024 / 1024);
                    } else {
                        size_str = format!("{:.2}  TB", size as f64 / 1024.0 / 1024.0 / 1024.0);
                        println!("名称:{:?},大小{} TB", name, size / 1024 / 1024 / 1024);
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

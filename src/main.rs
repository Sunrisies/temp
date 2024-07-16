use prettytable::{row, Cell, Row, Table};
use rayon::prelude::*;
use std::env;
use std::fs;
use std::fs::read_dir;
use std::io;
use std::path::Path;
// use std::{fs, io, path::Path};

// 递归地计算目录的大小
fn dir_size1(path: &Path) -> io::Result<u64> {
    let mut total_size = 0u64;

    if path.is_file() {
        return Ok(path.metadata()?.len());
    }
    // 尝试读取目录中的所有条目
    if let Ok(entries) = read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if let Ok(entry_type) = entry_path.metadata() {
                    if entry_type.is_file() {
                        total_size += entry_type.len();
                    } else if entry_type.is_dir() {
                        total_size += dir_size(&entry_path)?;
                    }
                }
            }
        }
    }
    Ok(total_size)
}
fn dir_size2(path: &Path) -> io::Result<u64> {
    let mut total_size = 0u64;

    if path.is_file() {
        return Ok(fs::metadata(path)?.len());
    }

    let entries = fs::read_dir(path)?;
    let mut subdirs = Vec::new();

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            subdirs.push(entry_path);
        }
    }

    // 使用 rayon 的 reduce 来并行求和子目录大小
    let subdirs_size = subdirs
        .par_iter()
        .map(|dir| dir_size(dir).map(|size| size as u128)) // 转换为 u128 以避免溢出
        .reduce(
            || Ok(0u128),
            |acc, size| acc.and_then(|a| size.map(|b| a + b)),
        )?;

    // 检查是否溢出
    total_size = if let Some(size) = subdirs_size.checked_add(total_size as u128) {
        size as u64
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "Total size overflow"));
    };
    Ok(total_size)
}

fn dir_size(path: &Path) -> io::Result<u64> {
    let mut total_size = 0u64;

    if path.is_file() {
        return Ok(path.metadata()?.len());
    }

    // 尝试读取目录中的所有条目
    if let Ok(entries) = fs::read_dir(path) {
        let mut dirs = vec![];
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if let Ok(entry_type) = entry_path.metadata() {
                    if entry_type.is_file() {
                        total_size += entry_type.len();
                    } else if entry_type.is_dir() {
                        dirs.push(entry_path);
                    }
                }
            }
        }

        // 并行处理子目录
        let subdir_sizes: io::Result<Vec<u64>> = dirs.par_iter().map(|dir| dir_size(dir)).collect();
        total_size += subdir_sizes?.par_iter().sum::<u64>();
    }

    Ok(total_size)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // 计时
        let start = std::time::Instant::now();
        let current_dir = &args[1];
        let entries = fs::read_dir(current_dir)?;
        let mut table = Table::new();
        table.add_row(row!["名称", "大小"]);
        for entry in entries {
            let entry = entry?;
            let name = entry.file_name().to_str().unwrap_or_default().to_string(); // 使用to_string_lossy来处理非UTF-8文件名
            let path = entry.path();
            match dir_size2(&path) {
                Ok(size) => {
                    let size_str = format_size(size);
                    table.add_row(Row::new(vec![Cell::new(&name), Cell::new(&size_str)]));
                }
                Err(e) => eprintln!("Error calculating directory size: {}", e),
            }
        }
        table.printstd();
        let end = std::time::Instant::now();
        println!("Time elapsed: {:.2?}", end - start);
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let size_f64 = bytes as f64;
    let unit: &str;
    let value: f64;

    if size_f64 >= TB {
        unit = "TB";
        value = size_f64 / TB;
    } else if size_f64 >= GB {
        unit = "GB";
        value = size_f64 / GB;
    } else if size_f64 >= MB {
        unit = "MB";
        value = size_f64 / MB;
    } else if size_f64 >= KB {
        unit = "KB";
        value = size_f64 / KB;
    } else {
        unit = "B";
        value = size_f64;
    }

    // 格式化输出，保留两位小数
    format!("{:.2} {}", value, unit)
}

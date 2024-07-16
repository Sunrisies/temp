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


fn main() {  
  let bytes = 1372160;  
  println!("Size: {}", format_size(bytes));  

  // 测试其他大小的字节  
  println!("Size of 1KB: {}", format_size(1024));  
  println!("Size of 1MB: {}", format_size(1024 * 1024));  
  println!("Size of 1GB: {}", format_size(1024 * 1024 * 1024));  
  println!("Size of 1TB: {}", format_size(1024 * 1024 * 1024 * 1024));  
}
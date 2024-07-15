use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
pub fn file_api() {
    let f = File::open("hello.txt").unwrap();
    let f = match f {
      Ok(file) => file,
      Err(error) => match error.kind() {
          ErrorKind::NotFound => match File::create("hello.txt") {
              Ok(fc) => fc,
              Err(e) => panic!("创建文件失败: {:?}", e),
          },
          other_error => panic!("打开文件失败: {:?}", other_error),
      },
  };
}

pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

pub fn vector_api() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    println!("{:?}", v);

    match v.get(2) {
        Some(x) => println!("The third element is {}", x),
        None => println!("There is no third element"),
    }
}

pub fn vector_api2() {
    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist1 = &v[100]; // 这个会提示错误
    let does_not_exist2 = v.get(100);
    // println!("does_not_exist1={:?}", does_not_exist1);
    println!("does_not_exist2={:?}", does_not_exist2);

    let mut v1 = vec![100, 32, 57];
    for i in &mut v1 {
        *i += 50;
    }
    println!("v1={:?}", v1);

    // 使用枚举来储存多种类型
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row={:?}", row);
}

pub fn string_api() {
    let mut s1 = String::from("hello");
    let s2 = "world";
    s1.push_str(s2);
    println!("s1={:?}", s1);

    // chars 返回的是每一个元素
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // bytes 返回的是每一个字节
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores={:?}", scores);
}

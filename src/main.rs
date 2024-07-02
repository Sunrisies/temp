// use open::that;
use std::process::Command;
fn main() {
    // 打开网页
    // that("https://cn.vitejs.dev/").expect("Failed to open browser");
    // println!("Hello, world!");
    // 根据操作系统执行不同的命令
    if cfg!(target_os = "windows") {
        Command::new("explorer")
            .arg(".")
            .spawn()
            .expect("Failed to open file manager on Windows");
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .arg(".")
            .spawn()
            .expect("Failed to open file manager on Linux");
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(".")
            .spawn()
            .expect("Failed to open file manager on macOS");
    } else {
        eprintln!("Unsupported operating system");
    }
    // 根据操作系统执行不同的命令 打开终端
    // if cfg!(target_os = "windows") {
    //     Command::new("cmd")
    //         .arg("/c")
    //         .arg("start")
    //         .arg("cmd.exe")
    //         .spawn()
    //         .expect("Failed to open terminal on Windows");
    // } else if cfg!(target_os = "linux") {
    //     Command::new("gnome-terminal")
    //         .arg("--")
    //         .arg("bash")
    //         .arg("-c")
    //         .arg("echo Hello, World!")
    //         .spawn()
    //         .expect("Failed to open terminal on Linux");
    // } else if cfg!(target_os = "macos") {
    //     Command::new("open")
    //         .arg("-a")
    //         .arg("Terminal")
    //         .arg("echo Hello, World!")
    //         .spawn()
    //         .expect("Failed to open terminal on macOS");
    // } else {
    //     eprintln!("Unsupported operating system");
    // }
}
// 第一步，先通过rust把数据保存到指定目录下面，然后通过读取数据的方式去读取数据
// 第二步，在页面上面显示数据，并且可以进行修改
// 第三步，实现数据的保存和读取功能
// 第四步，实现数据的修改功能
// 第五步，实现数据的删除功能
// 第六步，实现数据的搜索功能
// 第七步，根据搜索功能去匹配数据，如果有数据就直接打开浏览器，如果没有可以提示用户没有找到数据
// 第八步，根据搜索内容去匹配数据，如果是打开终端就直接打开终端，如果是打开文件管理器就打开文件管理器,
// 还要支持数据导入功能
// 此声明将会查找名为 `my.rs` 或 `my/mod.rs` 的文件，并将该文件的内容放到
// 此作用域中一个名为 `my` 的模块里面。
mod utils;

fn main() {

    // let sun = utils::utils::add(23, 32);
    // println!("sun is {}", sun);
    // utils::utils::vector_api();
    // utils::utils::vector_api2();
    // utils::utils::string_api();

    utils::utils::file_api();
}

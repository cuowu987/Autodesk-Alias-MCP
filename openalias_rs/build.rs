
fn main() {
    // AliasCore.lib 所在目录
    let lib_dir = r"..\bin";
    // 告诉 Rust 链接器搜索路径
    println!("cargo:rustc-link-search=native={}", lib_dir);
    // 链接库名（去掉前缀 lib 和 .lib）
    println!("cargo:rustc-link-lib=openAlias_C");
    println!("cargo:warning=Linking with openAlias_C.lib from {}", lib_dir);
}




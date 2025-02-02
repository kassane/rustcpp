fn main() {
    cxx_build::bridge("src/main.rs")
        .flag_if_supported("-std=c++20")
        .file("../cpp/cats.cpp")
        .compile("cats");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=../cpp/cats.cpp");
    println!("cargo:rerun-if-changed=../cpp/cats.hpp");
}

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/native/notation_engine.cpp")
        .flag_if_supported("-std=c++17")
        .compile("notation_engine_cpp");

    println!("cargo:rerun-if-changed=src/native/notation_engine.cpp");
    println!("cargo:rerun-if-changed=src/native/notation_engine.hpp");
}

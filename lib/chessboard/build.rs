
// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=src/cpp/*");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/cpp/BoardRep.cpp")
        .file("src/cpp/Move.cpp")
        .file("src/cpp/tables/move_tables.cpp")
        .file("src/cpp/tables/magics.cpp")
        .compile("cpp-cblib");
}

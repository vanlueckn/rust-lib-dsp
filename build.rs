use glob::glob;

fn main() {
    let mut builder = cc::Build::new();
    let build = builder.cpp(true);

    for entry in glob("libdsp/**/*.cpp").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            build.file(path);
        }
    }

    build.compile("libdsp");
}

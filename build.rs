use std::env;
use std::path::PathBuf;

struct XilVitisDriverProject {
    path: String,
}

impl XilVitisDriverProject {
    const DEFAULT_LINK_TYPE: &'static str = "static";
    const DEFAULT_DRIVER_ROOT: &'static str = "driver";
    const LIBRARY_NAME: &'static str = "vitisnetp4drv";
    const REQUIRED_HEADER_FILES: &'static [&'static str] = &[
        "bcam.h",
        "cue.h",
        "cam.h",
        "counter_extern.h",
        "vitisnetp4_target.h",
        "cam_shared.h",
        "vitis_net_p4_core_defs.h",
        "vitisnetp4_table_private.h",
        "cue_internal.h",
        "vitisnetp4_target_mgmt.h",
        "vitisnetp4_common.h",
        "vitisnetp4_table.h",
        "bf.h",
        "tiny_cam.h",
        "cam_private.h",
        "register_top.h",
        "tcam.h",
        "register.h",
        "cam_top.h",
        "stcam.h",
    ];

    fn new() -> Self {
        Self {
            path: env::var("DRIVER_ROOT").unwrap_or_else(|_| Self::DEFAULT_DRIVER_ROOT.to_string()),
        }
    }

    fn get_include_directory(&self) -> String {
        format!("{}/include", self.path)
    }

    fn get_lib_directory(&self) -> String {
        format!("{}/lib", self.path)
    }

    fn get_header_files(&self) -> Result<Vec<String>, std::io::Error> {
        let mut header_files = Vec::new();
        let include_directory = self.get_include_directory();
        for &header_name in Self::REQUIRED_HEADER_FILES {
            let header_path = PathBuf::from(&include_directory).join(header_name);
            let full_path = header_path.to_string_lossy().to_string();
            header_files.push(full_path.clone());
        }
        Ok(header_files)
    }

    fn get_static_library_file(&self) -> String {
        format!("{}/lib{}.a", self.get_lib_directory(), Self::LIBRARY_NAME)
    }

    fn get_dynamic_library_file(&self) -> String {
        format!("{}/lib{}.so", self.get_lib_directory(), Self::LIBRARY_NAME)
    }

    fn verify_directories(&self) {
        let include_directory = self.get_include_directory();
        let lib_directory = self.get_lib_directory();

        eprintln!("Verifying directories:");
        eprintln!("  Include directory: {}", include_directory);
        eprintln!("  Library directory: {}", lib_directory);

        if !PathBuf::from(&include_directory).exists() {
            panic!("Include directory not found: {}", include_directory);
        }
        if !PathBuf::from(&lib_directory).exists() {
            panic!("Library directory not found: {}", lib_directory);
        }

        eprintln!("Directory verification passed");
    }

    fn setup_library_linking(&self) {
        let link_type = env::var("LINK_TYPE").unwrap_or_else(|_| Self::DEFAULT_LINK_TYPE.to_string());

        eprintln!("Setting up library linking with type: {}", link_type);

        let (library_file, cargo_link_type) = match link_type.as_str() {
            "static" => (self.get_static_library_file(), "static"),
            "dynamic" => (self.get_dynamic_library_file(), "dylib"),
            _ => panic!("Invalid LINK_TYPE: {}. Use 'static' or 'dynamic'", link_type),
        };

        eprintln!("Looking for library file: {}", library_file);

        if PathBuf::from(&library_file).exists() {
            println!("cargo:rustc-link-lib={}={}", cargo_link_type, Self::LIBRARY_NAME);
            eprintln!("Library linking configured: {}={}", cargo_link_type, Self::LIBRARY_NAME);
        } else {
            panic!("{} library not found: {}", link_type, library_file);
        }
        println!("cargo:rustc-link-search=native={}", self.get_lib_directory());
    }

    fn setup_change_monitoring(&self) {
        eprintln!("Setting up change monitoring for: {}", self.path);
        println!("cargo:rerun-if-changed={}/", self.path);

        for &header_name in Self::REQUIRED_HEADER_FILES {
            let header_path = format!("{}/{}", self.get_include_directory(), header_name);
            println!("cargo:rerun-if-changed={}", header_path);
        }
    }
}

fn main() {
    eprintln!("=== XilVitis Driver Build Script Starting ===");

    let driver_project = XilVitisDriverProject::new();
    eprintln!("Driver project initialized with path: {}", driver_project.path);

    driver_project.verify_directories();

    let header_files = driver_project.get_header_files().expect("Failed to read header files");

    if header_files.is_empty() {
        panic!("No header files found in {}", driver_project.get_include_directory());
    }

    eprintln!("Building bindgen with {} header files", header_files.len());

    let mut builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}", driver_project.get_include_directory()))
        .clang_arg("-fparse-all-comments")
        .allowlist_function(".*")
        .allowlist_type(".*")
        .allowlist_var(".*")
        .derive_debug(true)
        .derive_default(true)
        .derive_copy(true)
        .derive_eq(true)
        .derive_hash(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    for header_file in header_files.iter() {
        eprintln!("Adding header file to bindgen: {}", header_file);
        builder = builder.header(header_file);
    }

    eprintln!("Generating bindings...");
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_file = out_path.join("bindings.rs");

    eprintln!("Writing bindings to: {:?}", bindings_file);
    bindings.write_to_file(&bindings_file).expect("Couldn't write bindings!");

    driver_project.setup_library_linking();
    driver_project.setup_change_monitoring();

    eprintln!("=== XilVitis Driver Build Script Completed Successfully ===");
}

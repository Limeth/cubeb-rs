/*!
  Thanks to Paul Adenot for the build script
 */
extern crate pkg_config;
extern crate submodules;

use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::process::Command;
use std::env;

fn check_command(cmd: &str) -> bool {
  return Command::new("which").arg(cmd)
                              .status()
                              .unwrap_or_else(|e| {
    panic!("Failed to execute command: {}", e)
  }).success();
}

fn append_to_cubeb_cmakelists(path: &Path) {
    const lines_to_add: &str = r#"# Start of auto-generated code by cubeb-rs
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fPIC")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fPIC")
# End of auto-generated code by cubeb-rs"#;
    let mut file = File::open(path).expect("Could not open CMakeLists.txt");
    let mut reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(err) => panic!("{}", err),
        }
    }
}

fn main()
{
  let build_cubeb = env::var("CARGO_FEATURE_BUILD_CUBEB").is_ok();
  let clean_cubeb = env::var("CARGO_FEATURE_CLEAN_CUBEB").is_ok();
  let target = env::var("TARGET").unwrap();
  let host = env::var("HOST").unwrap();

  if !build_cubeb {
    if target != host {
      panic!("For cross-builds use the 'build-cubeb' feature.");
    } else if !pkg_config::Config::new().find("cubeb").is_ok() {
      panic!("Missing libcubeb. Install it manually or build cubeb-rs with \
             '--features build-cubeb'.");
    }
    /* if using a pre-existing libcubeb, just link against it dynamically */
    println!("cargo:rustc-link-lib=dylib=cubeb");
    return
  }

  let out_dir = env::var("OUT_DIR").unwrap();

  let cubeb_dir = "cubeb";
  let cubeb_build_dir = "build";

  submodules::update().init().run();

  assert!(check_command("cmake"), "cmake missing!");
  assert!(check_command("ctest"), "ctest missing");

  assert!(env::set_current_dir(cubeb_dir).is_ok());

  assert!(Command::new("git").args(&["submodule", "update", "--init", "--recursive"])
                                    .status()
                                    .unwrap_or_else(|e| {
    panic!("Failed to execute command: {}", e);
  }).success(), "git exited with an error.");

  assert!(Command::new("mkdir").args(&["-p", "build"])
                                    .status()
                                    .unwrap_or_else(|e| {
    panic!("Failed to execute command: {}", e);
  }).success(), "mkdir exited with an error.");

  assert!(env::set_current_dir(cubeb_build_dir).is_ok());

  if clean_cubeb {
      assert!(Command::new("make").arg("clean")
                                     .status()
                                     .unwrap_or_else(|e| {
        panic!("Failed to execute command: {}", e);
      }).success(), "`make clean` exited with an error.");
  }

  let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let cmakelists = Path::new(&project_dir).join(cubeb_dir).join("CMakeLists.txt");

  append_to_cubeb_cmakelists(&cmakelists);

  assert!(Command::new("cmake").arg("..")
                                 .status()
                                 .unwrap_or_else(|e| {
    panic!("Failed to execute command: {}", e);
  }).success(), "`cmake ..` exited with an error.");

  assert!(Command::new("cmake").args(&["--build", "."])
                                 .status()
                                 .unwrap_or_else(|e| {
    panic!("Failed to execute command: {}", e);
  }).success(), "`cmake --build .` exited with an error.");

  // This gets annoying since it outputs sound, so I commented it out
  // assert!(Command::new("ctest").status()
  //                             .unwrap_or_else(|e| {
  //   panic!("Failed to execute command: {}", e);
  // }).success(), "ctest exited with an error.");

  println!("cargo:rustc-link-search=native={}/cubeb/build", project_dir);
  println!("cargo:rustc-link-lib=static=cubeb");
  println!("cargo:rustc-link-lib=stdc++");
  println!("cargo:rustc-link-lib=asound");
}

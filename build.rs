/*!
  Thanks to Paul Adenot for the build script
 */
extern crate pkg_config;
extern crate submodules;

use std::fs::OpenOptions;
use std::path::Path;
use std::io::{BufReader, BufRead, BufWriter, Write};
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
    const LINES_TO_ADD: &str = r#"# Start of auto-generated code by cubeb-rs
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fPIC")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fPIC")
# End of auto-generated code by cubeb-rs"#;
    let mut lines_to_add_iter = LINES_TO_ADD.lines();
    let mut current_line_to_add = lines_to_add_iter.next();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("Could not open CMakeLists.txt");

    {
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if line == current_line_to_add.unwrap() {
                        match lines_to_add_iter.next() {
                            Some(next_line_to_add) => current_line_to_add = Some(next_line_to_add),
                            None => return,
                        }
                    }
                },
                Err(err) => panic!("{}", err),
            }
        }
    }

    let mut writer = BufWriter::new(file);

    while current_line_to_add.is_some() {
        writeln!(writer, "{}", current_line_to_add.unwrap())
            .expect(&format!("Could not write the line '{}' to CMakeLists.txt", current_line_to_add.unwrap()));
        current_line_to_add = lines_to_add_iter.next();
    }

    writer.flush().expect("Could not update CMakeLists.txt");
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

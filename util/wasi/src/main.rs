use wasmer_runtime::{func, imports, instantiate};
use wasmer_wasi::{
    generate_import_object,
};
use wasiringrunner::*;



fn main() {
  let path = std::env::args().nth(1).expect("USAGE: wasiringrunner path/to/test.wasm");
  dbg!(&path);
  // Load the plugin data
  let wasm_bytes = std::fs::read(path.clone()).expect(&format!(
      "Could not read in WASM plugin at {}",
      path
  ));

  // WASI imports
  let mut base_imports = generate_import_object(vec![], vec![], vec![], vec![(".".to_owned(), ".".into())]);
  // env is the default namespace for extern functions
  let custom_imports = imports! {
      "env" => {
          "GFp_gcm_init_avx" => func!(gcm::__gfp_gcm_init_avx),
          "GFp_poly1305_blocks" => func!(poly1305::__gfp_poly1305_blocks),
      },
  };
  // The WASI imports object contains all required import functions for a WASI module to run.
  // Extend this imports with our custom imports containing "it_works" function so that our custom wasm code may run.
  base_imports.extend(custom_imports);
  let mut instance =
      instantiate(&wasm_bytes[..], &base_imports).expect("failed to instantiate wasm module");

  // get a reference to the function "plugin_entrypoint" which takes an i32 and returns an i32
  let entry_point = instance.func::<(), ()>("_start").unwrap();
  // call the "entry_point" function in WebAssembly with the number "2" as the i32 argument
  let result = entry_point.call().expect("failed to execute test");
}
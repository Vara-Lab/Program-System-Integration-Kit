use app::TransmitterProgram;
use sails_idl_gen::program;
use sails_client_gen::ClientGenerator;
use std::{env, path::PathBuf};

fn main() {
    gear_wasm_builder::build();

    // Path where the file "Cargo.toml" is located (points to the root of the project)
    // 'CARGO_MANIFEST_DIR' specifies this directory in en::var
    let cargo_toml_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Path where the file "app.idl" will be created
    let idl_path = cargo_toml_path.clone().join("app.idl");

    // This generate the contract IDL
    program::generate_idl_to_file::<TransmitterProgram>(idl_path.clone())
        .unwrap();

    // Generator of the clients of the contract
    ClientGenerator::from_idl_path(&idl_path)
        .with_mocks("with_mocks")
        .generate_to(cargo_toml_path.join("app_client.rs"))
        .unwrap();
}

use config_struct;
use config_struct::StructOptions;
use serde;
use serde::Deserialize;
use serde_yaml;
use serde_yaml::{Deserializer, Value};
use std::fs::File;

// =============================================================================
// You configure your Step4 with a 'vendor.yml' file.
// =============================================================================
#[derive(Deserialize)]
/// The target we are building to.
struct ConfigTarget {
    /// Determines what 'stand' we use. All around determines what asm is used.
    t0: String,

    /// Determines peripherals.
    t1: String,

    /// Determines more peripherals. We could use a passed DTB too.
    t2: String,
}

#[derive(Deserialize)]
/// Main config to pull in.
struct Config {
    /// See: `ConfigTarget`.
    target: ConfigTarget,
}

fn main() {
    let file = File::open("vendor.yml").expect(
        "\n\n*** You need to make a 'vendor.yml' file. Please make one to build Step4. ***\n\n",
    );
    let de = serde_yaml::Deserializer::from_reader(file);
    let config =
        Config::deserialize(de).expect("\n\n*** Your 'vendor.yml' file is malformed. ***\n\n");

    // call it a day, see: https://github.com/rust-lang/rfcs/issues/2259
    // also errors should NEVER happen here, it'd be a bitflip or some skid
    // writing to vendor.yml while we use it.
    config_struct::create_struct("vendor.yml", "src/vendor.rs", &StructOptions::default()).unwrap();

    let t0: &str = &conrfig.target.t0;
    match t0 {
        "armv7" => {}
        _ => {panic!("Target `{}` is unsupported at the moment.", t0)}
    };
}

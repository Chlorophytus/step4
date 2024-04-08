use chrono::Datelike;
use core::panic;
use serde;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml::Deserializer;

// =============================================================================
// You configure your Step4 with a 'vendor.toml' file.
// =============================================================================
#[derive(Deserialize)]
/// Bootup data
struct ConfigBoot {
    /// Microkernel linker script
    linker_script: String,

    /// Hardware subtype
    hw_type: String,
}

#[derive(Deserialize)]
/// The serial debug output's options
struct ConfigUART {
    /// The base address of the UART
    base_address: u64,
}

#[derive(Deserialize)]
/// Main config to pull in.
struct Config {
    /// See: `ConfigBoot`
    boot: ConfigBoot,
    /// See: `ConfigUART`.
    uart: ConfigUART,
}

fn main() {
    let mut vendor_toml = String::new();

    File::open("vendor.toml").expect(
        "\n\n*** You need to make a 'vendor.toml' file. Please make one to build Step4. ***\n\n",
    ).read_to_string(&mut vendor_toml).unwrap();

    // Deserializers are mutating
    let de = toml::Deserializer::new(&vendor_toml);
    let config: Config =
        Config::deserialize(de).expect("\n\n*** Your 'vendor.toml' file is malformed. ***\n\n");

    let build_date = chrono::offset::Utc::now();
    let build_time: u32 = ((build_date.day() & 0b11111) << 0)
        | ((build_date.month() & 0b1111) << 5)
        | (((build_date.year() as u32 - 2000) & 0b1111111) << 9);
    println!("cargo::rustc-env=S4BUILDTIME={}", build_time);

    let version = env!("CARGO_PKG_VERSION")
        .split(".")
        .map(|v| str::parse::<u32>(v).unwrap())
        .collect::<Vec<u32>>();
    println!(
        "cargo::rustc-env=S4BUILDVERSION={}",
        ((version[0] & 0xFF) << 24) | ((version[1] & 0xFF) << 16) | ((version[2] & 0xFF) << 2)
    );

    println!("cargo::rustc-link-lib=s4support");
    println!("cargo::rustc-cfg=s4hwtype=\"{}\"", config.boot.hw_type);
    println!(
        "cargo::rustc-env=S4UARTADDRESS={}",
        config.uart.base_address
    );
    let mut build = cc::Build::new();
    match std::env::var("TARGET").unwrap().as_str() {
        "armv7a-none-eabi" => {
            println!("cargo::rustc-cfg=s4arch=\"armv7a\"");
            build
            .file("src/support/device_tree.S")
                .file("src/support/armv7a.S")
                .compile("s4support");
            println!("cargo::rerun-if-changed={}", "src/support/armv7a.S");
        }
        unsupported => {
            panic!("Target `{}` is unsupported at the moment.", unsupported);
        }
    };
    println!("cargo::rerun-if-changed={}", config.boot.linker_script);
    println!(
        "cargo::rustc-link-arg=--script={}",
        config.boot.linker_script
    );
}

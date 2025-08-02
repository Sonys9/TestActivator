//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
//#![allow(rustdoc::missing_crate_level_docs)] 
use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::env;
use std::fs;
//use std::io;
use winreg::RegKey;
use cab;
use winreg::enums::*;

const SYSTEM32: &str = "C:\\Windows\\System32";
const KEYS: [&str; 13] = [
    "XGVPP-NMH47-7TTHJ-W3FW7-8HV2C",
    "YNMGQ-8RYV3-4PGQ3-C8XTP-7CFBY",
    "8PTT6-RNW4C-6V7J2-C2D3X-MHBPB",
    "DXG7C-N36C4-C4HTG-X4T3X-2YV77",
    "FWN7H-PF93Q-4GGP8-M8RF3-MDWWW",
    "NK96Y-D9CD8-W44CQ-R8YTK-DYJWX",
    "43TBQ-NH92J-XKTM7-KT3KK-P39PB",
    "YTMG3-N6DKC-DKB77-7M9GH-8HVX7",
    "YTMG3-N6DKC-DKB77-7M9GH-8HVX7",
    "XQQYW-NFFMW-XJPBH-K8732-CKFFD",
    "QPM6N-7J2WJ-P88HH-P3YRH-YY74H",
    "CGK42-GYN6Y-VD22B-BX98W-J8JXD",
    "VK7JG-NPHTM-C97JM-9MPGT-3V66T"
];
const FILE_NEW_BYTES: [(usize, u8); 66] = [
    (320, 0xf8),
    (321, 0xfb),
    (322, 0x05),
    (324, 0x03),
    (13672, 0x25),
    (13674, 0x73),
    (13676, 0x3b),
    (13678, 0x00),
    (13680, 0x00),
    (13682, 0x00),
    (13684, 0x00),
    (32748, 0xe9),
    (32749, 0x9e),
    (32750, 0x00),
    (32751, 0x00),
    (32752, 0x00),
    (32894, 0x8b),
    (32895, 0x44),
    (32897, 0x64),
    (32898, 0x85),
    (32899, 0xc0),
    (32900, 0x0f),
    (32901, 0x85),
    (32902, 0x1c),
    (32903, 0x02),
    (32904, 0x00),
    (32906, 0xe9),
    (32907, 0x3c),
    (32908, 0x01),
    (32909, 0x00),
    (32910, 0x00),
    (32911, 0x85),
    (32912, 0xdb),
    (32913, 0x75),
    (32914, 0xeb),
    (32915, 0xe9),
    (32916, 0x69),
    (32917, 0xff),
    (32918, 0xff),
    (32919, 0xff),
    (33094, 0xe9),
    (33095, 0x80),
    (33096, 0x00),
    (33097, 0x00),
    (33098, 0x00),
    (33449, 0x64),
    (33576, 0x8d),
    (33577, 0x54),
    (33579, 0x24),
    (33580, 0xe9),
    (33581, 0x55),
    (33582, 0x01),
    (33583, 0x00),
    (33584, 0x00),
    (33978, 0xc3),
    (34189, 0x59),
    (34190, 0xeb),
    (34191, 0x28),
    (34238, 0xe9),
    (34239, 0x4f),
    (34240, 0x00),
    (34241, 0x00),
    (34242, 0x00),
    (34346, 0x24),
    (34376, 0xeb),
    (34377, 0x63),
];

fn first_activate() {
    let binding = env::temp_dir();
    let TEMP: &str = binding.to_str().unwrap();
    fs::create_dir_all(format!("{}\\PlayerokTool", TEMP)).unwrap();
    for key in KEYS.iter() {
        let _ = Command::new(format!("{}\\cscript.exe", SYSTEM32))
            .arg(format!("//nologo {}\\slmgr.vbs /ipk {}", SYSTEM32, key));
    };
    let data = reqwest::blocking::get("https://download.microsoft.com/download/9/A/E/9AE69DD5-BA93-44E0-864E-180F5E700AB4/adk/Installers/14f4df8a2a7fc82a4f415cf6a341415d.cab").unwrap().bytes().unwrap();
    let mut file = File::create(format!("{}\\PlayerokTool\\data.cab", TEMP)).unwrap();
    let _ = file.write_all(&data);
    drop(file);
    let cab_file = File::open(format!("{}\\data.cab", TEMP)).unwrap();
    let mut cabinet = cab::Cabinet::new(cab_file).unwrap();
    for folder in cabinet.folder_entries() {
        for file in folder.file_entries() {
            if file.name() != "filf8377e82b29deadca67bc4858ed3fba9" { 
                continue; 
            };
            let mut output_file = File::create(format!("{}\\data.exe", TEMP)).unrwap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            output_file.write_all(&buffer).unwrap();
        };
    };
    let mut file_bytes: Vec<u8> = fs::read(
        format!("{}\\PlayerokTool\\data.exe", TEMP)
    ).unwrap();

    for new_bytes in FILE_NEW_BYTES.iter() {
        file_bytes[new_bytes.0] = new_bytes.1;
    };

    let mut file = File::create(format!("{}\\PlayerokTool\\GOS.exe", TEMP)).unwrap();
    let _ = file.write_all(&file_bytes);
    drop(file);

    let hkml = RegKey::predef(HKEY_LOCAL_MACHINE);
    let product_options = hkml.open_subkey("SYSTEM\\CurrentControlSet\\Control\\ProductOptions").unwrap();
    let os_product_pfn: String = product_options.get_value("OSProductPfn").unwrap();

    let _ = Command::new(format!("{}\\PlayerokTool\\GOS.exe", TEMP))
        .arg(
            format!("/c Pfn={}`;PKeyIID=465145217131314304264339481117862266242033457260311819664735280",
                os_product_pfn    
            )
        );
    
    let _ = Command::new(format!("{}\\clipup.exe", SYSTEM32))
        .arg(
            format!("-v -o -altto {}\\PlayerokTool",
                TEMP
            )
        );
    
    let _ = Command::new(format!("{}\\cscript.exe", SYSTEM32))
            .arg(
                format!("//nologo {}\\slmgr.vbs /ato",
                    SYSTEM32
                )
            );
}

fn main() { 
    println!("Активируем...");
    first_activate();
}


use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str::from_utf8;

pub struct CartridgeReader {
    nintendo_graphics_code: [u8; 48],
    cartridge: [u8; 65536],
}

impl CartridgeReader {
    pub fn new() -> CartridgeReader {
        let nintendo_graphics_code = [
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        let cartridge = [0; 65536];
        CartridgeReader {
            nintendo_graphics_code,
            cartridge,
        }
    }
    fn check_ninteno_code(&mut self) {
        let mut successes: u8 = 0;
        for n in 0x0104..0x0134 {
            if self.nintendo_graphics_code[n - 0x0104] == self.cartridge[n] {
                successes += 1
            }
        }
        println!(
            "Succesfully passed {} out of 48 of Nintendo graphics code bytes.",
            successes
        );
    }
    fn get_cartridge_name(&mut self) {
        let s = from_utf8(&self.cartridge[0x0134..0x143]).unwrap();
        println!("Name of cartridge: {}", s);
    }
    fn determine_gbc(&mut self) {
        if self.cartridge[0x0143] == 0x80 {
            println!("This cartridge is Gameboy Color.");
        } else {
            println!("This cartridge is Gameboy.");
        }
    }
    fn determine_sgb(&mut self) {
        if self.cartridge[0x0146] == 0x03 {
            println!("This cartridge enables Super Gameboy.");
        } else {
            println!("This cartridge does not enable Super Gameboy.");
        }
    }
    fn get_cartridge_type(&mut self) {
        let mut phrase = "The cartridge type is ".to_string();
        match self.cartridge[0x0147] {
            0x0 => phrase.push_str("ROM ONLY."),
            0x1 => phrase.push_str("ROM + MBC1."),
            0x2 => phrase.push_str("ROM + MBC1 + RAM."),
            0x3 => phrase.push_str("ROM + MBC1 + RAM + BATT."),
            0x5 => phrase.push_str("ROM + MBC2."),
            0x6 => phrase.push_str("ROM + MBC2  BATT."),
            0x8 => phrase.push_str("ROM + RAM."),
            0x9 => phrase.push_str("ROM + RAM + BATT."),
            0xB => phrase.push_str("ROM + MMM01."),
            0xC => phrase.push_str("ROM + MMM01 + SRAM."),
            0xD => phrase.push_str("ROM + + MMM01 + SRAM + BATT."),
            0xF => phrase.push_str("ROM + MBC3 + TIMER + BATT."),
            0x10 => phrase.push_str("ROM + MBC3 + TIMER + RAM + BATT."),
            0x11 => phrase.push_str("ROM + MBC3."),
            0x12 => phrase.push_str("ROM + MBC3 + RAM."),
            0x13 => phrase.push_str("ROM + MBC3 + RAM + BATT."),
            0x19 => phrase.push_str("ROM + MBC5."),
            0x1A => phrase.push_str("ROM + MBC5 + RAM."),
            0x1B => phrase.push_str("ROM + MBC5 + RAM + BATT."),
            0x1C => phrase.push_str("ROM + MBC5 + RUMBLE."),
            0x1D => phrase.push_str("ROM + MBC5 + RUMBLE + SRAM."),
            0x1E => phrase.push_str("ROM + MBC5 + RUMBLE + SRAM + BATT."),
            0x1F => phrase.push_str("Pocket Camera."),
            0xFD => phrase.push_str("Bandai TAMA5"),
            0xFE => phrase.push_str("Hudson HuC - 3"),
            0xFF => phrase.push_str("Hudson HuC - 1"),
            _ => phrase.push_str("ERROR, Cartridge type not recognized"),
        }
        println!("{}", phrase);
    }
    fn get_rom_size(&mut self) {
        let mut phrase = "The ROM size is ".to_string();
        match self.cartridge[0x0148] {
            0x0 => phrase.push_str("32KB (2 banks)"),
            0x1 => phrase.push_str("64KB (4 banks)"),
            0x2 => phrase.push_str("128KB (8 banks)"),
            0x3 => phrase.push_str("256KB (16 banks)"),
            0x4 => phrase.push_str("512KB (32 banks)"),
            0x5 => phrase.push_str("1MB (64 banks)"),
            0x6 => phrase.push_str("2MB (128 banks)"),
            0x52 => phrase.push_str("1.1MB (72 banks)"),
            0x53 => phrase.push_str("1.2MB (80 banks)"),
            0x54 => phrase.push_str("1.5MB (96 banks)"),
            _ => phrase.push_str("ERROR, ROM size not recognized"),
        }
        println!("{}", phrase);
    }
    fn get_ram_size(&mut self) {
        let mut phrase = "The RAM size is ".to_string();
        match self.cartridge[0x0149] {
            0x0 => phrase.push_str("0KB (no RAM)"),
            0x1 => phrase.push_str("2KB (1 bank)"),
            0x2 => phrase.push_str("8KB (1 bank)"),
            0x3 => phrase.push_str("32KB (4 banks)"),
            0x4 => phrase.push_str("128KB (16 banks)"),
            _ => phrase.push_str("ERROR, RAM size not recognized"),
        }
        println!("{}", phrase);
    }
    fn get_destination_code(&mut self) {
        let mut phrase = "The destination code is ".to_string();
        match self.cartridge[0x014A] {
            0x0 => phrase.push_str("Japanese"),
            0x1 => phrase.push_str("Non-Japanese"),
            _ => phrase.push_str("ERROR, destination code  not recognized"),
        }
        println!("{}", phrase);
    }
    fn get_licensee_code(&mut self) {
        let mut phrase = "The licensee code is ".to_string();
        match self.cartridge[0x014B] {
            0x33 => phrase.push_str(from_utf8(&self.cartridge[0x0144..0x146]).unwrap()),
            0x79 => phrase.push_str("Accolade"),
            0xA4 => phrase.push_str("Konami"),
            _ => phrase.push_str("ERROR, licensee code not recognized"),
        }
        println!("{}", phrase);
    }
    fn get_complement_check(&mut self) {
        println!("The complement check is {:x?}", self.cartridge[0x14D]);
    }
    fn get_checksum(&mut self) {
        println!("The checksum is {:x?}", &self.cartridge[0x14E..0x150]);
    }
    fn load_memory(&mut self, path: &str) {
        let mut f = File::open(path).expect("File problem!");
        f.read(&mut self.cartridge).expect("Read issue!");
    }
    pub fn get_info(&mut self, path: &str) {
        self.load_memory(path);
        self.check_ninteno_code();
        self.get_cartridge_name();
        self.determine_gbc();
        self.determine_sgb();
        self.get_cartridge_type();
        self.get_rom_size();
        self.get_ram_size();
        self.get_destination_code();
        self.get_licensee_code();
        self.get_complement_check();
        self.get_checksum();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cr = CartridgeReader::new();
    cr.get_info(&args[1]);
}

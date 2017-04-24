use std::rc::Rc;
use std::cell::RefCell;

pub struct CPUMemoryMap {
    pub ppu: ::ppu::PPU,
    pub ram: Vec<u8>,
    // input
    pub cart: Rc<RefCell<::cartridge::Cartridge>>,
}

// For now I'm just gonna assume NROM-128 because I'm just focusing on Donkey Kong
impl CPUMemoryMap {
    pub fn new(cart: Rc<RefCell<::cartridge::Cartridge>>, ppu: ::ppu::PPU) -> CPUMemoryMap {
        CPUMemoryMap{ram: vec![0; 2048], cart: cart, ppu: ppu}
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            // 2k of ram repeated 4 times
            0 ... 0x1fff => {
                let data = self.ram[address as usize % 0x800];
                //println!("Read {:x} from {:x}", data, address as usize % 0x800);

                data
            },

            0x2000 ... 0x3fff => {
                let modaddr = address % 8;
                println!("Read from PPU register: {}", modaddr);
                match modaddr {
                    0 => self.ppu.read_control_1(),
                    1 => self.ppu.read_control_2(),
                    2 => self.ppu.read_status(),
                    3 => self.ppu.read_oamaddr(),
                    4 => self.ppu.read_unknown4(),
                    5 => self.ppu.read_scroll_offset(),
                    6 => self.ppu.read_addr_offset(),
                    7 => self.ppu.read_unknown7(),
                    _ => panic!("Mod 8 cannot return anything greater than 7 but somehow it did")
                }
            },

            0x4000 ... 0x4017 => {
                panic!("0x4000 ... 0x4017");
            },

            0x4018 ... 0x401F => {
                panic!("0x4018 ... 0x401F");
            },

            // PRG-RAM
            0x6000 ... 0x7FFF => {
                panic!("PRG-RAM");
            },

            // First 16k of ROM
            0x8000 ... 0xBFFF => {
                let cart = self.cart.borrow();
                cart.rom[address as usize - 0x8000]
            }

            // Last 16k of ROM (just a mirror for NROM-128)
            0xC000 ... 0xFFFF => {
                // REMEMBER THIS IS A MIRROR OF THE PREVIOUS BECAUSE NROM 128
                let cart = self.cart.borrow();
                cart.rom[address as usize - 0xC000]
            }
            _ => 0
        }
    }

    pub fn write(&mut self, data: u8, address: u16) {
        match address {
            // 2k of ram repeated 4 times
            0 ... 0x1fff => {
                //println!("Writing {} at {}", data, address);
                self.ram[address as usize % 0x800] = data;
                //println!("Wrote {:x} at {:x}", self.ram[address as usize % 0x800], address as usize % 0x800);
            },

            0x2000 ... 0x3fff => {
                let modaddr = address % 7;
                //println!("Write to PPU register: {}", modaddr);
            },

            0x4000 ... 0x4017 => {
                panic!("0x4000 ... 0x4017");
            },

            0x4018 ... 0x401F => {
                panic!("0x4018 ... 0x401F");
            },

            // PRG-RAM
            0x6000 ... 0x7FFF => {
                panic!("PRG-RAM");
            },

            // First 16k of ROM
            0x8000 ... 0xBFFF => {
                panic!("Attempt to write to ROM");
            }

            // Last 16k of ROM (just a mirror for NROM-128)
            0xC000 ... 0xFFFF => {
                // REMEMBER THIS IS A MIRROR OF THE PREVIOUS BECAUSE NROM 128
                panic!("Attempt to write to ROM");
            }
            _ => panic!("Whelp shit")
        }
    }
}

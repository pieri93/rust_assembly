use std::io::Read;

fn disassemble(buffer: &[u8]) -> usize {
    match get_inst(buffer[0]) {
        instr::MovRegReg => {
            let w = buffer[0] & 0b00000001;
            let d = (buffer[0] >> 1) & 0b00000001;
            let mod_ = (buffer[1] >> 6) & 0b00000011;
            let mut reg = match mod_ {
                0b11 => get_reg(w, (buffer[1] >> 3) & 0b00000111).to_string(),
                0b00 => get_reg_mod00((buffer[1]) & 0b00000111).to_string(),
                0b01 => get_reg_mod01((buffer[1]) & 0b00000111, buffer[2]).to_string(),
                0b10 => get_reg_mod10(
                    (buffer[1]) & 0b00000111,
                    to_sixteen_bit_int(buffer[2], buffer[3]),
                )
                .to_string(),
                _ => panic!("Invalid mod"),
            };
            let mut reg1 = get_reg(w, buffer[1] & 0b00000111).to_string();
            if d == 1 {
                // We are swapping the registers because that's what the manual says
                std::mem::swap(&mut reg, &mut reg1);
            }
            println!("MOV {}, {}", reg1, reg);
            match buffer[1] >> 6 {
                0b11 => {
                    // match get_dw(buffer[0]) {
                    //     0b00 => println!(
                    //         "MOV {}, {}",
                    //         get_reg_w0(buffer[1] & 0b00000111),
                    //         get_reg_w0((buffer[1] >> 3) & 0b00000111)
                    //     ),
                    //     0b10 => println!(
                    //         "MOV {}, {}",
                    //         get_reg_w0((buffer[1] >> 3) & 0b00000111),
                    //         get_reg_w0(buffer[1] & 0b00000111)
                    //     ),
                    //     0b01 => println!(
                    //         "MOV {}, {}",
                    //         get_reg_w1(buffer[1] & 0b00000111),
                    //         get_reg_w1((buffer[1] >> 3) & 0b00000111)
                    //     ),
                    //     0b11 => println!(
                    //         "MOV {}, {}",
                    //         get_reg_w1((buffer[1] >> 3) & 0b00000111),
                    //         get_reg_w1(buffer[1] & 0b00000111)
                    //     ),

                    //     dw => panic!("{dw:2b}, is an invalid dw"),
                    // };
                }

                0b00 => return 2,
                0b01 => {
                    return 3;
                }
                0b10 => {
                    return 4;
                }
                _ => todo!(),
            }
            return 2;
        }
        instr::MovInmReg => {
            let reg = buffer[0] & 0b00000111;
            let w = (buffer[0] >> 3) & 0b00000001;
            if w == 1 {
                let data = to_sixteen_bit_int(buffer[1], buffer[2]);
                println!("MOV {}, {}", get_reg_w1(reg), data);
                return 3;
            }
            let data = buffer[1] as i8;
            println!("MOV {}, {}", get_reg_w0(reg), data);
            return 2;
        }
    }
}

enum instr {
    MovRegReg,
    MovInmReg,
}
fn to_sixteen_bit_int(low: u8, high: u8) -> i16 {
    ((high as i16) << 8) | low as i16
}
fn get_inst(input: u8) -> instr {
    // Check instructions of the first 6 bits
    match input >> 2 {
        0b100010 | 0b110001 => return instr::MovRegReg,
        _ => (),
    }
    match input >> 4 {
        0b1011 => return instr::MovInmReg,
        _ => (),
    }
    todo!("This instruction has not been handled")
}

fn get_dw(buffer: u8) -> u8 {
    buffer & 0b00000011
}

fn get_reg(w: u8, input: u8) -> &'static str {
    match w {
        1 => get_reg_w1(input),
        0 => get_reg_w0(input),
        _ => panic!("Invalid w"),
    }
}

fn get_reg_mod00(rm: u8) -> &'static str {
    match rm {
        0b000 => "[bx + si]",
        0b001 => "[bx + di]",
        0b010 => "[bp + si]",
        0b011 => "[bp + di]",
        0b100 => "[si]",
        0b101 => "[di]",
        0b110 => todo!("Not yet implemented direct address"),
        0b111 => "[bx]",
        _ => panic!("Invalid w"),
    }
}

fn get_reg_mod01(rm: u8, d8: u8) -> String {
    match rm {
        0b000 => format!("[bx + si + {d8}]"),
        0b001 => format!("[bx + di + {d8}]"),
        0b010 => format!("[bp + si + {d8}]"),
        0b011 => format!("[bp + di + {d8}]"),
        0b100 => format!("[si + {d8}]"),
        0b101 => format!("[di + {d8}]"),
        0b110 => format!("[bp + {d8}]"),
        0b111 => format!("[bx + {d8}]"),
        _ => panic!("Invalid w"),
    }
}

fn get_reg_mod10(rm: u8, d16: i16) -> String {
    match rm {
        0b000 => format!("[bx + si + {d16}]"),
        0b001 => format!("[bx + di + {d16}]"),
        0b010 => format!("[bp + si + {d16}]"),
        0b011 => format!("[bp + di + {d16}]"),
        0b100 => format!("[si + {d16}]"),
        0b101 => format!("[di + {d16}]"),
        0b110 => format!("[bp + {d16}]"),
        0b111 => format!("[bx + {d16}]"),
        _ => panic!("Invalid w"),
    }
}
fn get_reg_w1(input: u8) -> &'static str {
    match input {
        0b000 => "AX",
        0b001 => "CX",
        0b010 => "DX",
        0b011 => "BX",
        0b100 => "SP",
        0b101 => "BP",
        0b110 => "SI",
        0b111 => "DI",
        reg => panic!("{reg:3b}, is an invalid register"),
    }
}

fn get_reg_w0(input: u8) -> &'static str {
    match input {
        0b000 => "AL",
        0b001 => "CL",
        0b010 => "DL",
        0b011 => "BL",
        0b100 => "AH",
        0b101 => "CH",
        0b110 => "DH",
        0b111 => "BH",
        reg => panic!("{reg:3b}, is an invalid register"),
    }
}
fn main() {
    let mut buffer = Vec::new();

    let file_name = "../computer_enhance/perfaware/part1/listing_0039_more_movs";
    let mut file = std::fs::File::open(file_name).expect("file must exist");

    let mut read_bytes = file.read_to_end(&mut buffer).expect("unable to read file");
    let mut i = 0;
    while i < read_bytes {
        i += disassemble(&buffer[i..]);
    }
}

use std::io::Read;

fn disassemble(buffer: &[u8]) -> usize {
    match get_inst(buffer[0]) {
        instr::MovRegReg => {
            let w = buffer[0] & 0b00000001;
            let d = (buffer[0] >> 1) & 0b00000001;
            let mod_ = (buffer[1] >> 6) & 0b00000011;
            let mut reg = get_reg(w, (buffer[1] >> 3) & 0b00000111).to_string();
            let mut rm = match mod_ {
                0b11 => get_reg(w, (buffer[1]) & 0b00000111).to_string(),
                0b00 => get_reg_mod00((buffer[1]) & 0b00000111).to_string(),
                0b01 => get_reg_mod01((buffer[1]) & 0b00000111, buffer[2]).to_string(),
                0b10 => get_reg_mod10(
                    (buffer[1]) & 0b00000111,
                    to_sixteen_bit_int(buffer[2], buffer[3]),
                )
                .to_string(),
                _ => panic!("Invalid mod"),
            };
            if d == 1 {
                // We are swapping the registers because that's what the manual says
                println!("MOV {}, {}", reg, rm);
                // std::mem::swap(&mut reg, &mut reg1);
            } else {
                println!("MOV {}, {}", rm, reg);
            }

            match buffer[1] >> 6 {
                0b11 => {}

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
    let plus_suffix = add_plus_if_not_zero(d8 as i16);
    match rm {
        0b000 => format!("[bx + si{plus_suffix}]"),
        0b001 => format!("[bx + di{plus_suffix}]"),
        0b010 => format!("[bp + si{plus_suffix}]"),
        0b011 => format!("[bp + di{plus_suffix}]"),
        0b100 => format!("[si{plus_suffix}]"),
        0b101 => format!("[di{plus_suffix}]"),
        0b110 => format!("[bp{plus_suffix}]"),
        0b111 => format!("[bx{plus_suffix}]"),
        _ => panic!("Invalid w"),
    }
}

fn add_plus_if_not_zero(value: i16) -> String {
    if value == 0 {
        return (format!(""));
    } else {
        return (format!(" + {value}"));
    }
}

fn get_reg_mod10(rm: u8, d16: i16) -> String {
    let plus_suffix = add_plus_if_not_zero(d16);
    match rm {
        0b000 => format!("[bx + si{plus_suffix}]"),
        0b001 => format!("[bx + di{plus_suffix}]"),
        0b010 => format!("[bp + si{plus_suffix}]"),
        0b011 => format!("[bp + di{plus_suffix}]"),
        0b100 => format!("[si{plus_suffix}]"),
        0b101 => format!("[di{plus_suffix}]"),
        0b110 => format!("[bp{plus_suffix}]"),
        0b111 => format!("[bx{plus_suffix}]"),
        _ => panic!("Invalid w"),
    }
}
fn get_reg_w1(input: u8) -> &'static str {
    match input {
        0b000 => "ax",
        0b001 => "cx",
        0b010 => "dx",
        0b011 => "bx",
        0b100 => "sp",
        0b101 => "bp",
        0b110 => "si",
        0b111 => "di",
        reg => panic!("{reg:3b}, is an invalid register"),
    }
}

fn get_reg_w0(input: u8) -> &'static str {
    match input {
        0b000 => "al",
        0b001 => "cl",
        0b010 => "dl",
        0b011 => "bl",
        0b100 => "ah",
        0b101 => "ch",
        0b110 => "dh",
        0b111 => "bh",
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

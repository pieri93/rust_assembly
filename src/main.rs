use std::io::Read;

fn disassemble(buffer: &[u8]) -> usize {
    match get_inst(buffer[0]) {
        instr::MovRegReg => {
            let w = buffer[0] & 0b00000001;
            let reg = get_reg(w, (buffer[1] >> 3) & 0b00000111);
            match buffer[1] >> 6 {
                0b11 => {
                    match get_dw(buffer[0]) {
                        0b00 => println!(
                            "MOV {}, {}",
                            get_reg_w0(buffer[1] & 0b00000111),
                            get_reg_w0((buffer[1] >> 3) & 0b00000111)
                        ),
                        0b10 => println!(
                            "MOV {}, {}",
                            get_reg_w0((buffer[1] >> 3) & 0b00000111),
                            get_reg_w0(buffer[1] & 0b00000111)
                        ),
                        0b01 => println!(
                            "MOV {}, {}",
                            get_reg_w1(buffer[1] & 0b00000111),
                            get_reg_w1((buffer[1] >> 3) & 0b00000111)
                        ),
                        0b11 => println!(
                            "MOV {}, {}",
                            get_reg_w1((buffer[1] >> 3) & 0b00000111),
                            get_reg_w1(buffer[1] & 0b00000111)
                        ),

                        dw => panic!("{dw:2b}, is an invalid dw"),
                    };
                }

                0b00 => match buffer[1] & 0b00000111 {
                    0b000 => {
                        println!("MOV {}, [bx + si]", reg);
                    }
                    0b011 => {
                        println!("MOV {}, [bp + di]", reg);
                    }
                    _ => todo!(),
                },
                0b01 => {
                    let data = buffer[2];
                    match buffer[1] & 0b00000111 {
                        0b110 => match data {
                            0b0 => println!("MOV {}, [bp]", reg),
                            data => println!("MOV {}, [bp + {}]", reg, data),
                        },
                        0b011 => {
                            println!("MOV {}, [bp + di]", reg);
                        }
                        _ => todo!(),
                    }
                    return 3;
                }
                _ => todo!(),
            }

            return 2;
        }
        instr::MovInmReg => {
            let reg = buffer[0] & 0b00000111;
            let w = (buffer[0] >> 3) & 0b00000001;
            if w == 1 {
                let data = ((buffer[2] as i16) << 8) | buffer[1] as i16;
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

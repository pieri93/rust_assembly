use std::io::Read;

fn disassemble(buffer: [u8; 2]) {
    match get_dw(buffer[0]) {
        0b00 => println!(
            "{} {}, {}",
            get_inst((buffer[0] >> 2) & 0b00111111),
            get_reg_w0(buffer[1] & 0b00000111),
            get_reg_w0((buffer[1] >> 3) & 0b00000111)
        ),
        0b10 => println!(
            "{} {}, {}",
            get_inst((buffer[0] >> 2) & 0b00111111),
            get_reg_w0((buffer[1] >> 3) & 0b00000111),
            get_reg_w0(buffer[1] & 0b00000111)
        ),
        0b01 => println!(
            "{} {}, {}",
            get_inst((buffer[0] >> 2) & 0b00111111),
            get_reg_w1(buffer[1] & 0b00000111),
            get_reg_w1((buffer[1] >> 3) & 0b00000111)
        ),
        0b11 => println!(
            "{} {}, {}",
            get_inst((buffer[0] >> 2) & 0b00111111),
            get_reg_w1((buffer[1] >> 3) & 0b00000111),
            get_reg_w1(buffer[1] & 0b00000111)
        ),
        dw => panic!("{dw:2b}, is an invalid dw"),
    }
}

fn get_inst(input: u8) -> &'static str {
    match input {
        0b100010 | 0b110001 => "MOV",
        inst => panic!("{inst:6b}, is an invalid instruction"),
    }
}

fn get_dw(buffer: u8) -> u8 {
    buffer & 0b00000011
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
    let mut buffer = [0u8; 2];

    let file_name = "../computer_enhance/perfaware/part1/listing_0038_many_register_mov";
    let mut file = std::fs::File::open(file_name).expect("file must exist");

    let mut read_bytes = file.read(&mut buffer).expect("unable to read file");
    while read_bytes > 0 {
        disassemble(buffer);
        read_bytes = file.read(&mut buffer).expect("unable to read file");
    }
}

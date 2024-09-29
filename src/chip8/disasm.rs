use std::ops::Shr;

pub fn disasm_chip_8_op(memory: &[u8], pc: usize) {
    let first_part = memory[pc];
    let second_part = memory[pc + 1];
    let quad_1: u8 = first_part.shr(4);
    let quad_2: u8 = first_part & 0x0F;
    let quad_3: u8 = second_part.shr(4);
    let quad_4: u8 = second_part & 0x0F;
    let address = ((quad_2 as u16) << 8) | (second_part as u16);
    let number = quad_3 << 4 | quad_4;

    match quad_1 {
        0x00 => match address {
            0x00E0 => println!("CLS"),
            0x00EE => println!("RET"),
            _ => println!("SYS addr"),
        },
        0x01 => println!("JMP {:0x}", address),
        0x02 => println!("CALL {:0x}", address),
        0x03 => println!("SKIP IF V{:x} == {:0x}", address, number),
        0x04 => println!("SKIP IF V{:x} != {:0x}", address, number),
        0x05 => println!("SKIP IF V{:x} == V{:x}", quad_2, quad_3),
        0x06 => println!("STORE V{:x}, {:0x}", quad_2, number),
        0x07 => println!("ADD V{:x}, {:0x}", quad_2, number),
        0x08 => match quad_4 {
            0x01 => println!("OR V{:x}, V{:x}", quad_2, quad_3),
            0x02 => println!("AND V{:x}, V{:x}", quad_2, quad_3),
            0x03 => println!("XOR V{:x}, V{:x}", quad_2, quad_3),
            0x04 => println!("ADD V{:x}, V{:x}; VF = C", quad_2, quad_3),
            0x05 => println!("SUB V{:x}, V{:x}; VF = C", quad_2, quad_3),
            0x06 => println!("SHR V{:x}, V{:x}, 1; VF = LSB(V{1:x})", quad_2, quad_3),
            0x07 => println!("STORE V{:x}, V{:x} - V{0:x}, VF = C", quad_2, quad_3),
            0x0E => println!("SHL V{:x}, V{:x}, 1; VF = MSB(V{1:x})", quad_2, quad_3),
            _ => println!("UNSUPPORTED INSTRUCTION"),
        },
        0x09 => println!("SKIP IF V{:x} != V{:x}", quad_2, quad_3),
        0x0A => println!("STORE I, {:0x}", address),
        0x0B => println!("JMP {:0x} + V0", address),
        0x0C => println!("STORE V{:x}, RAND() & {:0x}", quad_2, number),
        0x0D => println!("DRAW V{:x}, V{:x}, {:0x}", quad_2, quad_3, quad_4),
        0x0E => match number {
            0x9E => println!("SKIP IF IS_PUSHED(V{:x})", quad_2),
            0xA1 => println!("SKIP IF !IS_PUSHED(V{:x})", quad_2),
            _ => println!("USNUPPORTED INSTRUCTION"),
        },
        0xF => match number {
            0x07 => println!("STORE V{:x}, DELAY_TIMER", quad_2),
            0x0A => println!("AWAIT KEY V{:x}", quad_2),
            0x15 => println!("STORE DELAY_TIMER, V{:x}", quad_2),
            0x18 => println!("STORE SOUND_TIMER, V{:x}", quad_2),
            0x1E => println!("ADD I, V{:x}", quad_2),
            0x29 => println!("STORE I, sprite_addr[V{:x}]", quad_2),
            0x33 => println!("STORE [I], [I+1],[I+1], BCD(V{:x})", quad_2),
            0x55 => println!(
                "REG DUMP FROM V0..=V{:x} to [I]; I = I + {0:0x} + 1",
                quad_2
            ),
            0x65 => println!(
                "REG LOAD FROM V0..=V{:x} from [I]; I = I + {0:0x} + 1",
                quad_2
            ),
            _ => println!("UNSUPPORTED INSTRUCTION"),
        },
        _ => todo!("No supported yet"),
    }
}

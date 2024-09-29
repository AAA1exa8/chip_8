use std::ops::Shr;

pub fn disasm_chip_8_op(memory: &[u8], pc: usize) -> String {
    let first_part = memory[pc];
    let second_part = memory[pc + 1];
    let whole = (first_part as u16) << 8 | second_part as u16;
    let quad_1: u8 = first_part.shr(4);
    let quad_2: u8 = first_part & 0x0F;
    let quad_3: u8 = second_part.shr(4);
    let quad_4: u8 = second_part & 0x0F;
    let address = ((quad_2 as u16) << 8) | (second_part as u16);
    let number = quad_3 << 4 | quad_4;

    match quad_1 {
        0x00 => match address {
            0x00E0 => return format!("CLS"),
            0x00EE => return format!("RET"),
            _ => return format!("SYS addr: {:0x}", address),
        },
        0x01 => return format!("JMP {:0x}", address),
        0x02 => return format!("CALL {:0x}", address),
        0x03 => return format!("SKIP IF V{:x} == {:0x}", address, number),
        0x04 => return format!("SKIP IF V{:x} != {:0x}", address, number),
        0x05 => return format!("SKIP IF V{:x} == V{:x}", quad_2, quad_3),
        0x06 => return format!("STORE V{:x}, {:0x}", quad_2, number),
        0x07 => return format!("ADD V{:x}, {:0x}", quad_2, number),
        0x08 => match quad_4 {
            0x01 => return format!("OR V{:x}, V{:x}", quad_2, quad_3),
            0x02 => return format!("AND V{:x}, V{:x}", quad_2, quad_3),
            0x03 => return format!("XOR V{:x}, V{:x}", quad_2, quad_3),
            0x04 => return format!("ADD V{:x}, V{:x}; VF = C", quad_2, quad_3),
            0x05 => return format!("SUB V{:x}, V{:x}; VF = C", quad_2, quad_3),
            0x06 => return format!("SHR V{:x}, V{:x}, 1; VF = LSB(V{1:x})", quad_2, quad_3),
            0x07 => return format!("STORE V{:x}, V{:x} - V{0:x}, VF = C", quad_2, quad_3),
            0x0E => return format!("SHL V{:x}, V{:x}, 1; VF = MSB(V{1:x})", quad_2, quad_3),
            _ => return format!("UNSUPPORTED INSTRUCTION: {:0x}", whole),
        },
        0x09 => return format!("SKIP IF V{:x} != V{:x}", quad_2, quad_3),
        0x0A => return format!("STORE I, {:0x}", address),
        0x0B => return format!("JMP {:0x} + V0", address),
        0x0C => return format!("STORE V{:x}, RAND() & {:0x}", quad_2, number),
        0x0D => return format!("DRAW V{:x}, V{:x}, {:0x}", quad_2, quad_3, quad_4),
        0x0E => match number {
            0x9E => return format!("SKIP IF IS_PUSHED(V{:x})", quad_2),
            0xA1 => return format!("SKIP IF !IS_PUSHED(V{:x})", quad_2),
            _ => return format!("USNUPPORTED INSTRUCTION: {:0x}", whole),
        },
        0xF => match number {
            0x07 => return format!("STORE V{:x}, DELAY_TIMER", quad_2),
            0x0A => return format!("AWAIT KEY V{:x}", quad_2),
            0x15 => return format!("STORE DELAY_TIMER, V{:x}", quad_2),
            0x18 => return format!("STORE SOUND_TIMER, V{:x}", quad_2),
            0x1E => return format!("ADD I, V{:x}", quad_2),
            0x29 => return format!("STORE I, sprite_addr[V{:x}]", quad_2),
            0x33 => return format!("STORE [I], [I+1],[I+1], BCD(V{:x})", quad_2),
            0x55 => {
                return format!(
                    "REG DUMP FROM V0..=V{:x} to [I]; I = I + {0:0x} + 1",
                    quad_2
                )
            }
            0x65 => {
                return format!(
                    "REG LOAD FROM V0..=V{:x} from [I]; I = I + {0:0x} + 1",
                    quad_2
                )
            }
            _ => return format!("UNSUPPORTED INSTRUCTION: {:0x}", whole),
        },
        _ => todo!("No supported yet"),
    }
}

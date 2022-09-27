use crate::generated::transaction_info::owned::CompiledInstruction;
use bebop::{Record, SubRecord};

mod generated;

fn main() {
    let mut buf = Vec::with_capacity(CompiledInstruction::MIN_SERIALIZED_SIZE);
    let instruction = CompiledInstruction {
        program_id_index: 3,
        accounts: vec![4_u8, 3, 2],
        data: vec![1_u8, 1, 1],
    };

    serialize_instruction(&instruction, &mut buf);

    let deserialized_instruction = deserialize_instruction(&buf);

    assert_eq!(instruction, deserialized_instruction);
}

fn serialize_instruction(instruction: &CompiledInstruction, buf: &mut Vec<u8>) {
    instruction.serialize(buf).unwrap();
}

fn deserialize_instruction(buf: &[u8]) -> CompiledInstruction {
    CompiledInstruction::deserialize(buf).unwrap()
}

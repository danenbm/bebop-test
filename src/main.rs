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

    instruction.serialize(&mut buf).unwrap();

    let deserialized_instruction = CompiledInstruction::deserialize(&buf).unwrap();

    assert_eq!(instruction, deserialized_instruction);
}

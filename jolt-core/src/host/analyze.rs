use std::{collections::HashMap, fs::File, io, path::PathBuf};

use serde::{Deserialize, Serialize};
use tracer::{ELFInstruction, JoltDevice, RVTraceRow, RV32IM};

use crate::jolt::vm::{bytecode::BytecodeRow, rv32i_vm::RV32I};

use common::{constants::MEMORY_OPS_PER_INSTRUCTION, rv_trace::MemoryOp};

#[derive(Clone, Serialize, Deserialize)]
pub struct ProgramSummary {
    pub raw_trace: Vec<RVTraceRow>,

    pub bytecode: Vec<ELFInstruction>,
    pub memory_init: Vec<(u64, u8)>,

    pub io_device: JoltDevice,
    pub bytecode_trace: Vec<BytecodeRow>,
    pub instruction_trace: Vec<Option<RV32I>>,
    pub memory_trace: Vec<[MemoryOp; MEMORY_OPS_PER_INSTRUCTION]>,
    pub circuit_flags: Vec<bool>,
}

impl ProgramSummary {
    pub fn trace_len(&self) -> usize {
        self.memory_trace.len()
    }

    pub fn analyze(&self) -> Vec<(RV32IM, usize)> {
        let mut counts = HashMap::<RV32IM, usize>::new();
        for row in self.raw_trace.iter() {
            let op = row.instruction.opcode;
            if let Some(count) = counts.get(&op) {
                counts.insert(op, count + 1);
            } else {
                counts.insert(op, 1);
            }
        }

        let mut counts: Vec<_> = counts.into_iter().collect();
        counts.sort_by_key(|v| v.1);
        counts.reverse();

        counts
    }

    pub fn write_to_file(self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(path)?;
        let data = bincode::serialize(&self)?;
        io::Write::write_all(&mut file, &data)?;
        Ok(())
    }
}

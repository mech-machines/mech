extern crate crossbeam_channel;
use mech_core::{hash_string, TableIndex, Table, Value, ValueType, ValueMethods, Transaction, Change, TableId, Register};
use mech_utilities::{Machine, MechCode, MachineRegistrar, RunLoopMessage};
//use std::sync::mpsc::{self, Sender};
use std::thread::{self};
use crossbeam_channel::Sender;
use std::collections::HashMap;

lazy_static! {
  static ref MECH_COMPILE: u64 = hash_string("mech/compile");
  static ref CODE: u64 = hash_string("code");
}

export_machine!(mech_compile, mech_compile_reg);

extern "C" fn mech_compile_reg(registrar: &mut dyn MachineRegistrar, outgoing: Sender<RunLoopMessage>) -> String {
  registrar.register_machine(Box::new(Compile{outgoing}));
  "#mech/compile = [|code|]".to_string()
}

#[derive(Debug)]
pub struct Compile {
  outgoing: Sender<RunLoopMessage>,
}

impl Machine for Compile {

  fn name(&self) -> String {
    "mech/compile".to_string()
  }

  fn id(&self) -> u64 {
    Register{table_id: TableId::Global(*MECH_COMPILE), row: TableIndex::All, column: TableIndex::All}.hash()
  }

  fn on_change(&mut self, table: &Table) -> Result<(), String> {
    for i in 1..=table.rows {
      let code = table.get_string(&TableIndex::Index(i), &TableIndex::Alias(*CODE));
      match code {
        Some((code,_)) => {
          let outgoing = self.outgoing.clone();
          outgoing.send(RunLoopMessage::Code((0,MechCode::String(code.to_string()))));
        }
        _ => (), // TODO Send error
      }
    }
    Ok(())
  }
}
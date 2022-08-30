extern crate crossbeam_channel;
use mech_core::*;
use mech_utilities::*;
use std::thread::{self};
use crossbeam_channel::Sender;
use std::collections::HashMap;

lazy_static! {
  static ref MECH_COMPILE: u64 = hash_str("mech/compile");
  static ref CODE: u64 = hash_str("code");
}

export_machine!(mech_compile, mech_compile_reg);

extern "C" fn mech_compile_reg(registrar: &mut dyn MachineRegistrar, outgoing: Sender<RunLoopMessage>) -> String {
  registrar.register_machine(Box::new(Compile{outgoing}));
  "#mech/compile = [|code<string>|]".to_string()
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
    hash_str(&self.name())
  }

  fn on_change(&mut self, table: &Table) -> Result<(), MechError> {
    for i in 1..=table.rows {
      let code = table.get(&TableIndex::Index(i), &TableIndex::Alias(*CODE))?;
      match code {
        Value::String(code) => {
          let outgoing = self.outgoing.clone();
          outgoing.send(RunLoopMessage::Code(MechCode::String(code.to_string())));
        }
        _ => (), // TODO Send error
      }
    }
    Ok(())
  }
}
use crate::lib::command::Command;

pub struct CommandGroup<'cg> {
  pub name: String,
  pub description: String,
  pub commands: Vec<&'cg (Command + Send + Sync)>,
}

use serenity::prelude::*;
use serenity::model::channel::Message;

pub trait Command {
  fn run(&self, ctx: &Context, msg: &Message, args: Vec<String>) -> Result<(), ()>;
  /* CONFIG FUNCTIONS */
  fn name(&self) -> String;
  fn aliases(&self) -> Vec<String> {
    vec![]
  }
  fn description(&self) -> String {
    String::from("")
  }
  fn examples(&self) -> Vec<String> {
    vec![]
  }
  fn format(&self) -> String {
    String::from("")
  }
  fn owner_only(&self) -> bool {
    false
  }
  fn min_args(&self) -> Option<usize> {
    None
  }
  fn max_args(&self) -> Option<usize> {
    None
  }
  fn arg_split_count(&self) -> ArgSplit {
    ArgSplit::None
  }
}

pub enum ArgSplit {
  None,
  Some(usize),
  All,
}

use command_engine::*;
use command_engine::shared::{Instruction, Output};

pub struct Debug;

impl CommandInfo for Debug {
    fn caller(&self) -> &str {
        "debug"
    }
}

#[async_trait]
impl Command for Debug {
    async fn on_execute(&self, _ins: Instruction) -> Output {
        Output::new_ok(0, Some("ok"))
    }
}

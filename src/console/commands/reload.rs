use crate::handler::reload_resource_map;
use super::*;

pub struct Reload;

impl CommandInfo for Reload {
    fn caller(&self) -> &str {
        "reload"
    }
}

#[async_trait]
impl Command for Reload {
    async fn on_execute(&self, ins: Instruction) -> Output {
        let args = ins.get_args();

        let arg_operation = get_arg!(args, 0, Output::new_error(1, Some("missing target argument [reload <target>]")));

        match arg_operation.as_str() {
            "frontend" => {
                reload_resource_map().await;
                Output::new_ok(0, Some("ok"))
            }
            unknown => Output::new_error(2, Some(format!(
                "unknown target argument ({})", unknown,
            ))),
        }
    }
}

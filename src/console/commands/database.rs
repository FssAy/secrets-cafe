use crate::database::Database;
use crate::database::types::ModTier;
use super::*;

pub struct Mod;

impl CommandInfo for Mod {
    fn caller(&self) -> &str {
        "mod"
    }
}

#[async_trait]
impl Command for Mod {
    async fn on_execute(&self, ins: Instruction) -> Output {
        let args = ins.get_args();

        let arg_operation = get_arg!(
            args,
            0,
            Output::new_error(1, Some("missing operation argument [ex: mod add]"))
        );

        match arg_operation.as_str() {
            "add" => {
                let db: Database = match Database::get().await {
                    Ok(database) => database,
                    Err(err) => return Output::new_error(4, Some(err.to_string())),
                };

                let name = get_arg!(args, 1, Output::new_error(1, Some("missing mod name [mod add <name> <pass> <tier>]")));
                let pass = get_arg!(args, 2, Output::new_error(1, Some("missing mod pass [mod add <name> <pass> <tier>]")));

                let tier = match ModTier::try_from(
                    get_arg!(args, 3, Output::new_error(1, Some("missing mod tier [mod add <name> <pass> <tier>]")))
                ) {
                    Ok(tier) => tier,
                    Err(_) => {
                        return Output::new_error(3, Some("invalid tier"))
                    }
                };

                match db.create_mod(name, pass, tier).await {
                    Ok(mod_id) => Output::new_ok(0, Some(format!("Added new mod with id: {}", mod_id))),
                    Err(err) => Output::new_error(4, Some(format!("{:?}", err))),
                }
            },
            unknown => Output::new_error(2, Some(format!(
                "unknown operation argument ({})", unknown,
            ))),
        }
    }
}

mod commands;

use command_engine::{Engine, EngineBuilder};
use tokio::io::{AsyncBufReadExt, BufReader, stdin};

pub struct Console {
    engine: Engine,
}

impl Console {
    pub async fn new() -> Result<Self, command_engine::shared::error::Error> {
        let engine = EngineBuilder::new()
            .help_caller("help")
            .build()?;

        let console = Self {
            engine,
        };

        console.add_commands().await?;

        Ok(console)
    }

    async fn add_commands(&self) -> Result<(), command_engine::shared::error::Error> {
        #[cfg(debug_assertions)]
        self.engine.add(commands::debug::Debug).await?;

        self.engine.add(commands::Mod).await?;

        Ok(())
    }

    pub fn start(self) {
        tokio::spawn(self.run());
    }

    async fn run(self) {
        let mut input = String::new();
        let mut reader = BufReader::new(stdin());

        info!("Console input initialized!");

        while let Ok(_) =  reader.read_line(&mut input).await {
            let instruction = input.trim();

            match self.engine.execute(instruction).await {
                Ok(output) => {
                    let status = output.result.status_code();
                    if output.result.is_ok() {
                        info!("[{:X}] {}", status, output.message);
                    } else {
                        error!("[{:X}] {}", status, output.message);
                    }
                },
                Err(err) => error!("[ENGINE ERROR] {}", err),
            }

            input.clear();
        }

        error!("Console input has been disabled!");
    }
}

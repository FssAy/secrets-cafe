///! This module adds a console commands system where the user with access to the console can input
///! custom commands.
///!
///! It is mainly used to manage server's resources.

mod commands;

use command_engine::{Engine, EngineBuilder};
use tokio::io::{AsyncBufReadExt, BufReader, stdin};

/// Wrapper for the command engine.
///
/// This helps with building the console commands system.
pub struct Console {
    engine: Engine,
}

impl Console {
    /// Creates a new static command engine.
    ///
    /// # Errors
    /// Will return an error when executed twice, or when the `Self::add_commads` function adds duplicated commands.
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

    /// Adds the console commands.
    ///
    /// # Errors
    /// Make sure to add only unique commands as adding a duplicate will result in an error.
    async fn add_commands(&self) -> Result<(), command_engine::shared::error::Error> {
        #[cfg(debug_assertions)]
        self.engine.add(commands::debug::Debug).await?;
        self.engine.add(commands::Mod).await?;
        self.engine.add(commands::Reload).await?;

        Ok(())
    }

    /// Starts the command engine on a new tokio task.
    pub fn start(self) {
        tokio::spawn(self.run());
    }

    /// Runs the command engine and blocks until the stdin is closed.
    ///
    /// Make sure to run it on a separate task as it will never return in normal conditions.
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

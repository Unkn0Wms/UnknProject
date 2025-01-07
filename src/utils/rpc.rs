use std::{sync::mpsc, thread};

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

pub struct Rpc {
    sender: mpsc::Sender<RpcUpdate>,
}

pub enum RpcUpdate {
    Update {
        state: Option<String>,
        details: Option<String>,
    },
    Shutdown,
}

impl Rpc {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut client = match DiscordIpcClient::new("1323284477618946078") {
                Ok(mut c) => match c.connect() {
                    Ok(_) => Some(c),
                    Err(e) => {
                        log::error!("Failed to connect to Discord RPC: {}", e);
                        None
                    }
                },
                Err(e) => {
                    log::error!("Failed to create Discord RPC client: {}", e);
                    None
                }
            };

            let mut current_state = String::new();
            let mut current_details = String::new();

            loop {
                match rx.recv() {
                    Ok(RpcUpdate::Update { state, details }) => {
                        if let Some(s) = state {
                            current_state = s;
                        }
                        if let Some(d) = details {
                            current_details = d;
                        }
                        if let Some(c) = &mut client {
                            if let Err(e) = c.set_activity(
                                activity::Activity::new()
                                    .state(&current_state)
                                    .details(&current_details)
                                    .assets(activity::Assets::new().large_image("logo")),
                            ) {
                                log::error!("Failed to set Discord RPC activity: {}", e);
                            }
                        }
                    }
                    Ok(RpcUpdate::Shutdown) => break,
                    Err(e) => {
                        log::error!("RPC channel error: {}", e);
                        break;
                    }
                }
            }

            if let Some(mut c) = client {
                if let Err(e) = c.close() {
                    log::error!("Failed to close Discord RPC connection: {}", e);
                }
            }
        });

        Rpc { sender: tx }
    }

    pub fn update(&self, state: Option<&str>, details: Option<&str>) {
        let update = RpcUpdate::Update {
            state: state.map(|s| s.to_string()),
            details: details.map(|d| d.to_string()),
        };

        if let Err(e) = self.sender.send(update) {
            log::error!("Failed to send RPC update: {}", e);
        }
    }
}

impl Drop for Rpc {
    fn drop(&mut self) {
        let _ = self.sender.send(RpcUpdate::Shutdown);
    }
}

use std::{
    path::PathBuf,
    process::Command,
    sync::{mpsc::Sender, Arc},
    thread,
    time::Duration,
};

use dll_syringe::{process::OwnedProcess, Syringe};
use eframe::egui::{self};

use crate::{utils::downloader::download_file, Hack, MyApp};

impl MyApp {
    pub fn delete_injectors(&mut self, arch: &str) -> Result<(), String> {
        let injectors = match arch {
            "both" => vec!["unknproject.exe", "unknproject.exe"],
            "x86" => vec!["unknproject.exe"],
            "x64" => vec!["unknproject.exe"],
            _ => return Err("Invalid architecture specified".to_string()),
        };

        for injector in injectors {
            let injector_path = dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("unknprojects")
                .join(injector);

            if injector_path.exists() {
                if let Err(e) = std::fs::remove_file(&injector_path) {
                    log::error!("Failed to delete {} injector: {}", injector, e);
                    return Err(format!("Failed to delete {} injector: {}", injector, e));
                }
                log::info!("Deleted {}", injector);
            }
        }
        Ok(())
    }

    pub fn inject(
        &mut self,
        dll_path: Option<std::path::PathBuf>,
        target_process: &str,
        message_sender: Sender<String>,
    ) {
        if let Some(process) = OwnedProcess::find_first_by_name(target_process) {
            let syringe = Syringe::for_process(process);
            let dll_path_clone = dll_path.clone().unwrap();
            if let Err(e) = syringe.inject(dll_path.unwrap()) {
                let _ = message_sender.send(format!("Failed to inject: {}", e));
                log::error!("Failed to inject: {}", e);
            } else {
                let _ = message_sender.send(format!(
                    "SUCCESS: {}",
                    dll_path_clone.file_name().unwrap().to_string_lossy()
                ));
                log::info!("Injected into {}", target_process);
            }
        } else {
            let _ = message_sender.send(format!("Process '{}' not found.", target_process));
            log::error!("Process '{}' not found.", target_process);
        }
    }

    pub fn manual_map_inject(
        &mut self,
        dll_path: Option<std::path::PathBuf>,
        target_process: &str,
        message_sender: Sender<String>,
    ) {
        let dll_path_clone = dll_path.clone().unwrap();
        let is_cs2 = target_process.eq_ignore_ascii_case("cs2.exe");
        let injector_process = if is_cs2 {
            "unknproject.exe"
        } else {
            "unknproject.exe"
        };

        log::debug!("Using {} injector", if is_cs2 { "x64" } else { "x86" });

        let file_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("unknproject")
            .join(injector_process);

        if !file_path.exists() {
            match download_file(injector_process, file_path.to_str().unwrap()) {
                Ok(_) => {
                    log::debug!("Downloaded manual map injector");
                }
                Err(e) => {
                    let _ = message_sender
                        .send(format!("Failed to download manual map injector: {}", e));
                    log::error!("Failed to download manual map injector: {}", e);
                    return;
                }
            }
        }

        let output = Command::new(file_path)
            .arg(target_process)
            .arg(dll_path.unwrap())
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout_message = String::from_utf8_lossy(&output.stdout).to_string();
                    log::info!("Manual map injector output (stdout): {}", stdout_message);
                    let _ = message_sender.send(format!(
                        "SUCCESS: {}",
                        dll_path_clone.file_name().unwrap().to_string_lossy()
                    ));
                    log::info!("Injected into {}", target_process);
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr).to_string();
                    let formatted_error_message = error_message
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .chunks(7)
                        .map(|chunk| chunk.join(" "))
                        .collect::<Vec<String>>()
                        .join("\n");

                    let _ = message_sender.send(formatted_error_message.clone());
                    log::info!("Failed to execute injector: {}", formatted_error_message);
                }
            }
            Err(e) => {
                let _ = message_sender.send(format!("Failed to execute injector: {}", e));
                log::error!("Failed to execute injector: {}", e);
            }
        }
    }

    pub fn start_injection(
        &mut self,
        selected: Hack,
        ctx: egui::Context,
        message_sender: Sender<String>,
    ) {
        let inject_in_progress = Arc::clone(&self.communication.inject_in_progress);
        let status_message = Arc::clone(&self.communication.status_message);
        let selected_clone = selected.clone();
        let ctx_clone = ctx.clone();
        let skip_injects_clone = self.app.config.skip_injects_delay.clone();

        {
            let mut status = status_message.lock().unwrap();
            *status = "Starting injection...".to_string();
        }

        inject_in_progress.store(true, std::sync::atomic::Ordering::SeqCst);

        thread::spawn(move || {
            ctx_clone.request_repaint();
            if !skip_injects_clone {
                thread::sleep(Duration::from_secs(1));
            }

            if !selected_clone.file_path.exists() {
                {
                    let mut status = status_message.lock().unwrap();
                    *status = "Downloading...".to_string();
                }
                ctx_clone.request_repaint();

                match selected_clone
                    .download(selected_clone.file_path.to_string_lossy().to_string())
                {
                    Ok(_) => {
                        let mut status = status_message.lock().unwrap();
                        *status = "Downloaded.".to_string();
                        log::debug!("Downloaded {}", selected_clone.name);
                        ctx_clone.request_repaint();
                    }
                    Err(e) => {
                        let mut status = status_message.lock().unwrap();
                        *status = format!("{}", e);
                        inject_in_progress.store(false, std::sync::atomic::Ordering::SeqCst);
                        log::error!("Failed to download: {}", e);
                        ctx_clone.request_repaint();
                        let _ = message_sender.send(format!("Failed to inject: {}", e));
                    }
                }
            }

            if !skip_injects_clone {
                thread::sleep(Duration::from_secs(1));
            }

            {
                let mut status = status_message.lock().unwrap();
                *status = "Injecting...".to_string();
            }

            ctx_clone.request_repaint();

            if !skip_injects_clone {
                thread::sleep(Duration::from_secs(1));
            }

            if let Some(target_process) = OwnedProcess::find_first_by_name(&selected_clone.process)
            {
                let syringe = Syringe::for_process(target_process);
                if let Err(e) = syringe.inject(selected_clone.file_path.clone()) {
                    let mut status = status_message.lock().unwrap();
                    *status = format!("Failed to inject: {}", e);
                    ctx_clone.request_repaint();
                    log::error!("Failed to inject: {}", e);
                    let _ = message_sender.send(format!("Failed to inject: {}", e));
                    Ok::<(), ()>(())
                } else {
                    let mut status = status_message.lock().unwrap();
                    *status = "Injection successful.".to_string();
                    inject_in_progress.store(false, std::sync::atomic::Ordering::SeqCst);
                    ctx_clone.request_repaint();
                    log::info!("Injected {}", selected_clone.name);
                    let _ = message_sender
                        .send(format!("SUCCESS: {}", selected_clone.name).to_string());
                    Ok(())
                }
            } else {
                let mut status = status_message.lock().unwrap();
                *status = format!(
                    "Failed to inject: Process '{}' not found.",
                    selected_clone.process
                );
                inject_in_progress.store(false, std::sync::atomic::Ordering::SeqCst);
                ctx_clone.request_repaint();
                log::error!("Process '{}' not found.", selected_clone.process);
                let _ =
                    message_sender.send(format!("Process '{}' not found.", selected_clone.process));
                Ok(())
            }
        });
    }

    // MARK: Manual map injection
    pub fn manual_map_injection(
        &mut self,
        selected: Hack,
        ctx: egui::Context,
        message_sender: Sender<String>,
    ) {
        let inject_in_progress = Arc::clone(&self.communication.inject_in_progress);
        let status_message = Arc::clone(&self.communication.status_message);
        let selected_clone = selected.clone();
        let ctx_clone = ctx.clone();
        let skip_inject_delay = self.app.config.skip_injects_delay;

        {
            let mut status = status_message.lock().unwrap();
            *status = "Starting injection...".to_string();
        }

        inject_in_progress.store(true, std::sync::atomic::Ordering::SeqCst);

        thread::spawn(move || {
            ctx_clone.request_repaint();
            if !skip_inject_delay {
                thread::sleep(Duration::from_secs(1));
            }

            if !selected_clone.file_path.exists() {
                {
                    let mut status = status_message.lock().unwrap();
                    *status = format!("Downloading {}...", selected_clone.name);
                }
                ctx_clone.request_repaint();

                match selected_clone
                    .download(selected_clone.file_path.to_string_lossy().to_string())
                {
                    Ok(_) => {
                        let mut status = status_message.lock().unwrap();
                        *status = "Downloaded.".to_string();
                        ctx_clone.request_repaint();
                        log::debug!("Downloaded {}", selected_clone.name);
                    }
                    Err(e) => {
                        let mut status = status_message.lock().unwrap();
                        *status = format!("{}", e);
                        inject_in_progress.store(false, std::sync::atomic::Ordering::SeqCst);
                        ctx_clone.request_repaint();
                        log::error!("Failed to download: {}", e);
                        let _ = message_sender.send(format!("Failed to download: {}", e));
                        return;
                    }
                }
            }

            if !skip_inject_delay {
                thread::sleep(Duration::from_secs(1));
            }

            {
                let mut status = status_message.lock().unwrap();
                *status = "Injecting...".to_string();
            }
            ctx_clone.request_repaint();

            if !skip_inject_delay {
                thread::sleep(Duration::from_secs(1));
            }

            let is_cs2 = selected_clone.process.eq_ignore_ascii_case("cs2.exe");
            let injector_process = if is_cs2 {
                "unknproject.exe"
            } else {
                "unknproject.exe"
            };

            log::debug!("Using {} injector", if is_cs2 { "x64" } else { "x86" });

            let file_path = dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("unknproject")
                .join(injector_process);

            if !file_path.exists() {
                {
                    let mut status = status_message.lock().unwrap();
                    *status = "Downloading manual map injector...".to_string();
                }

                ctx_clone.request_repaint();

                if !skip_inject_delay {
                    thread::sleep(Duration::from_secs(2));
                }

                match download_file(&injector_process, file_path.to_str().unwrap()) {
                    Ok(_) => {
                        let mut status = status_message.lock().unwrap();
                        *status = "Downloaded manual map injector.".to_string();
                        log::debug!("Downloaded manual map injector");
                        ctx_clone.request_repaint();
                    }
                    Err(e) => {
                        let mut status = status_message.lock().unwrap();
                        *status = format!("Failed to download manual map injector: {}", e);
                        inject_in_progress.store(false, std::sync::atomic::Ordering::SeqCst);
                        log::error!("Failed to download manual map injector: {}", e);
                        ctx_clone.request_repaint();
                        let _ = message_sender
                            .send(format!("Failed to download manual map injector: {}", e));
                        return;
                    }
                }
            }

            if !skip_inject_delay {
                thread::sleep(Duration::from_secs(1));
            }

            {
                let mut status = status_message.lock().unwrap();
                *status = "Injecting with manual map injector...".to_string();
                ctx_clone.request_repaint();
            }

            let dll_path = selected_clone.file_path;

            let output = Command::new(file_path).arg(dll_path).output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        let stdout_message = String::from_utf8_lossy(&output.stdout).to_string();
                        log::info!("{}", stdout_message);
                        let mut status = status_message.lock().unwrap();
                        *status = "Injection successful.".to_string();
                        log::info!("Injected {}", selected_clone.name);
                        let _ = message_sender
                            .send(format!("SUCCESS: {}", selected_clone.name).to_string());
                    } else {
                        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
                        let formatted_error_message =
                            format!("Failed to inject: {}", error_message.replace("\n", ""));
                        let _ = message_sender.send(formatted_error_message.clone());
                        log::error!("{}", formatted_error_message);
                        let mut status = status_message.lock().unwrap();
                        *status = formatted_error_message;
                    }
                }
                Err(e) => {
                    let mut status = status_message.lock().unwrap();
                    *status = format!("Failed to execute injector: {}", e);
                    log::error!("Failed to execute injector: {}", e);
                    let _ = message_sender.send(format!("Failed to execute injector: {}", e));
                }
            }

            inject_in_progress.store(false, std::sync::atomic::Ordering::SeqCst);
            ctx_clone.request_repaint();
        });
    }
}
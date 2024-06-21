use eframe::egui::{self, CentralPanel, Context};
use tokio::runtime::Runtime;
use crate::api::{self, CrateResponse};

pub fn create_ui(ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Crates.io Desktop");

        if ui.button("Fetch Crate Info").clicked() {
            let rt = Runtime::new().unwrap();
            match rt.block_on(api::fetch_crate_info("sponge")) {
                Ok(crate_response) => {
                    display_crate_info(ui, &crate_response);
                }
                Err(e) => ui.label(format!("Error: {:?}", e)),
            }
        }
    });
}

fn display_crate_info(ui: &mut egui::Ui, crate_response: &CrateResponse) {
    ui.heading(&crate_response.crate.name);
    ui.label(format!("Description: {}", crate_response.crate.description));
    ui.label(format!("Downloads: {}", crate_response.crate.downloads));
    if let Some(homepage) = &crate_response.crate.homepage {
        ui.hyperlink(homepage);
    }
    if let Some(repository) = &crate_response.crate.repository {
        ui.hyperlink(repository);
    }
    ui.label(format!("Latest Version: {}", crate_response.crate.max_version));

    ui.separator();
    ui.heading("Versions");

    for version in &crate_response.versions {
        ui.group(|ui| {
            ui.label(format!("Version: {}", version.num));
            ui.label(format!("Downloads: {}", version.downloads));
            ui.label(format!("Created At: {}", version.created_at));
            if let Some(license) = &version.license {
                ui.label(format!("License: {}", license));
            }
            if let Some(published_by) = &version.published_by {
                ui.horizontal(|ui| {
                    if let Some(avatar) = &published_by.avatar {
                        ui.image(egui::TextureId::default(), [20.0, 20.0]); // placeholder for avatar image
                    }
                    ui.label(format!("Published by: {}", published_by.login));
                });
            }
        });
    }
}

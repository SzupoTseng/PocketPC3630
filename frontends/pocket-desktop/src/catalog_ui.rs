//! Catalog previews do not receive a runner or input sender.
use egui::{self, RichText, ScrollArea};
use pocket_library::{catalog, controls::Profile, Library};
pub struct CatalogView {
    query: String,
    need: String,
    limit: usize,
    selected: Option<String>,
    profile: Profile,
    landscape: bool,
    installed: String,
}
impl Default for CatalogView {
    fn default() -> Self {
        Self {
            query: String::new(),
            need: "all".into(),
            limit: 48,
            selected: None,
            profile: Profile::Full,
            landscape: false,
            installed: String::new(),
        }
    }
}
impl CatalogView {
    pub fn ui(&mut self, ui: &mut egui::Ui, library: &Library) -> Option<(String, Profile)> {
        ui.heading("Software catalog");
        ui.label("Read-only catalog. Startup records do not verify controls or progress on this platform.");
        ui.horizontal(|ui| {
            ui.label("Search name / version");
            if ui.text_edit_singleline(&mut self.query).changed() {
                self.limit = 48;
            }
            egui::ComboBox::from_id_source("catalog_needs")
                .selected_text(&self.need)
                .show_ui(ui, |ui| {
                    for need in [
                        "all",
                        "stylus",
                        "direction",
                        "text",
                        "hardware",
                        "mouse",
                        "keyboard",
                    ] {
                        if ui
                            .selectable_value(&mut self.need, need.into(), need)
                            .changed()
                        {
                            self.limit = 48;
                        }
                    }
                });
        });
        let query = self.query.to_lowercase();
        let visible: Vec<_> = catalog::entries()
            .iter()
            .filter(|entry| {
                format!("{} {}", entry.name, entry.id)
                    .to_lowercase()
                    .contains(&query)
                    && (self.need == "all"
                        || catalog::guidance(&entry.id)
                            .is_some_and(|g| g.needs.contains(&self.need)))
            })
            .collect();
        ui.label(format!("{} programs", visible.len()));
        ScrollArea::vertical().show(ui, |ui| {
            for entry in visible.iter().take(self.limit) {
                ui.group(|ui| {
                    ui.label(RichText::new(&entry.name).strong());
                    ui.label(format!(
                        "{} · {} · {}",
                        entry.year,
                        entry.cpus.join(" / "),
                        entry.platforms.join(" / ")
                    ));
                    ui.label(format!(
                        "Catalog startup: {} · Controls/progress: unverified here",
                        entry.status
                    ));
                    if ui.button("About / controls preview").clicked() {
                        self.selected = Some(entry.id.clone());
                        self.profile = catalog::guidance(&entry.id)
                            .map(|g| g.recommended)
                            .unwrap_or_default();
                    }
                });
            }
            if visible.len() > self.limit && ui.button("Show more").clicked() {
                self.limit += 48;
            }
        });
        let mut selected = None;
        if let Some(entry) = self
            .selected
            .as_ref()
            .and_then(|id| catalog::entries().iter().find(|e| e.id == *id))
        {
            let mut open = true;
            egui::Window::new(&entry.name).id(egui::Id::new("catalog_details")).open(&mut open).vscroll(true).default_width(560.0).show(ui.ctx(),|ui| {
                ui.label(catalog::description(&entry.id,false));
                ui.label(format!("{} · {} · {}",entry.year,entry.cpus.join(" / "),entry.platforms.join(" / ")));
                ui.label("Screenshot placeholder: no local screen capture is bundled for this version.");
                if let Some(shot)=&entry.shot {ui.hyperlink_to("Open catalog screen record",format!("https://szupotseng.github.io/PocketPC3630/library/shots/{shot}"));}
                if let Some(guide)=catalog::guidance(&entry.id) {
                    for (i,step) in guide.steps.en.iter().enumerate(){ui.label(format!("{}. {step}",i+1));}
                    ui.label(&guide.save.en);
                } else {ui.label("No version-specific guide. Choose controls using the program's own instructions.");}
                ui.separator();
                ui.label(format!("Assets/installation: local binary unmatched. Catalog startup record: {}. Controls/progress: not verified on this platform.",entry.status));
                egui::ComboBox::from_id_source("catalog_profile").selected_text(self.profile.label()).show_ui(ui,|ui| {for p in Profile::ALL {ui.selectable_value(&mut self.profile,p,p.label());}});
                if ui.button("Rotate preview").clicked(){self.landscape= !self.landscape;}
                ui.label(if self.landscape{"Landscape preview — no guest input"}else{"Portrait preview — no guest input"});
                ui.group(|ui| {
                    ui.label("Program screen area (illustration)");
                    egui::Grid::new("catalog_buttons").show(ui,|ui| {
                        for (i,vk) in self.profile.keys().iter().enumerate() {
                            let text=match *vk {38=>"Up".into(),40=>"Down".into(),37=>"Left".into(),39=>"Right".into(),134=>"CE Action".into(),193..=198=>format!("App {}",vk-192),_=>format!("CE VK {vk}")};
                            ui.add_sized([68.0,44.0],egui::Button::new(text)).on_hover_text("Preview only; sends no input");
                            if i%3==2{ui.end_row();}
                        }
                    });
                });
                if self.profile==Profile::Device {ui.label("Generic CE keys. Device/OS/skin baseline unverified; Power and Reset unavailable.");}
                ui.label("Save inside the program before backing up progress. A progress ZIP is not a running-state snapshot.");
                egui::ComboBox::from_id_source("catalog_installation").selected_text(library.get(&self.installed).map(|g|g.display_name.as_str()).unwrap_or("Choose installed program (version unmatched)")).show_ui(ui,|ui| {
                    for game in library.games(){ui.selectable_value(&mut self.installed,game.id.clone(),&game.display_name);}
                });
                if ui.add_enabled(library.get(&self.installed).is_some(),egui::Button::new("Edit this explicit choice…")).clicked(){selected=Some((self.installed.clone(),self.profile));}
                ui.label("This opens a draft. Nothing is saved, installed or started until you explicitly act in the editor.");
            });
            if !open {
                self.selected = None;
            }
        }
        selected
    }
}

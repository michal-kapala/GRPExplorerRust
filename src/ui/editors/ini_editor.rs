use super::*;
use crate::objects::*;

pub struct IniEditor;

impl EditorImpl for IniEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if let Some(v) = match &ectx.bf.object_table.get(&key).unwrap().archetype {
            ObjectArchetype::Ini(ini) => {
                let mut open_tab = None;
                for value in ini.entries.iter() {
                    ui.horizontal(|ui| {
                        match value {
                            IniEntry::Int(key, value) => { 
                                ui.label(format!("{} - {:#010X}", key, value));
                            },
                            IniEntry::AssetKey(key, value) => {
                                ui.label(format!("{} -", key));
                                if ui.selectable_label(false, format!("{:#010X}", value)).clicked() {
                                    open_tab = Some(*value);
                                }
                            },
                            _ => { 
                                ui.label("invalid entry type");
                            }
                        }
                    });
                }
                open_tab
            },
            _ => { None }
        } {
            ectx.respond(EditorResponse::OpenNewTab(v.into()));
        }
    }
}
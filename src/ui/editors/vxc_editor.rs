use super::*;

#[derive(Default)]
pub struct VertexColorsEditor;

impl EditorImpl for VertexColorsEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        let vxc = match &ectx.bf.object_table.get(&key).unwrap().archetype {
            ObjectArchetype::VertexColors(ref vxc) => vxc,
            _ => { return; }
        };

        ui.label(format!("num: {}", vxc.colors.len()));
        egui::ScrollArea::vertical().show(ui, |ui| {
            for col in vxc.colors.iter() {
                ui.label(format!("{}", col));
            }
        });
    }
}
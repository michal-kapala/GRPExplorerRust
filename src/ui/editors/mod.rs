mod script_editor; use script_editor::*;
mod blank_editor; use blank_editor::*;
mod ini_editor; use ini_editor::*;
mod curve_editor; use curve_editor::*;
mod otf_editor; use otf_editor::*;
mod layer_editor; use layer_editor::*;
mod gao_editor; use gao_editor::*;
mod feu_editor; use feu_editor::*;
mod ai_const_editor; use ai_const_editor::*;
mod dbk_editor; use dbk_editor::*;
mod meshes_editor; use meshes_editor::*;
mod texture_editor; use texture_editor::*;
mod sound_editor; use sound_editor::*;
mod shadergraph_editor; use shadergraph_editor::*;
mod skeleton_editor; use skeleton_editor::*;
mod eps_editor; use eps_editor::*;
mod zone_editor; use zone_editor::*;
mod dtb_editor; use dtb_editor::*;
mod vxc_editor; use vxc_editor::*;
mod world_editor; use world_editor::*;
mod collision_editor; use collision_editor::*;

pub use crate::egui as egui;

use super::editor_tabs_view::EditorTabContext;
pub use super::{EditorContext, EditorResponse};

use crate::{bigfile::metadata::ObjectType, metadata::YKey, objects::ObjectArchetype};

pub trait EditorImpl {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, tctx: &EditorTabContext);
}

pub fn create_editor_for_type(obj_type: &ObjectType) -> Box<dyn EditorImpl> {
    match obj_type {
        ObjectType::zc  => Box::new(ScriptEditor { }),
        ObjectType::ini => Box::new(IniEditor { }),
        ObjectType::cur => Box::new(CurveEditor { }),
        ObjectType::otf => Box::new(OtfEditor { }),
        ObjectType::lay => Box::new(LayerEditor { }),
        ObjectType::gao => Box::new(GameobjectEditor::default()),
        ObjectType::feu => Box::new(FeuEditor { }),
        ObjectType::cst => Box::new(AIConstEditor { }),
        ObjectType::dbk => Box::new(DbkEditor { }),
        ObjectType::msd => Box::new(MeshDataEditor { }),
        ObjectType::tga => Box::new(TextureMetadataEditor::default()),
        ObjectType::txd => Box::new(TextureDataEditor { }),
        ObjectType::snk => Box::new(SnkEditor { }),
        ObjectType::shd => Box::new(ShaderGraphEditor { }),
        ObjectType::ske => Box::new(SkeletonEditor::default()),
        ObjectType::eps => Box::new(EditableParamStructEditor { }),
        ObjectType::zon => Box::new(ZoneEditor { }),
        ObjectType::dbr => Box::new(DbrEditor { }),
        ObjectType::epl => Box::new(EditableParamsListEditor { }),
        ObjectType::dtb => Box::new(DataTableEditor { }),
        ObjectType::msh => Box::new(MeshMetadataEditor { }),
        ObjectType::vxc => Box::new(VertexColorsEditor::default()),
        ObjectType::got => Box::new(GraphicObjectTableEditor::default()),
        ObjectType::wor => Box::new(WorldEditor::default()),
        ObjectType::col => Box::new(CollisionObjectEditor::default()),
        _               => Box::new(BlankEditor { }),
    }
}

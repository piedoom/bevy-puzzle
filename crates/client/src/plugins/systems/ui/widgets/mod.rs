mod pattern;

use bevy::{asset::Asset, prelude::*, reflect::TypeUuid};
use bevy_egui::egui::{self, Widget};
pub use pattern::*;
use std::fmt::Display;

pub struct SelectAssetWidget<'a, T>
where
    T: Asset + TypeUuid + Display,
{
    pub name: &'a str,
    pub selection: &'a mut Option<Handle<T>>,
    pub assets: &'a Assets<T>,
}

impl<'a, T> Widget for SelectAssetWidget<'a, T>
where
    T: Asset + TypeUuid + Display,
{
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> bevy_egui::egui::Response {
        let handle = self.selection.clone().unwrap_or_default();
        let maybe_asset = self.assets.get(handle.clone());
        ui.vertical(|ui| {
            egui::ComboBox::from_label(self.name)
                .selected_text(
                    maybe_asset
                        .map(|t| format!("{}", t))
                        .unwrap_or_else(|| String::from("None selected")),
                )
                .show_ui(ui, |ui| {
                    for (id, t) in self.assets.iter() {
                        let row_handle = self.assets.get_handle(id);
                        if ui
                            .selectable_value(
                                &mut handle.clone(),
                                row_handle.clone(),
                                format!("{}", t),
                            )
                            .clicked()
                        {
                            *self.selection = Some(row_handle.clone());
                        }
                    }
                })
        })
        .response
    }
}

// egui::ComboBox::from_label("Map")
//                     .selected_text(&map.map(|x| &x.name).unwrap_or(&"None selected".to_string()))
//                     .show_ui(ui, |ui| {
//                         for (id, map) in maps.iter() {
//                             let select_handle = maps.get_handle(id);
//                             if ui
//                                 .selectable_value(
//                                     &mut menu_state.map,
//                                     Some(select_handle.clone()),
//                                     &map.name,
//                                 )
//                                 .clicked()
//                             {
//                                 menu_state.map = Some(select_handle.clone());
//                             }
//                         }
//                     });

use crate::{
    draw::{component::base::key_input::KeyInput, event::Event, frame::Frame},
    error::OxideResult,
    s,
    util::arcm::Arcm,
};

use super::{
    base::{checkbox::Checkbox, float_input::FloatInput, window::Window},
    Component, Components,
};

#[derive(Debug)]
pub struct VisualsWindow {
    window: Window,
}

impl VisualsWindow {
    pub fn new(visible: Arcm<bool>) -> VisualsWindow {
        let mut window = Window::new("VISUALS".to_owned(), visible);
        let mut y = 10;
        macro_rules! a {
            ($e:expr) => {
                window.add($e);
                #[allow(unused_assignments)]
                y += $e.get_base().h + 8
            };
        }

        a!(Checkbox::new(
            "third person",
            s!().visual.third_person.clone(),
            10,
            y,
        ));
        a!(KeyInput::new(
            20,
            y,
            "toggle key",
            s!().visual.tp_key.clone(),
        ));
        a!(KeyInput::new(
            20,
            y,
            "offset key",
            s!().visual.tp_offset_key.clone(),
        ));
        a!(FloatInput::new(
            30,
            y,
            "x offset",
            s!().visual.tp_offset_x.clone(),
            None,
        ));
        a!(FloatInput::new(
            30,
            y,
            "y offset",
            s!().visual.tp_offset_y.clone(),
            None,
        ));
        a!(FloatInput::new(
            30,
            y,
            "z offset",
            s!().visual.tp_offset_z.clone(),
            None,
        ));

        a!(FloatInput::new(10, y, "fov", s!().visual.fov.clone(), None,));

        a!(Checkbox::new("esp", s!().visual.esp.clone(), 10, y));
        a!(Checkbox::new(
            "friendlies",
            s!().visual.esp_friendlies.clone(),
            20,
            y
        ));
        a!(Checkbox::new(
            "sentries",
            s!().visual.esp_sentreis.clone(),
            20,
            y
        ));
        a!(Checkbox::new(
            "projectiles",
            s!().visual.esp_projectiles.clone(),
            20,
            y
        ));

        a!(Checkbox::new(
            "hitboxes",
            s!().visual.hitboxes.clone(),
            10,
            y,
        ));
        a!(Checkbox::new(
            "remove scope",
            s!().visual.remove_scope.clone(),
            10,
            y,
        ));
        a!(Checkbox::new(
            "remove zoom",
            s!().visual.remove_zoom.clone(),
            10,
            y,
        ));
        a!(Checkbox::new(
            "remove disguises",
            s!().visual.remove_disguises.clone(),
            10,
            y,
        ));
        a!(Checkbox::new(
            "spectator list",
            s!().visual.spectator_list.clone(),
            10,
            y,
        ));
        a!(Checkbox::new(
            "pure bypass",
            s!().visual.pure_bypass.clone(),
            10,
            y,
        ));
        VisualsWindow { window }
    }
}

impl Component for VisualsWindow {
    fn draw(&mut self, frame: &mut Frame) -> OxideResult<()> {
        self.window.draw(frame)
    }
    fn handle_event(&mut self, event: &mut Event) {
        self.window.handle_event(event);
    }
    fn get_draw_order(&self) -> super::DrawOrder {
        self.window.get_draw_order()
    }
    fn set_draw_order(&mut self, order: super::DrawOrder) {
        self.window.set_draw_order(order)
    }
    fn get_base(&mut self) -> &mut super::ComponentBase {
        self.window.get_base()
    }
}

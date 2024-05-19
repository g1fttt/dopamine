use crate::game::Entity;

pub struct RenderableObject<'a> {
    pub entity: &'a Entity,
}

impl<'a> RenderableObject<'a> {
    pub fn new(entity: &'a Entity) -> Self {
        Self { entity }
    }

    pub fn should_draw(&self) -> bool {
        self.entity.should_draw() && !self.entity.is_dormant()
    }

    pub fn draw_model(&self) {
        self.entity.draw_model();

        let mut attachment = self.entity.move_child();
        while let Some(att) = attachment {
            if att.should_draw() {
                att.draw_model();
            }
            attachment = self.entity.move_peer();
        }
    }
}

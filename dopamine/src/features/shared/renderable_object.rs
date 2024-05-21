use crate::game::Entity;

#[derive(Clone)]
pub struct RenderableObject<'a> {
    pub entity: &'a Entity,
    pub model_was_drawn: bool,
}

impl<'a> RenderableObject<'a> {
    pub fn new(entity: &'a Entity) -> Self {
        Self {
            entity,
            model_was_drawn: false,
        }
    }

    pub fn should_draw_model(&self) -> bool {
        self.entity.should_draw() && !self.entity.is_dormant()
    }

    pub fn draw_model(&self) {
        self.entity.draw_model();
    }

    pub fn draw_attachments(&self) {
        self.entity
            .attachments()
            .filter(|&att| att.should_draw())
            .for_each(|att| {
                att.draw_model();
            });
    }
}

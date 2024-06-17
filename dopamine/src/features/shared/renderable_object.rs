use crate::game::{Entity, RenderableEntity};

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
    self.entity.renderable().should_draw() && !self.entity.networkable().is_dormant()
  }

  pub fn draw_model(&self) {
    self.entity.renderable().draw_model();
  }

  pub fn draw_attachments(&self) {
    self
      .entity
      .attachments()
      .map(Entity::renderable)
      .filter(|att_rend| att_rend.should_draw())
      .for_each(RenderableEntity::draw_model);
  }
}

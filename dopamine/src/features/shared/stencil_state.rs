use crate::game::material_system::{RenderContext, StencilCmpFn, StencilOp};

pub struct StencilState {
    pub enable: bool,
    pub fail_op: StencilOp,
    pub z_fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub cmp_fn: StencilCmpFn,
    pub ref_value: i32,
    pub test_mask: u32,
    pub write_mask: u32,
}

impl StencilState {
    pub fn set(&self, render_ctx: &RenderContext) {
        render_ctx.set_stencil_enable(self.enable);
        render_ctx.set_stencil_fail_op(self.fail_op);
        render_ctx.set_stencil_z_fail_op(self.z_fail_op);
        render_ctx.set_stencil_pass_op(self.pass_op);
        render_ctx.set_stencil_cmp_fn(self.cmp_fn);
        render_ctx.set_stencil_ref_value(self.ref_value);
        render_ctx.set_stencil_test_mask(self.test_mask);
        render_ctx.set_stencil_write_mask(self.write_mask);
    }
}

impl Default for StencilState {
    fn default() -> Self {
        Self {
            enable: bool::default(),
            fail_op: StencilOp::default(),
            z_fail_op: StencilOp::default(),
            pass_op: StencilOp::default(),
            cmp_fn: StencilCmpFn::default(),
            ref_value: i32::default(),
            test_mask: 0xFFFFFFFF,
            write_mask: 0xFFFFFFFF,
        }
    }
}

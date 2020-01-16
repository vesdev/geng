use crate::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DepthFunc {
    Less = ugl::LESS as _,
    LessOrEqual = ugl::LEQUAL as _,
    Greater = ugl::GREATER as _,
}

impl Default for DepthFunc {
    fn default() -> DepthFunc {
        DepthFunc::Less
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BlendMode {
    Alpha,
}

impl Default for BlendMode {
    fn default() -> BlendMode {
        BlendMode::Alpha
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CullFace {
    Back = ugl::BACK as _,
    Front = ugl::FRONT as _,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DrawParameters {
    pub depth_func: Option<DepthFunc>,
    pub blend_mode: Option<BlendMode>,
    pub cull_face: Option<CullFace>,
    pub viewport: Option<AABB<usize>>,
    pub write_depth: bool,
}

impl Default for DrawParameters {
    fn default() -> Self {
        Self {
            depth_func: None,
            blend_mode: None,
            cull_face: None,
            viewport: None,
            write_depth: true,
        }
    }
}

impl DrawParameters {
    pub(crate) fn apply(&self, gl: &ugl::Context, framebuffer_size: Vec2<usize>) {
        match self.depth_func {
            Some(depth_test) => gl.depth_func(depth_test as _),
            None => gl.depth_func(ugl::ALWAYS),
        }
        match self.blend_mode {
            Some(blend_mode) => {
                gl.enable(ugl::BLEND);
                match blend_mode {
                    BlendMode::Alpha => gl.blend_func(ugl::SRC_ALPHA, ugl::ONE_MINUS_SRC_ALPHA),
                }
            }
            None => gl.disable(ugl::BLEND),
        }
        match self.cull_face {
            Some(cull_face) => {
                gl.enable(ugl::CULL_FACE);
                gl.cull_face(cull_face as ugl::Enum);
            }
            None => gl.disable(ugl::CULL_FACE),
        }
        if let Some(rect) = self.viewport {
            gl.viewport(
                rect.x_min as _,
                rect.y_min as _,
                rect.width() as _,
                rect.height() as _,
            );
        } else {
            gl.viewport(0, 0, framebuffer_size.x as _, framebuffer_size.y as _);
        }
        gl.depth_mask(gl_bool(self.write_depth));
    }
}

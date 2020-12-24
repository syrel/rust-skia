use crate::prelude::*;
use crate::{
    scalar, BlendMode, Color, Color4f, ColorFilter, ColorSpace, FilterQuality, ImageFilter,
    MaskFilter, Path, PathEffect, Rect, Shader,
};
use skia_bindings as sb;
use skia_bindings::SkPaint;
use std::hash::{Hash, Hasher};
use std::ptr;

pub use sb::SkPaint_Style as Style;
#[test]
pub fn test_style_naming() {
    let _ = Style::Fill;
}

pub use sb::SkPaint_Cap as Cap;
#[test]
pub fn test_cap_naming() {
    let _ = Cap::Butt;
}

pub use sb::SkPaint_Join as Join;
#[test]
pub fn test_join_naming() {
    let _ = Join::Miter;
}

pub type Paint = Handle<SkPaint>;
unsafe impl Send for Paint {}
unsafe impl Sync for Paint {}

impl NativeDrop for SkPaint {
    fn drop(&mut self) {
        unsafe { sb::C_SkPaint_destruct(self) }
    }
}

impl NativeClone for SkPaint {
    fn clone(&self) -> Self {
        unsafe { SkPaint::new2(self) }
    }
}

impl NativePartialEq for SkPaint {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkPaint_Equals(self, rhs) }
    }
}

impl NativeHash for SkPaint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { self.getHash() }.hash(state)
    }
}

impl Default for Handle<SkPaint> {
    fn default() -> Self {
        Paint::from_native_c(unsafe { SkPaint::new() })
    }
}

impl Handle<SkPaint> {
    pub fn new(color: impl AsRef<Color4f>, color_space: Option<&ColorSpace>) -> Paint {
        Paint::from_native_c(unsafe {
            SkPaint::new1(
                color.as_ref().native(),
                color_space.native_ptr_or_null_mut_force(),
            )
        })
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.native_mut().reset() }
        self
    }

    pub fn is_anti_alias(&self) -> bool {
        unsafe { self.native().__bindgen_anon_1.fBitfields.fAntiAlias() != 0 }
    }

    pub fn set_anti_alias(&mut self, anti_alias: bool) -> &mut Self {
        unsafe {
            self.native_mut()
                .__bindgen_anon_1
                .fBitfields
                .set_fAntiAlias(anti_alias as _);
        }
        self
    }

    pub fn is_dither(&self) -> bool {
        unsafe { self.native().__bindgen_anon_1.fBitfields.fDither() != 0 }
    }

    pub fn set_dither(&mut self, dither: bool) -> &mut Self {
        unsafe {
            self.native_mut()
                .__bindgen_anon_1
                .fBitfields
                .set_fDither(dither as _);
        }
        self
    }

    pub fn filter_quality(&self) -> FilterQuality {
        unsafe { sb::C_SkPaint_getFilterQuality(self.native()) }
    }

    pub fn set_filter_quality(&mut self, quality: FilterQuality) -> &mut Self {
        unsafe { self.native_mut().setFilterQuality(quality) }
        self
    }

    pub fn style(&self) -> Style {
        unsafe { sb::C_SkPaint_getStyle(self.native()) }
    }

    pub fn set_style(&mut self, style: Style) -> &mut Self {
        unsafe { self.native_mut().setStyle(style) }
        self
    }

    pub fn set_stroke(&mut self, stroke: bool) -> &mut Self {
        unsafe { self.native_mut().setStroke(stroke) }
        self
    }

    pub fn color(&self) -> Color {
        self.color4f().to_color()
    }

    pub fn color4f(&self) -> Color4f {
        Color4f::from_native_c(self.native().fColor4f)
    }

    pub fn set_color(&mut self, color: impl Into<Color>) -> &mut Self {
        let color = color.into();
        unsafe { self.native_mut().setColor(color.into_native()) }
        self
    }

    pub fn set_color4f(
        &mut self,
        color: impl AsRef<Color4f>,
        color_space: &ColorSpace,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .setColor1(color.as_ref().native(), color_space.native_mut_force())
        }
        self
    }

    pub fn alpha_f(&self) -> f32 {
        self.color4f().a
    }

    pub fn alpha(&self) -> u8 {
        unsafe { sb::C_SkPaint_getAlpha(self.native()) }
    }

    pub fn set_alpha_f(&mut self, alpha: f32) -> &mut Self {
        unsafe { self.native_mut().setAlphaf(alpha) }
        self
    }

    pub fn set_alpha(&mut self, alpha: u8) -> &mut Self {
        self.set_alpha_f(f32::from(alpha) * (1.0 / 255.0))
    }

    pub fn set_argb(&mut self, a: u8, r: u8, g: u8, b: u8) -> &mut Self {
        unsafe {
            self.native_mut()
                .setARGB(a.into(), r.into(), g.into(), b.into())
        }
        self
    }

    pub fn stroke_width(&self) -> scalar {
        self.native().fWidth
    }

    pub fn set_stroke_width(&mut self, width: scalar) -> &mut Self {
        unsafe { self.native_mut().setStrokeWidth(width) }
        self
    }

    pub fn stroke_miter(&self) -> scalar {
        self.native().fMiterLimit
    }

    pub fn set_stroke_miter(&mut self, miter: scalar) -> &mut Self {
        unsafe { self.native_mut().setStrokeMiter(miter) }
        self
    }

    pub fn stroke_cap(&self) -> Cap {
        unsafe { sb::C_SkPaint_getStrokeCap(self.native()) }
    }

    pub fn set_stroke_cap(&mut self, cap: Cap) -> &mut Self {
        unsafe { self.native_mut().setStrokeCap(cap) }
        self
    }

    pub fn stroke_join(&self) -> Join {
        unsafe { sb::C_SkPaint_getStrokeJoin(self.native()) }
    }

    pub fn set_stroke_join(&mut self, join: Join) -> &mut Self {
        unsafe { self.native_mut().setStrokeJoin(join) }
        self
    }

    pub fn get_fill_path(
        &self,
        src: &Path,
        cull_rect: Option<&Rect>,
        res_scale: impl Into<Option<scalar>>,
    ) -> Option<Path> {
        let mut r = Path::default();

        let cull_rect_ptr = cull_rect
            .map(|r| r.native() as *const _)
            .unwrap_or(ptr::null());

        unsafe {
            self.native().getFillPath(
                src.native(),
                r.native_mut(),
                cull_rect_ptr,
                res_scale.into().unwrap_or(1.0),
            )
        }
        .if_true_some(r)
    }

    pub fn shader(&self) -> Option<Shader> {
        Shader::from_unshared_ptr(self.native().fShader.fPtr)
    }

    pub fn set_shader(&mut self, shader: impl Into<Option<Shader>>) -> &mut Self {
        unsafe { sb::C_SkPaint_setShader(self.native_mut(), shader.into().into_ptr_or_null()) }
        self
    }

    pub fn color_filter(&self) -> Option<ColorFilter> {
        ColorFilter::from_unshared_ptr(self.native().fColorFilter.fPtr)
    }

    pub fn set_color_filter(&mut self, color_filter: impl Into<Option<ColorFilter>>) -> &mut Self {
        unsafe {
            sb::C_SkPaint_setColorFilter(self.native_mut(), color_filter.into().into_ptr_or_null())
        }
        self
    }

    pub fn blend_mode(&self) -> BlendMode {
        unsafe { sb::C_SkPaint_getBlendMode(self.native()) }
    }

    pub fn is_src_over(&self) -> bool {
        self.blend_mode() == BlendMode::SrcOver
    }

    pub fn set_blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        unsafe {
            self.native_mut()
                .__bindgen_anon_1
                .fBitfields
                .set_fBlendMode(mode as _);
        };
        self
    }

    pub fn path_effect(&self) -> Option<PathEffect> {
        PathEffect::from_unshared_ptr(self.native().fPathEffect.fPtr)
    }

    pub fn set_path_effect(&mut self, path_effect: impl Into<Option<PathEffect>>) -> &mut Self {
        unsafe {
            sb::C_SkPaint_setPathEffect(self.native_mut(), path_effect.into().into_ptr_or_null())
        }
        self
    }

    pub fn mask_filter(&self) -> Option<MaskFilter> {
        MaskFilter::from_unshared_ptr(self.native().fMaskFilter.fPtr)
    }

    pub fn set_mask_filter(&mut self, mask_filter: impl Into<Option<MaskFilter>>) -> &mut Self {
        unsafe {
            sb::C_SkPaint_setMaskFilter(self.native_mut(), mask_filter.into().into_ptr_or_null())
        }
        self
    }

    pub fn image_filter(&self) -> Option<ImageFilter> {
        ImageFilter::from_unshared_ptr(self.native().fImageFilter.fPtr)
    }

    pub fn set_image_filter(&mut self, image_filter: impl Into<Option<ImageFilter>>) -> &mut Self {
        unsafe {
            sb::C_SkPaint_setImageFilter(self.native_mut(), image_filter.into().into_ptr_or_null())
        }
        self
    }

    pub fn nothing_to_draw(&self) -> bool {
        unsafe { self.native().nothingToDraw() }
    }
}

#[test]
fn default_creation() {
    let paint = Paint::default();
    drop(paint)
}

#[test]
fn method_chaining_compiles() {
    let mut paint = Paint::default();
    let _paint = paint.reset().reset();
}

#[test]
fn union_flags() {
    let mut paint = Paint::default();
    assert!(!paint.is_anti_alias());
    assert!(!paint.is_dither());
    assert_eq!(paint.filter_quality(), FilterQuality::None);
    assert_eq!(paint.style(), Style::Fill);

    {
        paint.set_anti_alias(true);

        assert!(paint.is_anti_alias());
        assert!(!paint.is_dither());
        assert_eq!(paint.filter_quality(), FilterQuality::None);
        assert_eq!(paint.style(), Style::Fill);

        paint.set_anti_alias(false);
    }

    {
        paint.set_filter_quality(FilterQuality::High);

        assert!(!paint.is_anti_alias());
        assert!(!paint.is_dither());
        assert_eq!(paint.filter_quality(), FilterQuality::High);
        assert_eq!(paint.style(), Style::Fill);

        paint.set_filter_quality(FilterQuality::None);
    }

    {
        paint.set_style(Style::StrokeAndFill);

        assert!(!paint.is_anti_alias());
        assert!(!paint.is_dither());
        assert_eq!(paint.filter_quality(), FilterQuality::None);
        assert_eq!(paint.style(), Style::StrokeAndFill);

        paint.set_style(Style::Fill);
    }
}

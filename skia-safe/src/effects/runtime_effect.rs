use crate::{
    interop::{self, AsStr},
    prelude::*,
    ColorFilter, Data, Matrix, Shader,
};
use skia_bindings::{
    self as sb, SkRefCntBase, SkRuntimeEffect, SkRuntimeEffect_Options, SkRuntimeEffect_Uniform,
    SkRuntimeEffect_Varying,
};
use std::{ffi::CStr, fmt};

pub type Uniform = Handle<SkRuntimeEffect_Uniform>;

#[deprecated(since = "0.35.0", note = "Use Uniform instead")]
pub type Variable = Uniform;

unsafe impl Send for Uniform {}
unsafe impl Sync for Uniform {}

impl NativeDrop for SkRuntimeEffect_Uniform {
    fn drop(&mut self) {
        panic!("native type SkRuntimeEffect::Uniform can't be owned by Rust");
    }
}

impl fmt::Debug for Uniform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.native().fmt(f)
    }
}

impl Uniform {
    pub fn name(&self) -> &str {
        self.native().name.as_str()
    }

    pub fn offset(&self) -> usize {
        self.native().offset
    }

    pub fn ty(&self) -> uniform::Type {
        self.native().type_
    }

    pub fn count(&self) -> i32 {
        self.native().count
    }

    pub fn flags(&self) -> uniform::Flags {
        uniform::Flags::from_bits(self.native().flags).unwrap()
    }

    pub fn marker(&self) -> u32 {
        self.native().marker
    }

    pub fn is_array(&self) -> bool {
        self.flags().contains(uniform::Flags::ARRAY)
    }

    pub fn size_in_bytes(&self) -> usize {
        unsafe { self.native().sizeInBytes() }
    }
}

pub mod uniform {
    use skia_bindings as sb;

    pub use sb::SkRuntimeEffect_Uniform_Type as Type;
    #[test]
    fn test_type_naming() {
        let _ = Type::Float2x2;
    }

    bitflags! {
        pub struct Flags : u32 {
            const ARRAY = sb::SkRuntimeEffect_Uniform_Flags_kArray_Flag as _;
            const MARKER = sb::SkRuntimeEffect_Uniform_Flags_kMarker_Flag as _;
            const MARKER_NORMALS = sb::SkRuntimeEffect_Uniform_Flags_kMarkerNormals_Flag as _;
            const SRGB_UNPREMUL = sb::SkRuntimeEffect_Uniform_Flags_kSRGBUnpremul_Flag as _;
        }
    }
}

pub type Varying = Handle<SkRuntimeEffect_Varying>;
unsafe impl Send for Varying {}
unsafe impl Sync for Varying {}

impl NativeDrop for SkRuntimeEffect_Varying {
    fn drop(&mut self) {
        panic!("native type SkRuntimeEffect::Varying can't be owned by Rust");
    }
}

impl fmt::Debug for Varying {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Varying")
            .field("name", &self.name())
            .field("width", &self.width())
            .finish()
    }
}

impl Varying {
    pub fn name(&self) -> &str {
        self.native().name.as_str()
    }

    pub fn width(&self) -> i32 {
        self.native().width
    }
}

pub type RuntimeEffect = RCHandle<SkRuntimeEffect>;

impl NativeRefCountedBase for SkRuntimeEffect {
    type Base = SkRefCntBase;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct Options {
    pub force_no_inline: bool,
}

impl NativeTransmutable<SkRuntimeEffect_Options> for Options {}

pub fn new(sksl: impl AsRef<str>) -> Result<RuntimeEffect, String> {
    new_with_options(sksl, None)
}

pub fn new_with_options<'a>(
    sksl: impl AsRef<str>,
    options: impl Into<Option<&'a Options>>,
) -> Result<RuntimeEffect, String> {
    let str = interop::String::from_str(sksl);
    let options = options.into().copied().unwrap_or_default();
    let mut error = interop::String::default();
    RuntimeEffect::from_ptr(unsafe {
        sb::C_SkRuntimeEffect_Make(str.native(), options.native(), error.native_mut())
    })
    .ok_or_else(|| error.as_str().to_owned())
}

impl fmt::Debug for RuntimeEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuntimeEffect")
            .field("uniform_size", &self.uniform_size())
            .field("uniforms", &self.uniforms())
            .field("varyings", &self.varyings())
            .finish()
    }
}

impl RuntimeEffect {
    pub fn make_shader<'a>(
        &self,
        uniforms: impl Into<Data>,
        children: impl IntoIterator<Item = Shader>,
        local_matrix: impl Into<Option<&'a Matrix>>,
        is_opaque: bool,
    ) -> Option<Shader> {
        let mut children: Vec<_> = children
            .into_iter()
            .map(|shader| shader.into_ptr())
            .collect();
        Shader::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_makeShader(
                self.native(),
                uniforms.into().into_ptr(),
                children.as_mut_ptr(),
                children.len(),
                local_matrix.into().native_ptr_or_null(),
                is_opaque,
            )
        })
    }

    #[cfg(feature = "gpu")]
    pub fn make_image<'a>(
        &self,
        context: &mut crate::gpu::RecordingContext,
        uniforms: impl Into<Data>,
        children: impl IntoIterator<Item = Shader>,
        local_matrix: impl Into<Option<&'a Matrix>>,
        result_info: crate::ImageInfo,
        mipmapped: bool,
    ) -> Option<crate::Image> {
        let mut children: Vec<_> = children
            .into_iter()
            .map(|shader| shader.into_ptr())
            .collect();

        crate::Image::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_makeImage(
                self.native(),
                context.native_mut(),
                uniforms.into().into_ptr(),
                children.as_mut_ptr(),
                children.len(),
                local_matrix.into().native_ptr_or_null(),
                result_info.native(),
                mipmapped,
            )
        })
    }

    pub fn make_color_filter(&self, inputs: impl Into<Data>) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_makeColorFilter(self.native(), inputs.into().into_ptr())
        })
    }

    pub fn source(&self) -> &str {
        unsafe { (*sb::C_SkRuntimeEffect_source(self.native())).as_str() }
    }

    #[deprecated(since = "0.35.0", note = "Use uniform_size() instead")]
    pub fn input_size(&self) -> usize {
        self.uniform_size()
    }

    pub fn uniform_size(&self) -> usize {
        unsafe { self.native().uniformSize() }
    }

    #[deprecated(since = "0.35.0", note = "Use uniforms() instead")]
    pub fn inputs(&self) -> &[Uniform] {
        self.uniforms()
    }

    pub fn uniforms(&self) -> &[Uniform] {
        unsafe {
            let mut count: usize = 0;
            let ptr = sb::C_SkRuntimeEffect_uniforms(self.native(), &mut count);
            safer::from_raw_parts(Uniform::from_native_ptr(ptr), count)
        }
    }

    pub fn children(&self) -> impl Iterator<Item = &str> {
        unsafe {
            let mut count: usize = 0;
            let ptr = sb::C_SkRuntimeEffect_children(self.native(), &mut count);
            let slice = safer::from_raw_parts(ptr, count);
            slice.iter().map(|str| str.as_str())
        }
    }

    pub fn varyings(&self) -> &[Varying] {
        unsafe {
            let mut count: usize = 0;
            let ptr = sb::C_SkRuntimeEffect_varyings(self.native(), &mut count);
            safer::from_raw_parts(Varying::from_native_ptr(ptr), count)
        }
    }

    #[deprecated(since = "0.35.0", note = "Use find_uniform()")]
    pub fn find_input(&self, name: impl AsRef<CStr>) -> Option<&Uniform> {
        self.find_uniform(name)
    }

    pub fn find_uniform(&self, name: impl AsRef<CStr>) -> Option<&Uniform> {
        unsafe { self.native().findUniform(name.as_ref().as_ptr()) }
            .into_option()
            .map(|ptr| Uniform::from_native_ref(unsafe { &*ptr }))
    }

    pub fn find_child(&self, name: impl AsRef<CStr>) -> Option<usize> {
        unsafe {
            self.native()
                .findChild(name.as_ref().as_ptr())
                .try_into()
                .ok()
        }
    }
}

// TODO: wrap SkRuntimeEffectBuilder, SkRuntimeShaderBuilder

#[cfg(test)]
mod tests {
    use crate::prelude::NativeTransmutable;

    #[test]
    fn options_layout() {
        super::Options::test_layout()
    }
}

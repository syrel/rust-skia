use crate::prelude::NativeTransmutable;
use crate::{ISize, Matrix};
use skia_bindings as sb;
use skia_bindings::SkEncodedOrigin;

// Even though possible, we are not using the original SkEncodedOrigin enum, because of the
// `to_matrix()` implementation below, which needs an `ISize` and so can not be implemented in the
// skia-bindings crate.

/// These values match the orientation www.exif.org/Exif2-2.PDF.
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum EncodedOrigin {
    /// Default
    TopLeft = SkEncodedOrigin::TopLeft as _,
    /// Reflected across y-axis
    TopRight = SkEncodedOrigin::TopRight as _,
    /// Rotated 180
    BottomRight = SkEncodedOrigin::BottomRight as _,
    /// Reflected across x-axis
    BottomLeft = SkEncodedOrigin::BottomLeft as _,
    /// Reflected across x-axis, Rotated 90 CCW
    LeftTop = SkEncodedOrigin::LeftTop as _,
    /// Rotated 90 CW
    RightTop = SkEncodedOrigin::RightTop as _,
    /// Reflected across x-axis, Rotated 90 CW
    RightBottom = SkEncodedOrigin::RightBottom as _,
    /// Rotated 90 CCW
    LeftBottom = SkEncodedOrigin::LeftBottom as _,
}

impl NativeTransmutable<SkEncodedOrigin> for EncodedOrigin {}

impl Default for EncodedOrigin {
    fn default() -> Self {
        EncodedOrigin::TopLeft
    }
}

impl EncodedOrigin {
    pub const LAST: Self = EncodedOrigin::LeftBottom;
    pub const DEFAULT: Self = EncodedOrigin::TopLeft;

    /// Given an width and height of the source data, returns a matrix that transforms the source
    /// rectangle with upper left corner at `[0, 0]` and origin to a correctly oriented destination
    /// rectangle of `[0, 0, w, h]`.
    pub fn to_matrix(self, size: impl Into<ISize>) -> Matrix {
        let size = size.into();
        let mut m = Matrix::default();
        unsafe {
            sb::C_SkEncodedOriginToMatrix(
                self.into_native(),
                size.width,
                size.height,
                m.native_mut(),
            )
        };
        m
    }

    /// Return `true` if the encoded origin includes a 90 degree rotation, in which case the width
    /// and height of the source data are swapped relative to a correctly oriented destination.
    pub fn swaps_width_height(self) -> bool {
        (self as i32) >= EncodedOrigin::LeftTop as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::NativeTransmutable, EncodedOrigin};

    #[test]
    fn test_encoded_origin_layout() {
        EncodedOrigin::test_layout();
    }
}

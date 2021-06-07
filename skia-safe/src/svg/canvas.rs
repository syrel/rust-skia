use crate::interop::DynamicMemoryWStream;
use crate::prelude::*;
use crate::{Data, Rect};
use skia_bindings as sb;
use skia_bindings::SkCanvas;
use std::pin::Pin;
use std::ptr;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

pub struct Canvas {
    canvas: *mut SkCanvas,
    stream: Pin<Box<DynamicMemoryWStream>>,
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            sb::C_SkCanvas_delete(self.canvas);
        }
    }
}

impl Deref for Canvas {
    type Target = crate::Canvas;

    fn deref(&self) -> &Self::Target {
        crate::Canvas::borrow_from_native(unsafe { &*self.canvas })
    }
}

impl DerefMut for Canvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        crate::Canvas::borrow_from_native_mut(unsafe { &mut *self.canvas })
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Flags : u32 {
        const CONVERT_TEXT_TO_PATHS = sb::SkSVGCanvas_kConvertTextToPaths_Flag as _;
        const NO_PRETTY_XML = sb::SkSVGCanvas_kNoPrettyXML_Flag as _;
    }
}

impl fmt::Debug for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Canvas")
            .field(
                "canvas",
                crate::Canvas::borrow_from_native(unsafe { &*self.canvas }),
            )
            .field("stream", &self.stream)
            .finish()
    }
}

impl Canvas {
    /// Creates a new SVG canvas.
    pub fn new(bounds: impl AsRef<Rect>, flags: impl Into<Option<Flags>>) -> Canvas {
        let bounds = bounds.as_ref();
        let flags = flags.into().unwrap_or_default();
        let mut stream = Box::pin(DynamicMemoryWStream::new());
        let canvas = unsafe {
            sb::C_SkSVGCanvas_Make(
                bounds.native(),
                &mut stream.native_mut()._base,
                flags.bits(),
            )
        };
        Canvas { canvas, stream }
    }

    /// Ends the Canvas drawing and returns the resulting SVG.
    /// TODO: rename to into_svg() or into_svg_data()?
    pub fn end(mut self) -> Data {
        // note: flushing canvas + XMLStreamWriter does not seem to work,
        // we have to delete the canvas and destruct the stream writer
        // to get all data out _and_ keep the referential integrity.
        unsafe {
            sb::C_SkCanvas_delete(self.canvas);
        }
        self.canvas = ptr::null_mut();
        self.stream.detach_as_data()
    }
}

#[test]
fn test_svg() {
    use crate::Paint;

    let mut canvas = Canvas::new(&Rect::from_size((20, 20)), None);
    let paint = Paint::default();
    canvas.draw_circle((10, 10), 10.0, &paint);
    let data = canvas.end();
    let contents = String::from_utf8_lossy(data.as_bytes());
    dbg!(&contents);
    assert!(contents.contains(r#"<ellipse cx="10" cy="10" rx="10" ry="10"/>"#));
    assert!(contents.contains(r#"</svg>"#));
}

#[test]
fn test_svg_without_ending() {
    use crate::Paint;
    let mut canvas = Canvas::new(&Rect::from_size((20, 20)), None);
    let paint = Paint::default();
    canvas.draw_circle((10, 10), 10.0, &paint);
}

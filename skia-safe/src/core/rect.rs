use crate::prelude::*;
use crate::private::safe32::{sk32, sk64};
use crate::{scalar, Contains, IPoint, ISize, IVector, Point, Size, Vector};
use skia_bindings as sb;
use skia_bindings::{SkIRect, SkRect};
use std::cmp::{max, min};
use std::mem;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct IRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl NativeTransmutable<SkIRect> for IRect {}

#[test]
fn test_irect_layout() {
    IRect::test_layout();
}

impl AsRef<IRect> for IRect {
    fn as_ref(&self) -> &IRect {
        self
    }
}

impl IRect {
    pub const fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub const fn new_empty() -> Self {
        Self::new(0, 0, 0, 0)
    }

    pub fn from_wh(w: i32, h: i32) -> Self {
        Self::from_size((w, h))
    }

    pub fn from_size(size: impl Into<ISize>) -> Self {
        let size = size.into();
        Self::new(0, 0, size.width, size.height)
    }

    pub const fn from_ltrb(l: i32, t: i32, r: i32, b: i32) -> Self {
        Self::new(l, t, r, b)
    }

    pub fn from_xywh(x: i32, y: i32, w: i32, h: i32) -> Self {
        IRect {
            left: x,
            top: y,
            right: sk32::sat_add(x, w),
            bottom: sk32::sat_add(y, h),
        }
    }

    pub fn left(&self) -> i32 {
        self.left
    }

    pub fn top(&self) -> i32 {
        self.top
    }

    pub fn right(&self) -> i32 {
        self.right
    }

    pub fn bottom(&self) -> i32 {
        self.bottom
    }

    pub fn x(&self) -> i32 {
        self.left
    }

    pub fn y(&self) -> i32 {
        self.top
    }

    pub fn width(&self) -> i32 {
        sk32::can_overflow_sub(self.right, self.left)
    }

    pub fn height(&self) -> i32 {
        sk32::can_overflow_sub(self.bottom, self.top)
    }

    pub fn size(&self) -> ISize {
        (self.width(), self.height()).into()
    }

    pub fn width_64(&self) -> i64 {
        i64::from(self.right) - i64::from(self.left)
    }

    pub fn height_64(&self) -> i64 {
        i64::from(self.bottom) - i64::from(self.top)
    }

    pub fn is_empty_64(&self) -> bool {
        self.right <= self.left || self.bottom <= self.top
    }

    pub fn is_empty(&self) -> bool {
        unsafe { sb::C_SkIRect_isEmpty(self.native()) }
    }

    pub fn set_empty(&mut self) {
        *self = Self::new_empty()
    }

    #[deprecated(since = "0.19.0", note = "use set_ltrb()")]
    pub fn set(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.set_ltrb(left, top, right, bottom)
    }

    pub fn set_ltrb(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        *self = Self::new(left, top, right, bottom);
    }

    pub fn set_xywh(&mut self, x: i32, y: i32, w: i32, h: i32) {
        *self = Self::from_xywh(x, y, w, h);
    }

    pub fn set_wh(&mut self, width: i32, height: i32) {
        self.left = 0;
        self.top = 0;
        self.right = width;
        self.bottom = height;
    }

    #[must_use]
    pub fn with_offset(&self, delta: impl Into<IVector>) -> Self {
        let mut copied = *self;
        copied.offset(delta);
        copied
    }

    #[must_use]
    pub fn with_inset(&self, delta: impl Into<IVector>) -> Self {
        self.with_outset(-delta.into())
    }

    #[must_use]
    pub fn with_outset(&self, delta: impl Into<IVector>) -> Self {
        let delta = delta.into();
        let (dx, dy) = (delta.x, delta.y);
        IRect::new(
            sk32::sat_sub(self.left, dx),
            sk32::sat_sub(self.top, dy),
            sk32::sat_add(self.right, dx),
            sk32::sat_add(self.bottom, dy),
        )
    }

    pub fn offset(&mut self, delta: impl Into<IPoint>) {
        let delta = delta.into();
        let (dx, dy) = (delta.x, delta.y);

        self.left = sk32::sat_add(self.left, dx);
        self.top = sk32::sat_add(self.top, dy);
        self.right = sk32::sat_add(self.right, dx);
        self.bottom = sk32::sat_add(self.bottom, dy);
    }

    pub fn offset_to(&mut self, new_p: impl Into<IPoint>) {
        *self = self.with_offset_to(new_p)
    }

    #[must_use]
    pub fn with_offset_to(&self, new_p: impl Into<IPoint>) -> Self {
        let new_p = new_p.into();
        let (new_x, new_y) = (new_p.x, new_p.y);

        IRect::new(
            sk64::pin_to_s32(i64::from(self.right) + i64::from(new_x) - i64::from(self.left)),
            sk64::pin_to_s32(i64::from(self.bottom) + i64::from(new_y) - i64::from(self.top)),
            new_x,
            new_y,
        )
    }

    pub fn inset(&mut self, delta: impl Into<IVector>) {
        *self = self.with_inset(delta)
    }

    pub fn outset(&mut self, delta: impl Into<IVector>) {
        *self = self.with_outset(delta)
    }

    #[must_use]
    pub fn with_adjustment(&self, d_l: i32, d_t: i32, d_r: i32, d_b: i32) -> Self {
        IRect::new(
            sk32::sat_add(self.left, d_l),
            sk32::sat_add(self.top, d_t),
            sk32::sat_add(self.right, d_r),
            sk32::sat_add(self.bottom, d_b),
        )
    }

    pub fn adjust(&mut self, d_l: i32, d_t: i32, d_r: i32, d_b: i32) {
        *self = self.with_adjustment(d_l, d_t, d_r, d_b)
    }

    // contains() is implemented through a trait below.

    pub fn contains_no_empty_check(&self, r: &Self) -> bool {
        debug_assert!(self.left < self.right && self.top < self.bottom);
        debug_assert!(r.left < r.right && r.top < r.bottom);

        self.left <= r.left && self.top <= r.top && self.right >= r.right && self.bottom >= r.bottom
    }

    pub fn intersect(a: &Self, b: &Self) -> Option<Self> {
        let mut r = Self::default();
        unsafe { r.native_mut().intersect(a.native(), b.native()) }.if_true_some(r)
    }

    #[deprecated(since = "0.19.0", note = "removed without alternative")]
    pub fn intersect_no_empty_check(a: &Self, b: &Self) -> Option<Self> {
        Self::intersect_no_empty_check_(a, b)
    }

    pub fn intersects(a: &Self, b: &Self) -> bool {
        Self::intersect(a, b).is_some()
    }

    #[deprecated(since = "0.19.0", note = "removed without alternative")]
    pub fn intersects_no_empty_check(a: &Self, b: &Self) -> bool {
        Self::intersect_no_empty_check_(a, b).is_some()
    }

    fn intersect_no_empty_check_(a: &Self, b: &Self) -> Option<Self> {
        debug_assert!(!a.is_empty_64() && !b.is_empty_64());
        let r = IRect::new(
            max(a.left, b.left),
            max(a.top, b.top),
            min(a.right, b.right),
            min(a.bottom, b.bottom),
        );
        r.is_empty().if_false_some(r)
    }

    pub fn join(a: &Self, b: &Self) -> Self {
        let mut copied = *a;
        unsafe { copied.native_mut().join(b.native()) }
        copied
    }

    pub fn sort(&mut self) {
        *self = self.sorted()
    }

    #[must_use]
    pub fn sorted(&self) -> Self {
        Self::new(
            min(self.left, self.right),
            min(self.top, self.bottom),
            max(self.left, self.right),
            max(self.top, self.bottom),
        )
    }

    #[deprecated(since = "0.27.0", note = "removed without replacement")]
    #[must_use]
    pub fn empty() -> &'static Self {
        &EMPTY_IRECT
    }
}

pub const EMPTY_IRECT: IRect = IRect {
    left: 0,
    top: 0,
    right: 0,
    bottom: 0,
};

impl Contains<IPoint> for IRect {
    fn contains(&self, other: IPoint) -> bool {
        let (x, y) = (other.x, other.y);
        x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }
}

impl Contains<&IRect> for IRect {
    fn contains(&self, r: &IRect) -> bool {
        !r.is_empty()
            && !self.is_empty()
            && self.left <= r.left
            && self.top <= r.top
            && self.right >= r.right
            && self.bottom >= r.bottom
    }
}

impl Contains<&Rect> for IRect {
    fn contains(&self, other: &Rect) -> bool {
        unsafe { sb::C_SkIRect_contains(self.native(), other.native()) }
    }
}

impl Contains<IRect> for IRect {
    fn contains(&self, other: IRect) -> bool {
        self.contains(&other)
    }
}

impl Contains<Rect> for IRect {
    fn contains(&self, other: Rect) -> bool {
        self.contains(&other)
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rect {
    pub left: scalar,
    pub top: scalar,
    pub right: scalar,
    pub bottom: scalar,
}

impl NativeTransmutable<SkRect> for Rect {}

#[test]
fn test_rect_layout() {
    Rect::test_layout();
}

impl AsRef<Rect> for Rect {
    fn as_ref(&self) -> &Rect {
        self
    }
}

impl Rect {
    pub fn new(left: scalar, top: scalar, right: scalar, bottom: scalar) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn new_empty() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn from_wh(w: scalar, h: scalar) -> Self {
        Self::new(0.0, 0.0, w, h)
    }

    pub fn from_iwh(w: i32, h: i32) -> Self {
        Self::from_wh(w as scalar, h as scalar)
    }

    pub fn from_size(size: impl Into<Size>) -> Self {
        (Point::default(), size.into()).into()
    }

    pub fn from_xywh(x: scalar, y: scalar, w: scalar, h: scalar) -> Self {
        Self::new(x, y, x + w, y + h)
    }

    pub fn from_point_and_size(p: impl Into<Point>, sz: impl Into<Size>) -> Self {
        (p.into(), sz.into()).into()
    }

    pub fn from_isize(isize: impl Into<ISize>) -> Self {
        let isize = isize.into();
        Self::from_iwh(isize.width, isize.height)
    }

    pub fn from_irect(irect: impl AsRef<IRect>) -> Self {
        let irect = irect.as_ref();
        Self::new(
            irect.left as scalar,
            irect.top as scalar,
            irect.right as scalar,
            irect.bottom as scalar,
        )
    }

    pub fn is_empty(&self) -> bool {
        // We write it as the NOT of a non-empty rect, so we will return true if any values
        // are NaN.
        !(self.left < self.right && self.top < self.bottom)
    }

    pub fn is_sorted(&self) -> bool {
        self.left <= self.right && self.top <= self.bottom
    }

    pub fn is_finite(&self) -> bool {
        let mut accum: f32 = 0.0;
        accum *= self.left;
        accum *= self.top;
        accum *= self.right;
        accum *= self.bottom;

        // accum is either NaN or it is finite (zero).
        debug_assert!(0.0 == accum || accum.is_nan());

        // value==value will be true iff value is not NaN
        // TODO: is it faster to say !accum or accum==accum?
        !accum.is_nan()
    }

    pub fn x(&self) -> scalar {
        self.left
    }

    pub fn y(&self) -> scalar {
        self.top
    }

    pub fn left(&self) -> scalar {
        self.left
    }

    pub fn top(&self) -> scalar {
        self.top
    }

    pub fn right(&self) -> scalar {
        self.right
    }

    pub fn bottom(&self) -> scalar {
        self.bottom
    }

    pub fn size(&self) -> Size {
        (self.width(), self.height()).into()
    }

    pub fn width(&self) -> scalar {
        self.native().fRight - self.native().fLeft
    }

    pub fn height(&self) -> scalar {
        self.native().fBottom - self.native().fTop
    }

    pub fn center(&self) -> Point {
        Point::from((self.center_x(), self.center_y()))
    }

    pub fn center_x(&self) -> scalar {
        // don't use (fLeft + fBottom) * 0.5 as that might overflow before the 0.5
        self.left * 0.5 + self.right * 0.5
    }

    pub fn center_y(&self) -> scalar {
        // don't use (fTop + fBottom) * 0.5 as that might overflow before the 0.5
        self.top * 0.5 + self.bottom * 0.5
    }

    pub fn to_quad(&self) -> [Point; 4] {
        let mut quad = [Point::default(); 4];
        unsafe { self.native().toQuad(quad.native_mut().as_mut_ptr()) }
        quad
    }

    pub fn set_empty(&mut self) {
        *self = Self::new_empty()
    }

    // TODO: deprecate and rename to set() as soon the other set() variant is removed.
    pub fn set_irect(&mut self, irect: impl AsRef<IRect>) {
        *self = Self::from_irect(irect)
    }

    #[deprecated(since = "0.19.0", note = "use set_ltrb()")]
    pub fn set(&mut self, left: scalar, top: scalar, right: scalar, bottom: scalar) {
        self.set_ltrb(left, top, right, bottom)
    }

    pub fn set_ltrb(&mut self, left: scalar, top: scalar, right: scalar, bottom: scalar) {
        *self = Self::new(left, top, right, bottom)
    }

    #[deprecated(since = "0.19.0", note = "use from_irect()")]
    pub fn iset(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        *self = Self::from_irect(IRect::new(left, top, right, bottom))
    }

    #[deprecated(since = "0.19.0", note = "use from_isize()")]
    pub fn iset_wh(&mut self, width: i32, height: i32) {
        *self = Self::from_isize(ISize::new(width, height))
    }

    pub fn set_bounds(&mut self, points: &[Point]) {
        unsafe {
            self.native_mut()
                .setBoundsCheck(points.native().as_ptr(), points.len().try_into().unwrap());
        }
    }

    pub fn set_bounds_check(&mut self, points: &[Point]) -> bool {
        unsafe {
            self.native_mut()
                .setBoundsCheck(points.native().as_ptr(), points.len().try_into().unwrap())
        }
    }

    pub fn set_bounds_no_check(&mut self, points: &[Point]) {
        unsafe {
            self.native_mut()
                .setBoundsNoCheck(points.native().as_ptr(), points.len().try_into().unwrap())
        }
    }

    pub fn set_bounds2(&mut self, p0: impl Into<Point>, p1: impl Into<Point>) {
        let (p0, p1) = (p0.into(), p1.into());
        self.left = p0.x.min(p1.x);
        self.right = p0.x.max(p1.x);
        self.top = p0.y.min(p1.y);
        self.bottom = p0.y.max(p1.y);
    }

    pub fn from_bounds(points: &[Point]) -> Option<Self> {
        let mut r = Self::default();
        unsafe {
            r.native_mut()
                .setBoundsCheck(points.native().as_ptr(), points.len().try_into().unwrap())
        }
        .if_true_some(r)
    }

    pub fn set_xywh(&mut self, x: scalar, y: scalar, width: scalar, height: scalar) {
        *self = Self::from_xywh(x, y, width, height)
    }

    pub fn set_wh(&mut self, w: scalar, h: scalar) {
        *self = Self::from_wh(w, h)
    }

    pub fn set_iwh(&mut self, width: i32, height: i32) {
        *self = Self::from_iwh(width, height)
    }

    pub fn with_offset(&self, d: impl Into<Vector>) -> Self {
        let d = d.into();
        Self::new(
            self.left + d.x,
            self.top + d.y,
            self.right + d.x,
            self.bottom + d.y,
        )
    }

    pub fn with_inset(&self, d: impl Into<Vector>) -> Self {
        let d = d.into();
        Self::new(
            self.left + d.x,
            self.top + d.y,
            self.right - d.x,
            self.bottom - d.y,
        )
    }

    pub fn with_outset(&self, d: impl Into<Vector>) -> Self {
        let d = d.into();
        Self::new(
            self.left - d.x,
            self.top - d.y,
            self.right + d.x,
            self.bottom + d.y,
        )
    }

    pub fn offset(&mut self, d: impl Into<Vector>) {
        *self = self.with_offset(d)
    }

    pub fn offset_to(&mut self, new_p: impl Into<Point>) {
        *self = self.with_offset_to(new_p)
    }

    pub fn with_offset_to(&self, new_p: impl Into<Point>) -> Self {
        let new_p = new_p.into();
        Self::new(new_p.x, new_p.y, new_p.x - self.left, new_p.y - self.top)
    }

    pub fn inset(&mut self, d: impl Into<Vector>) {
        *self = self.with_inset(d)
    }

    pub fn outset(&mut self, d: impl Into<Vector>) {
        *self = self.with_outset(d)
    }

    pub fn intersect(&mut self, r: impl AsRef<Rect>) -> bool {
        unsafe { self.native_mut().intersect(r.as_ref().native()) }
    }

    #[deprecated(since = "0.19.0", note = "use intersect()")]
    pub fn intersect_ltrb(
        &mut self,
        left: scalar,
        top: scalar,
        right: scalar,
        bottom: scalar,
    ) -> bool {
        self.intersect(Rect::new(left, top, right, bottom))
    }

    #[must_use]
    pub fn intersect2(&mut self, a: impl AsRef<Rect>, b: impl AsRef<Rect>) -> bool {
        unsafe {
            self.native_mut()
                .intersect1(a.as_ref().native(), b.as_ref().native())
        }
    }

    #[deprecated(since = "0.19.0", note = "use intersects()")]
    pub fn intersects_ltrb(
        &self,
        left: scalar,
        top: scalar,
        right: scalar,
        bottom: scalar,
    ) -> bool {
        Self::intersects_(
            self.left,
            self.top,
            self.right,
            self.bottom,
            left,
            top,
            right,
            bottom,
        )
    }

    pub fn intersects(&self, r: impl AsRef<Rect>) -> bool {
        let r = r.as_ref();
        Self::intersects_(
            self.left,
            self.top,
            self.right,
            self.bottom,
            r.left,
            r.top,
            r.right,
            r.bottom,
        )
    }

    pub fn intersects2(a: impl AsRef<Rect>, b: impl AsRef<Rect>) -> bool {
        a.as_ref().intersects(b)
    }

    #[allow(clippy::too_many_arguments)]
    fn intersects_(
        al: scalar,
        at: scalar,
        ar: scalar,
        ab: scalar,
        bl: scalar,
        bt: scalar,
        br: scalar,
        bb: scalar,
    ) -> bool {
        let l = al.max(bl);
        let r = ar.min(br);
        let t = at.max(bt);
        let b = ab.min(bb);
        l < r && t < b
    }

    #[deprecated(since = "0.19.0", note = "use join()")]
    pub fn join_ltrb(&mut self, left: scalar, top: scalar, right: scalar, bottom: scalar) {
        self.join(Rect::new(left, top, right, bottom))
    }

    pub fn join(&mut self, r: impl AsRef<Rect>) {
        let r = r.as_ref();
        unsafe { self.native_mut().join(r.native()) }
    }

    pub fn join2(a: impl AsRef<Rect>, b: impl AsRef<Rect>) -> Rect {
        let mut result = *a.as_ref();
        result.join(b);
        result
    }

    pub fn join_non_empty_arg(&mut self, r: impl AsRef<Rect>) {
        let r = r.as_ref();
        debug_assert!(!r.is_empty());
        if self.left >= self.right || self.top >= self.bottom {
            *self = *r;
        } else {
            self.join_possibly_empty_rect(r);
        }
    }

    pub fn join_possibly_empty_rect(&mut self, r: impl AsRef<Rect>) {
        let r = r.as_ref();
        self.left = self.left.min(r.left);
        self.top = self.top.min(r.top);
        self.right = self.right.max(r.right);
        self.bottom = self.bottom.max(r.bottom);
    }

    // The set of contains() functions are defined as a trait below.

    #[must_use]
    pub fn round(&self) -> IRect {
        let mut r = IRect::default();
        unsafe { sb::C_SkRect_round(self.native(), r.native_mut()) };
        r
    }

    // The functions round_out() are defined as a trait below.

    #[must_use]
    pub fn round_in(&self) -> IRect {
        let mut r = IRect::default();
        unsafe { sb::C_SkRect_roundIn(self.native(), r.native_mut()) };
        r
    }

    pub fn sort(&mut self) {
        if self.left > self.right {
            mem::swap(&mut self.left, &mut self.right);
        }

        if self.top > self.bottom {
            mem::swap(&mut self.top, &mut self.bottom);
        }
    }

    #[must_use]
    pub fn sorted(&self) -> Rect {
        Rect::new(
            self.left.min(self.right),
            self.top.min(self.bottom),
            self.left.max(self.right),
            self.top.max(self.bottom),
        )
    }

    pub fn as_scalars(&self) -> &[scalar; 4] {
        unsafe { transmute_ref(&self.left) }
    }

    pub fn dump(&self, as_hex: impl Into<Option<bool>>) {
        unsafe { self.native().dump(as_hex.into().unwrap_or_default()) }
    }

    pub fn dump_hex(&self) {
        self.dump(true)
    }
}

impl Contains<Point> for Rect {
    fn contains(&self, p: Point) -> bool {
        p.x >= self.left && p.x < self.right && p.y >= self.top && p.y < self.bottom
    }
}

impl Contains<Rect> for Rect {
    fn contains(&self, r: Rect) -> bool {
        // todo: can we eliminate the this->isempty check?
        !r.is_empty()
            && !self.is_empty()
            && self.left <= r.left
            && self.top <= r.top
            && self.right >= r.right
            && self.bottom >= r.bottom
    }
}

impl Contains<IRect> for Rect {
    fn contains(&self, r: IRect) -> bool {
        // todo: can we eliminate the this->isEmpty check?
        !r.is_empty()
            && !self.is_empty()
            && self.left <= r.left as scalar
            && self.top <= r.top as scalar
            && self.right >= r.right as scalar
            && self.bottom >= r.bottom as scalar
    }
}

#[test]
fn contains_overloads_compile() {
    let r = Rect::default();
    r.contains(Point::default());
    r.contains(Rect::default());
    r.contains(IRect::default());
}

pub trait RoundOut<R> {
    fn round_out(&self) -> R;
}

impl RoundOut<IRect> for Rect {
    fn round_out(&self) -> IRect {
        let mut r = IRect::default();
        unsafe { sb::C_SkRect_roundOut(self.native(), r.native_mut()) };
        r
    }
}

impl RoundOut<Rect> for Rect {
    fn round_out(&self) -> Rect {
        Rect::new(
            self.left.floor(),
            self.top.floor(),
            self.right.ceil(),
            self.bottom.ceil(),
        )
    }
}

//
// From
//

impl From<(IPoint, ISize)> for IRect {
    fn from((point, size): (IPoint, ISize)) -> Self {
        IRect::new(
            point.x,
            point.y,
            point.x + size.width,
            point.y + size.height,
        )
    }
}

impl From<(Point, Size)> for Rect {
    fn from((point, size): (Point, Size)) -> Self {
        Rect::new(
            point.x,
            point.y,
            point.x + size.width,
            point.y + size.height,
        )
    }
}

impl From<ISize> for Rect {
    fn from(isize: ISize) -> Self {
        Self::from_isize(isize)
    }
}
impl From<IRect> for Rect {
    fn from(irect: IRect) -> Self {
        Self::from_irect(irect)
    }
}

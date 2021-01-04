#[doc = "Reader of register PE"]
pub type R = crate::R<u32, super::PE>;
#[doc = "Writer for register PE"]
pub type W = crate::W<u32, super::PE>;
#[doc = "Register PE `reset()`'s with value 0"]
impl crate::ResetValue for super::PE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "KBI Pin Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum KBIPE_A {
    #[doc = "0: Pin is not enabled as KBI interrupt."]
    _0 = 0,
    #[doc = "1: Pin is enabled as KBI interrupt."]
    _1 = 1,
}
impl From<KBIPE_A> for u32 {
    #[inline(always)]
    fn from(variant: KBIPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KBIPE`"]
pub type KBIPE_R = crate::R<u32, KBIPE_A>;
impl KBIPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, KBIPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(KBIPE_A::_0),
            1 => Val(KBIPE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KBIPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KBIPE_A::_1
    }
}
#[doc = "Write proxy for field `KBIPE`"]
pub struct KBIPE_W<'a> {
    w: &'a mut W,
}
impl<'a> KBIPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBIPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pin is not enabled as KBI interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBIPE_A::_0)
    }
    #[doc = "Pin is enabled as KBI interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBIPE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - KBI Pin Enables"]
    #[inline(always)]
    pub fn kbipe(&self) -> KBIPE_R {
        KBIPE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - KBI Pin Enables"]
    #[inline(always)]
    pub fn kbipe(&mut self) -> KBIPE_W {
        KBIPE_W { w: self }
    }
}

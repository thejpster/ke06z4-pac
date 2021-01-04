#[doc = "Reader of register ES"]
pub type R = crate::R<u32, super::ES>;
#[doc = "Writer for register ES"]
pub type W = crate::W<u32, super::ES>;
#[doc = "Register ES `reset()`'s with value 0"]
impl crate::ResetValue for super::ES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "KBI Edge Selects\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum KBEDG_A {
    #[doc = "0: Falling edge/low level."]
    _0 = 0,
    #[doc = "1: Rising edge/high level."]
    _1 = 1,
}
impl From<KBEDG_A> for u32 {
    #[inline(always)]
    fn from(variant: KBEDG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KBEDG`"]
pub type KBEDG_R = crate::R<u32, KBEDG_A>;
impl KBEDG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, KBEDG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(KBEDG_A::_0),
            1 => Val(KBEDG_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KBEDG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KBEDG_A::_1
    }
}
#[doc = "Write proxy for field `KBEDG`"]
pub struct KBEDG_W<'a> {
    w: &'a mut W,
}
impl<'a> KBEDG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBEDG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Falling edge/low level."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBEDG_A::_0)
    }
    #[doc = "Rising edge/high level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBEDG_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - KBI Edge Selects"]
    #[inline(always)]
    pub fn kbedg(&self) -> KBEDG_R {
        KBEDG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - KBI Edge Selects"]
    #[inline(always)]
    pub fn kbedg(&mut self) -> KBEDG_W {
        KBEDG_W { w: self }
    }
}

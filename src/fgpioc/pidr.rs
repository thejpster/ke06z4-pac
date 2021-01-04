#[doc = "Reader of register PIDR"]
pub type R = crate::R<u32, super::PIDR>;
#[doc = "Writer for register PIDR"]
pub type W = crate::W<u32, super::PIDR>;
#[doc = "Register PIDR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PIDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PID_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input. Corresponding Port Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID_A> for u32 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u32, PID_A>;
impl PID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PID_A::_0),
            1 => Val(PID_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PID_A::_1
    }
}
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input. Corresponding Port Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
}

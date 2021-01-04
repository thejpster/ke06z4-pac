#[doc = "Reader of register APCTL1"]
pub type R = crate::R<u32, super::APCTL1>;
#[doc = "Writer for register APCTL1"]
pub type W = crate::W<u32, super::APCTL1>;
#[doc = "Register APCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::APCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC Pin Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ADPC_A {
    #[doc = "0: ADx pin I/O control enabled."]
    _0 = 0,
    #[doc = "1: ADx pin I/O control disabled."]
    _1 = 1,
}
impl From<ADPC_A> for u16 {
    #[inline(always)]
    fn from(variant: ADPC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADPC`"]
pub type ADPC_R = crate::R<u16, ADPC_A>;
impl ADPC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ADPC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADPC_A::_0),
            1 => Val(ADPC_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADPC_A::_1
    }
}
#[doc = "Write proxy for field `ADPC`"]
pub struct ADPC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADPC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADx pin I/O control enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADPC_A::_0)
    }
    #[doc = "ADx pin I/O control disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADPC_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ADC Pin Control"]
    #[inline(always)]
    pub fn adpc(&self) -> ADPC_R {
        ADPC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC Pin Control"]
    #[inline(always)]
    pub fn adpc(&mut self) -> ADPC_W {
        ADPC_W { w: self }
    }
}

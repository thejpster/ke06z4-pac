#[doc = "Reader of register CANIDMR2"]
pub type R = crate::R<u8, super::CANIDMR2>;
#[doc = "Writer for register CANIDMR2"]
pub type W = crate::W<u8, super::CANIDMR2>;
#[doc = "Register CANIDMR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CANIDMR2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Acceptance Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AM_A {
    #[doc = "0: Match corresponding acceptance code register and identifier bits."]
    _0 = 0,
    #[doc = "1: Ignore corresponding acceptance code register bit."]
    _1 = 1,
}
impl From<AM_A> for u8 {
    #[inline(always)]
    fn from(variant: AM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AM`"]
pub type AM_R = crate::R<u8, AM_A>;
impl AM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AM_A::_0),
            1 => Val(AM_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AM_A::_1
    }
}
#[doc = "Write proxy for field `AM`"]
pub struct AM_W<'a> {
    w: &'a mut W,
}
impl<'a> AM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Match corresponding acceptance code register and identifier bits."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AM_A::_0)
    }
    #[doc = "Ignore corresponding acceptance code register bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AM_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Acceptance Mask Bits"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance Mask Bits"]
    #[inline(always)]
    pub fn am(&mut self) -> AM_W {
        AM_W { w: self }
    }
}

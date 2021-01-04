#[doc = "Reader of register C2"]
pub type R = crate::R<u8, super::C2>;
#[doc = "Writer for register C2"]
pub type W = crate::W<u8, super::C2>;
#[doc = "Register C2 `reset()`'s with value 0"]
impl crate::ResetValue for super::C2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ACMP Input Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACIPE_A {
    #[doc = "0: The corresponding external analog input is not allowed."]
    _0 = 0,
    #[doc = "1: The corresponding external analog input is allowed."]
    _1 = 1,
}
impl From<ACIPE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACIPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACIPE`"]
pub type ACIPE_R = crate::R<u8, ACIPE_A>;
impl ACIPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ACIPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ACIPE_A::_0),
            1 => Val(ACIPE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACIPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACIPE_A::_1
    }
}
#[doc = "Write proxy for field `ACIPE`"]
pub struct ACIPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACIPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACIPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding external analog input is not allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACIPE_A::_0)
    }
    #[doc = "The corresponding external analog input is allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACIPE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - ACMP Input Pin Enable"]
    #[inline(always)]
    pub fn acipe(&self) -> ACIPE_R {
        ACIPE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ACMP Input Pin Enable"]
    #[inline(always)]
    pub fn acipe(&mut self) -> ACIPE_W {
        ACIPE_W { w: self }
    }
}

#[doc = "Reader of register CANTARQ"]
pub type R = crate::R<u8, super::CANTARQ>;
#[doc = "Writer for register CANTARQ"]
pub type W = crate::W<u8, super::CANTARQ>;
#[doc = "Register CANTARQ `reset()`'s with value 0"]
impl crate::ResetValue for super::CANTARQ {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Abort Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ABTRQ_A {
    #[doc = "0: No abort request."]
    _0 = 0,
    #[doc = "1: Abort request pending."]
    _1 = 1,
}
impl From<ABTRQ_A> for u8 {
    #[inline(always)]
    fn from(variant: ABTRQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ABTRQ`"]
pub type ABTRQ_R = crate::R<u8, ABTRQ_A>;
impl ABTRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ABTRQ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ABTRQ_A::_0),
            1 => Val(ABTRQ_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABTRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABTRQ_A::_1
    }
}
#[doc = "Write proxy for field `ABTRQ`"]
pub struct ABTRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTRQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No abort request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABTRQ_A::_0)
    }
    #[doc = "Abort request pending."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABTRQ_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Abort Request"]
    #[inline(always)]
    pub fn abtrq(&self) -> ABTRQ_R {
        ABTRQ_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Abort Request"]
    #[inline(always)]
    pub fn abtrq(&mut self) -> ABTRQ_W {
        ABTRQ_W { w: self }
    }
}

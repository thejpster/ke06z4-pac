#[doc = "Reader of register C4"]
pub type R = crate::R<u8, super::C4>;
#[doc = "Writer for register C4"]
pub type W = crate::W<u8, super::C4>;
#[doc = "Register C4 `reset()`'s with value 0"]
impl crate::ResetValue for super::C4 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCFTRIM`"]
pub type SCFTRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCFTRIM`"]
pub struct SCFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCFTRIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Clock Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CME_A {
    #[doc = "0: Clock monitor is disabled."]
    _0 = 0,
    #[doc = "1: Generates a reset request on loss of external clock."]
    _1 = 1,
}
impl From<CME_A> for bool {
    #[inline(always)]
    fn from(variant: CME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CME`"]
pub type CME_R = crate::R<bool, CME_A>;
impl CME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CME_A {
        match self.bits {
            false => CME_A::_0,
            true => CME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CME_A::_1
    }
}
#[doc = "Write proxy for field `CME`"]
pub struct CME_W<'a> {
    w: &'a mut W,
}
impl<'a> CME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock monitor is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CME_A::_0)
    }
    #[doc = "Generates a reset request on loss of external clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CME_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Loss of Lock Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLIE_A {
    #[doc = "0: No request on loss of lock."]
    _0 = 0,
    #[doc = "1: Generates an interrupt request on loss of lock."]
    _1 = 1,
}
impl From<LOLIE_A> for bool {
    #[inline(always)]
    fn from(variant: LOLIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOLIE`"]
pub type LOLIE_R = crate::R<bool, LOLIE_A>;
impl LOLIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOLIE_A {
        match self.bits {
            false => LOLIE_A::_0,
            true => LOLIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOLIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOLIE_A::_1
    }
}
#[doc = "Write proxy for field `LOLIE`"]
pub struct LOLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOLIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOLIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No request on loss of lock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLIE_A::_0)
    }
    #[doc = "Generates an interrupt request on loss of lock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLIE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slow Internal Reference Clock Fine Trim"]
    #[inline(always)]
    pub fn scftrim(&self) -> SCFTRIM_R {
        SCFTRIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme(&self) -> CME_R {
        CME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loss of Lock Interrupt"]
    #[inline(always)]
    pub fn lolie(&self) -> LOLIE_R {
        LOLIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slow Internal Reference Clock Fine Trim"]
    #[inline(always)]
    pub fn scftrim(&mut self) -> SCFTRIM_W {
        SCFTRIM_W { w: self }
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme(&mut self) -> CME_W {
        CME_W { w: self }
    }
    #[doc = "Bit 7 - Loss of Lock Interrupt"]
    #[inline(always)]
    pub fn lolie(&mut self) -> LOLIE_W {
        LOLIE_W { w: self }
    }
}

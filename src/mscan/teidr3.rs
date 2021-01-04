#[doc = "Reader of register TEIDR3"]
pub type R = crate::R<u8, super::TEIDR3>;
#[doc = "Writer for register TEIDR3"]
pub type W = crate::W<u8, super::TEIDR3>;
#[doc = "Register TEIDR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TEIDR3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Remote Transmission Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERTR_A {
    #[doc = "0: Data frame."]
    _0 = 0,
    #[doc = "1: Remote frame."]
    _1 = 1,
}
impl From<TERTR_A> for bool {
    #[inline(always)]
    fn from(variant: TERTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TERTR`"]
pub type TERTR_R = crate::R<bool, TERTR_A>;
impl TERTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TERTR_A {
        match self.bits {
            false => TERTR_A::_0,
            true => TERTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TERTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TERTR_A::_1
    }
}
#[doc = "Write proxy for field `TERTR`"]
pub struct TERTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TERTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TERTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TERTR_A::_0)
    }
    #[doc = "Remote frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TERTR_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TEID6_TEID0`"]
pub type TEID6_TEID0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEID6_TEID0`"]
pub struct TEID6_TEID0_W<'a> {
    w: &'a mut W,
}
impl<'a> TEID6_TEID0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u8) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Remote Transmission Request"]
    #[inline(always)]
    pub fn tertr(&self) -> TERTR_R {
        TERTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - Extended Format Identifier 6-0"]
    #[inline(always)]
    pub fn teid6_teid0(&self) -> TEID6_TEID0_R {
        TEID6_TEID0_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Remote Transmission Request"]
    #[inline(always)]
    pub fn tertr(&mut self) -> TERTR_W {
        TERTR_W { w: self }
    }
    #[doc = "Bits 1:7 - Extended Format Identifier 6-0"]
    #[inline(always)]
    pub fn teid6_teid0(&mut self) -> TEID6_TEID0_W {
        TEID6_TEID0_W { w: self }
    }
}

#[doc = "Reader of register TEIDR1"]
pub type R = crate::R<u8, super::TEIDR1>;
#[doc = "Writer for register TEIDR1"]
pub type W = crate::W<u8, super::TEIDR1>;
#[doc = "Register TEIDR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TEIDR1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TEID17_TEID15`"]
pub type TEID17_TEID15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEID17_TEID15`"]
pub struct TEID17_TEID15_W<'a> {
    w: &'a mut W,
}
impl<'a> TEID17_TEID15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "ID Extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIDE_A {
    #[doc = "0: Standard format (11 bit)."]
    _0 = 0,
    #[doc = "1: Extended format (29 bit)."]
    _1 = 1,
}
impl From<TEIDE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIDE`"]
pub type TEIDE_R = crate::R<bool, TEIDE_A>;
impl TEIDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIDE_A {
        match self.bits {
            false => TEIDE_A::_0,
            true => TEIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEIDE_A::_1
    }
}
#[doc = "Write proxy for field `TEIDE`"]
pub struct TEIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard format (11 bit)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEIDE_A::_0)
    }
    #[doc = "Extended format (29 bit)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEIDE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TSRR`"]
pub type TSRR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSRR`"]
pub struct TSRR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSRR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TEID20_TEID18`"]
pub type TEID20_TEID18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEID20_TEID18`"]
pub struct TEID20_TEID18_W<'a> {
    w: &'a mut W,
}
impl<'a> TEID20_TEID18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u8) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Extended Format Identifier 17-15"]
    #[inline(always)]
    pub fn teid17_teid15(&self) -> TEID17_TEID15_R {
        TEID17_TEID15_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - ID Extended"]
    #[inline(always)]
    pub fn teide(&self) -> TEIDE_R {
        TEIDE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Substitute Remote Request"]
    #[inline(always)]
    pub fn tsrr(&self) -> TSRR_R {
        TSRR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Extended Format Identifier 20-18"]
    #[inline(always)]
    pub fn teid20_teid18(&self) -> TEID20_TEID18_R {
        TEID20_TEID18_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Extended Format Identifier 17-15"]
    #[inline(always)]
    pub fn teid17_teid15(&mut self) -> TEID17_TEID15_W {
        TEID17_TEID15_W { w: self }
    }
    #[doc = "Bit 3 - ID Extended"]
    #[inline(always)]
    pub fn teide(&mut self) -> TEIDE_W {
        TEIDE_W { w: self }
    }
    #[doc = "Bit 4 - Substitute Remote Request"]
    #[inline(always)]
    pub fn tsrr(&mut self) -> TSRR_W {
        TSRR_W { w: self }
    }
    #[doc = "Bits 5:7 - Extended Format Identifier 20-18"]
    #[inline(always)]
    pub fn teid20_teid18(&mut self) -> TEID20_TEID18_W {
        TEID20_TEID18_W { w: self }
    }
}

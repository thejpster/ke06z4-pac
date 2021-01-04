#[doc = "Reader of register TSIDR1"]
pub type R = crate::R<u8, super::TSIDR1>;
#[doc = "Writer for register TSIDR1"]
pub type W = crate::W<u8, super::TSIDR1>;
#[doc = "Register TSIDR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TSIDR1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ID Extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIDE_A {
    #[doc = "0: Standard format (11 bit)."]
    _0 = 0,
    #[doc = "1: Extended format (29 bit)."]
    _1 = 1,
}
impl From<TSIDE_A> for bool {
    #[inline(always)]
    fn from(variant: TSIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSIDE`"]
pub type TSIDE_R = crate::R<bool, TSIDE_A>;
impl TSIDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIDE_A {
        match self.bits {
            false => TSIDE_A::_0,
            true => TSIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSIDE_A::_1
    }
}
#[doc = "Write proxy for field `TSIDE`"]
pub struct TSIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard format (11 bit)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIDE_A::_0)
    }
    #[doc = "Extended format (29 bit)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIDE_A::_1)
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
#[doc = "Remote Transmission Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSRTR_A {
    #[doc = "0: Data frame."]
    _0 = 0,
    #[doc = "1: Remote frame."]
    _1 = 1,
}
impl From<TSRTR_A> for bool {
    #[inline(always)]
    fn from(variant: TSRTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSRTR`"]
pub type TSRTR_R = crate::R<bool, TSRTR_A>;
impl TSRTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSRTR_A {
        match self.bits {
            false => TSRTR_A::_0,
            true => TSRTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSRTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSRTR_A::_1
    }
}
#[doc = "Write proxy for field `TSRTR`"]
pub struct TSRTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSRTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSRTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSRTR_A::_0)
    }
    #[doc = "Remote frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSRTR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TSID2_TSID0`"]
pub type TSID2_TSID0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSID2_TSID0`"]
pub struct TSID2_TSID0_W<'a> {
    w: &'a mut W,
}
impl<'a> TSID2_TSID0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u8) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - ID Extended"]
    #[inline(always)]
    pub fn tside(&self) -> TSIDE_R {
        TSIDE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Remote Transmission Request"]
    #[inline(always)]
    pub fn tsrtr(&self) -> TSRTR_R {
        TSRTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Standard Format Identifier 2-0"]
    #[inline(always)]
    pub fn tsid2_tsid0(&self) -> TSID2_TSID0_R {
        TSID2_TSID0_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - ID Extended"]
    #[inline(always)]
    pub fn tside(&mut self) -> TSIDE_W {
        TSIDE_W { w: self }
    }
    #[doc = "Bit 4 - Remote Transmission Request"]
    #[inline(always)]
    pub fn tsrtr(&mut self) -> TSRTR_W {
        TSRTR_W { w: self }
    }
    #[doc = "Bits 5:7 - Standard Format Identifier 2-0"]
    #[inline(always)]
    pub fn tsid2_tsid0(&mut self) -> TSID2_TSID0_W {
        TSID2_TSID0_W { w: self }
    }
}

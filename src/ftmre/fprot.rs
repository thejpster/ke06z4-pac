#[doc = "Reader of register FPROT"]
pub type R = crate::R<u8, super::FPROT>;
#[doc = "Writer for register FPROT"]
pub type W = crate::W<u8, super::FPROT>;
#[doc = "Register FPROT `reset()`'s with value 0"]
impl crate::ResetValue for super::FPROT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPLS`"]
pub type FPLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FPLS`"]
pub struct FPLS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Flash Protection Lower Address Range Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPLDIS_A {
    #[doc = "0: Protection/Unprotection enabled."]
    _0 = 0,
    #[doc = "1: Protection/Unprotection disabled."]
    _1 = 1,
}
impl From<FPLDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FPLDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FPLDIS`"]
pub type FPLDIS_R = crate::R<bool, FPLDIS_A>;
impl FPLDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPLDIS_A {
        match self.bits {
            false => FPLDIS_A::_0,
            true => FPLDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FPLDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FPLDIS_A::_1
    }
}
#[doc = "Write proxy for field `FPLDIS`"]
pub struct FPLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPLDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPLDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection/Unprotection enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPLDIS_A::_0)
    }
    #[doc = "Protection/Unprotection disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPLDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FPHS`"]
pub type FPHS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FPHS`"]
pub struct FPHS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPHS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u8) & 0x03) << 3);
        self.w
    }
}
#[doc = "Flash Protection Higher Address Range Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPHDIS_A {
    #[doc = "0: Protection/Unprotection enabled."]
    _0 = 0,
    #[doc = "1: Protection/Unprotection disabled."]
    _1 = 1,
}
impl From<FPHDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FPHDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FPHDIS`"]
pub type FPHDIS_R = crate::R<bool, FPHDIS_A>;
impl FPHDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPHDIS_A {
        match self.bits {
            false => FPHDIS_A::_0,
            true => FPHDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FPHDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FPHDIS_A::_1
    }
}
#[doc = "Write proxy for field `FPHDIS`"]
pub struct FPHDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPHDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPHDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection/Unprotection enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPHDIS_A::_0)
    }
    #[doc = "Protection/Unprotection disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPHDIS_A::_1)
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
#[doc = "Reader of field `RNV6`"]
pub type RNV6_R = crate::R<bool, bool>;
#[doc = "Flash Protection Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPOPEN_A {
    #[doc = "0: When FPOPEN is clear, the FPHDIS and FPLDIS fields define unprotected address ranges as specified by the corresponding FPHS and FPLS fields."]
    _0 = 0,
    #[doc = "1: When FPOPEN is set, the FPHDIS and FPLDIS fields enable protection for the address range specified by the corresponding FPHS and FPLS fields."]
    _1 = 1,
}
impl From<FPOPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FPOPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FPOPEN`"]
pub type FPOPEN_R = crate::R<bool, FPOPEN_A>;
impl FPOPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPOPEN_A {
        match self.bits {
            false => FPOPEN_A::_0,
            true => FPOPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FPOPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FPOPEN_A::_1
    }
}
#[doc = "Write proxy for field `FPOPEN`"]
pub struct FPOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPOPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPOPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "When FPOPEN is clear, the FPHDIS and FPLDIS fields define unprotected address ranges as specified by the corresponding FPHS and FPLS fields."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPOPEN_A::_0)
    }
    #[doc = "When FPOPEN is set, the FPHDIS and FPLDIS fields enable protection for the address range specified by the corresponding FPHS and FPLS fields."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPOPEN_A::_1)
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
    #[doc = "Bits 0:1 - Flash Protection Lower Address Size"]
    #[inline(always)]
    pub fn fpls(&self) -> FPLS_R {
        FPLS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Flash Protection Lower Address Range Disable"]
    #[inline(always)]
    pub fn fpldis(&self) -> FPLDIS_R {
        FPLDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Flash Protection Higher Address Size"]
    #[inline(always)]
    pub fn fphs(&self) -> FPHS_R {
        FPHS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Flash Protection Higher Address Range Disable"]
    #[inline(always)]
    pub fn fphdis(&self) -> FPHDIS_R {
        FPHDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reserved Nonvolatile Bit"]
    #[inline(always)]
    pub fn rnv6(&self) -> RNV6_R {
        RNV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Flash Protection Operation Enable"]
    #[inline(always)]
    pub fn fpopen(&self) -> FPOPEN_R {
        FPOPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash Protection Lower Address Size"]
    #[inline(always)]
    pub fn fpls(&mut self) -> FPLS_W {
        FPLS_W { w: self }
    }
    #[doc = "Bit 2 - Flash Protection Lower Address Range Disable"]
    #[inline(always)]
    pub fn fpldis(&mut self) -> FPLDIS_W {
        FPLDIS_W { w: self }
    }
    #[doc = "Bits 3:4 - Flash Protection Higher Address Size"]
    #[inline(always)]
    pub fn fphs(&mut self) -> FPHS_W {
        FPHS_W { w: self }
    }
    #[doc = "Bit 5 - Flash Protection Higher Address Range Disable"]
    #[inline(always)]
    pub fn fphdis(&mut self) -> FPHDIS_W {
        FPHDIS_W { w: self }
    }
    #[doc = "Bit 7 - Flash Protection Operation Enable"]
    #[inline(always)]
    pub fn fpopen(&mut self) -> FPOPEN_W {
        FPOPEN_W { w: self }
    }
}

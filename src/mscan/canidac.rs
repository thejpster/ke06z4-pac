#[doc = "Reader of register CANIDAC"]
pub type R = crate::R<u8, super::CANIDAC>;
#[doc = "Writer for register CANIDAC"]
pub type W = crate::W<u8, super::CANIDAC>;
#[doc = "Register CANIDAC `reset()`'s with value 0"]
impl crate::ResetValue for super::CANIDAC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Identifier Acceptance Hit Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDHIT_A {
    #[doc = "0: Filter 0 hit."]
    _000 = 0,
    #[doc = "1: Filter 1 hit."]
    _001 = 1,
    #[doc = "2: Filter 2 hit."]
    _010 = 2,
    #[doc = "3: Filter 3 hit."]
    _011 = 3,
    #[doc = "4: Filter 4 hit."]
    _100 = 4,
    #[doc = "5: Filter 5 hit."]
    _101 = 5,
    #[doc = "6: Filter 6 hit."]
    _110 = 6,
    #[doc = "7: Filter 7 hit."]
    _111 = 7,
}
impl From<IDHIT_A> for u8 {
    #[inline(always)]
    fn from(variant: IDHIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IDHIT`"]
pub type IDHIT_R = crate::R<u8, IDHIT_A>;
impl IDHIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDHIT_A {
        match self.bits {
            0 => IDHIT_A::_000,
            1 => IDHIT_A::_001,
            2 => IDHIT_A::_010,
            3 => IDHIT_A::_011,
            4 => IDHIT_A::_100,
            5 => IDHIT_A::_101,
            6 => IDHIT_A::_110,
            7 => IDHIT_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == IDHIT_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == IDHIT_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == IDHIT_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == IDHIT_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == IDHIT_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == IDHIT_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == IDHIT_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == IDHIT_A::_111
    }
}
#[doc = "Identifier Acceptance Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDAM_A {
    #[doc = "0: Two 32-bit acceptance filters."]
    _00 = 0,
    #[doc = "1: Four 16-bit acceptance filters."]
    _01 = 1,
    #[doc = "2: Eight 8-bit acceptance filters."]
    _10 = 2,
    #[doc = "3: Filter closed."]
    _11 = 3,
}
impl From<IDAM_A> for u8 {
    #[inline(always)]
    fn from(variant: IDAM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IDAM`"]
pub type IDAM_R = crate::R<u8, IDAM_A>;
impl IDAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDAM_A {
        match self.bits {
            0 => IDAM_A::_00,
            1 => IDAM_A::_01,
            2 => IDAM_A::_10,
            3 => IDAM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IDAM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IDAM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IDAM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IDAM_A::_11
    }
}
#[doc = "Write proxy for field `IDAM`"]
pub struct IDAM_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDAM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Two 32-bit acceptance filters."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IDAM_A::_00)
    }
    #[doc = "Four 16-bit acceptance filters."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IDAM_A::_01)
    }
    #[doc = "Eight 8-bit acceptance filters."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IDAM_A::_10)
    }
    #[doc = "Filter closed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IDAM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Identifier Acceptance Hit Indicator"]
    #[inline(always)]
    pub fn idhit(&self) -> IDHIT_R {
        IDHIT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - Identifier Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&self) -> IDAM_R {
        IDAM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Identifier Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&mut self) -> IDAM_W {
        IDAM_W { w: self }
    }
}

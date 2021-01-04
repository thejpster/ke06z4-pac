#[doc = "Reader of register C2"]
pub type R = crate::R<u8, super::C2>;
#[doc = "Writer for register C2"]
pub type W = crate::W<u8, super::C2>;
#[doc = "Register C2 `reset()`'s with value 0x20"]
impl crate::ResetValue for super::C2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Low Power Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP_A {
    #[doc = "0: FLL is not disabled in bypass mode."]
    _0 = 0,
    #[doc = "1: FLL is disabled in bypass modes unless debug is active."]
    _1 = 1,
}
impl From<LP_A> for bool {
    #[inline(always)]
    fn from(variant: LP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LP`"]
pub type LP_R = crate::R<bool, LP_A>;
impl LP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_A {
        match self.bits {
            false => LP_A::_0,
            true => LP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LP_A::_1
    }
}
#[doc = "Write proxy for field `LP`"]
pub struct LP_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLL is not disabled in bypass mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LP_A::_0)
    }
    #[doc = "FLL is disabled in bypass modes unless debug is active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LP_A::_1)
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
#[doc = "Bus Frequency Divider\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BDIV_A {
    #[doc = "0: Encoding 0-Divides the selected clock by 1."]
    _000 = 0,
    #[doc = "1: Encoding 1-Divides the selected clock by 2 (reset default)."]
    _001 = 1,
    #[doc = "2: Encoding 2-Divides the selected clock by 4."]
    _010 = 2,
    #[doc = "3: Encoding 3-Divides the selected clock by 8."]
    _011 = 3,
    #[doc = "4: Encoding 4-Divides the selected clock by 16."]
    _100 = 4,
    #[doc = "5: Encoding 5-Divides the selected clock by 32."]
    _101 = 5,
    #[doc = "6: Encoding 6-Divides the selected clock by 64."]
    _110 = 6,
    #[doc = "7: Encoding 7-Divides the selected clock by 128."]
    _111 = 7,
}
impl From<BDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: BDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BDIV`"]
pub type BDIV_R = crate::R<u8, BDIV_A>;
impl BDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDIV_A {
        match self.bits {
            0 => BDIV_A::_000,
            1 => BDIV_A::_001,
            2 => BDIV_A::_010,
            3 => BDIV_A::_011,
            4 => BDIV_A::_100,
            5 => BDIV_A::_101,
            6 => BDIV_A::_110,
            7 => BDIV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == BDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == BDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == BDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == BDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == BDIV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == BDIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == BDIV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == BDIV_A::_111
    }
}
#[doc = "Write proxy for field `BDIV`"]
pub struct BDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Encoding 0-Divides the selected clock by 1."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(BDIV_A::_000)
    }
    #[doc = "Encoding 1-Divides the selected clock by 2 (reset default)."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(BDIV_A::_001)
    }
    #[doc = "Encoding 2-Divides the selected clock by 4."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(BDIV_A::_010)
    }
    #[doc = "Encoding 3-Divides the selected clock by 8."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(BDIV_A::_011)
    }
    #[doc = "Encoding 4-Divides the selected clock by 16."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(BDIV_A::_100)
    }
    #[doc = "Encoding 5-Divides the selected clock by 32."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(BDIV_A::_101)
    }
    #[doc = "Encoding 6-Divides the selected clock by 64."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(BDIV_A::_110)
    }
    #[doc = "Encoding 7-Divides the selected clock by 128."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(BDIV_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u8) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&self) -> LP_R {
        LP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Bus Frequency Divider"]
    #[inline(always)]
    pub fn bdiv(&self) -> BDIV_R {
        BDIV_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&mut self) -> LP_W {
        LP_W { w: self }
    }
    #[doc = "Bits 5:7 - Bus Frequency Divider"]
    #[inline(always)]
    pub fn bdiv(&mut self) -> BDIV_W {
        BDIV_W { w: self }
    }
}

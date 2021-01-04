#[doc = "Reader of register CANBTR1"]
pub type R = crate::R<u8, super::CANBTR1>;
#[doc = "Writer for register CANBTR1"]
pub type W = crate::W<u8, super::CANBTR1>;
#[doc = "Register CANBTR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CANBTR1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Time Segment 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSEG1_A {
    #[doc = "0: 1 Tq clock cycle (not valid)"]
    _0000 = 0,
    #[doc = "1: 2 Tq clock cycles (not valid)"]
    _0001 = 1,
    #[doc = "2: 3 Tq clock cycles (not valid)"]
    _0010 = 2,
    #[doc = "3: 4 Tq clock cycles"]
    _0011 = 3,
    #[doc = "14: 15 Tq clock cycles"]
    _1110 = 14,
    #[doc = "15: 16 Tq clock cycles"]
    _1111 = 15,
}
impl From<TSEG1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEG1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSEG1`"]
pub type TSEG1_R = crate::R<u8, TSEG1_A>;
impl TSEG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSEG1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSEG1_A::_0000),
            1 => Val(TSEG1_A::_0001),
            2 => Val(TSEG1_A::_0010),
            3 => Val(TSEG1_A::_0011),
            14 => Val(TSEG1_A::_1110),
            15 => Val(TSEG1_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TSEG1_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TSEG1_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == TSEG1_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == TSEG1_A::_0011
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == TSEG1_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == TSEG1_A::_1111
    }
}
#[doc = "Write proxy for field `TSEG1`"]
pub struct TSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEG1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 Tq clock cycle (not valid)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TSEG1_A::_0000)
    }
    #[doc = "2 Tq clock cycles (not valid)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TSEG1_A::_0001)
    }
    #[doc = "3 Tq clock cycles (not valid)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TSEG1_A::_0010)
    }
    #[doc = "4 Tq clock cycles"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TSEG1_A::_0011)
    }
    #[doc = "15 Tq clock cycles"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TSEG1_A::_1110)
    }
    #[doc = "16 Tq clock cycles"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TSEG1_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Time Segment 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSEG2_A {
    #[doc = "0: 1 Tq clock cycle (not valid)"]
    _000 = 0,
    #[doc = "1: 2 Tq clock cycles"]
    _001 = 1,
    #[doc = "2: 3 Tq clock cycles"]
    _010 = 2,
    #[doc = "3: 4 Tq clock cycles"]
    _011 = 3,
    #[doc = "4: 5 Tq clock cycles"]
    _100 = 4,
    #[doc = "5: 6 Tq clock cycles"]
    _101 = 5,
    #[doc = "6: 7 Tq clock cycles"]
    _110 = 6,
    #[doc = "7: 8 Tq clock cycles"]
    _111 = 7,
}
impl From<TSEG2_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEG2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSEG2`"]
pub type TSEG2_R = crate::R<u8, TSEG2_A>;
impl TSEG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEG2_A {
        match self.bits {
            0 => TSEG2_A::_000,
            1 => TSEG2_A::_001,
            2 => TSEG2_A::_010,
            3 => TSEG2_A::_011,
            4 => TSEG2_A::_100,
            5 => TSEG2_A::_101,
            6 => TSEG2_A::_110,
            7 => TSEG2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TSEG2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TSEG2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TSEG2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TSEG2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TSEG2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == TSEG2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == TSEG2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == TSEG2_A::_111
    }
}
#[doc = "Write proxy for field `TSEG2`"]
pub struct TSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEG2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 Tq clock cycle (not valid)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(TSEG2_A::_000)
    }
    #[doc = "2 Tq clock cycles"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(TSEG2_A::_001)
    }
    #[doc = "3 Tq clock cycles"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(TSEG2_A::_010)
    }
    #[doc = "4 Tq clock cycles"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(TSEG2_A::_011)
    }
    #[doc = "5 Tq clock cycles"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(TSEG2_A::_100)
    }
    #[doc = "6 Tq clock cycles"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(TSEG2_A::_101)
    }
    #[doc = "7 Tq clock cycles"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(TSEG2_A::_110)
    }
    #[doc = "8 Tq clock cycles"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(TSEG2_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u8) & 0x07) << 4);
        self.w
    }
}
#[doc = "Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMP_A {
    #[doc = "0: One sample per bit."]
    _0 = 0,
    #[doc = "1: Three samples per bit. In this case, PHASE_SEG1 must be at least 2 time quanta (Tq)."]
    _1 = 1,
}
impl From<SAMP_A> for bool {
    #[inline(always)]
    fn from(variant: SAMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAMP`"]
pub type SAMP_R = crate::R<bool, SAMP_A>;
impl SAMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMP_A {
        match self.bits {
            false => SAMP_A::_0,
            true => SAMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAMP_A::_1
    }
}
#[doc = "Write proxy for field `SAMP`"]
pub struct SAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One sample per bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAMP_A::_0)
    }
    #[doc = "Three samples per bit. In this case, PHASE_SEG1 must be at least 2 time quanta (Tq)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAMP_A::_1)
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
    #[doc = "Bits 0:3 - Time Segment 1"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Time Segment 2"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Sampling"]
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time Segment 1"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> TSEG1_W {
        TSEG1_W { w: self }
    }
    #[doc = "Bits 4:6 - Time Segment 2"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W {
        TSEG2_W { w: self }
    }
    #[doc = "Bit 7 - Sampling"]
    #[inline(always)]
    pub fn samp(&mut self) -> SAMP_W {
        SAMP_W { w: self }
    }
}

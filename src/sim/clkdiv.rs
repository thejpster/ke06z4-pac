#[doc = "Reader of register CLKDIV"]
pub type R = crate::R<u32, super::CLKDIV>;
#[doc = "Writer for register CLKDIV"]
pub type W = crate::W<u32, super::CLKDIV>;
#[doc = "Register CLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock 3 output divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDIV3_A {
    #[doc = "0: Same as ICSOUTCLK."]
    _0 = 0,
    #[doc = "1: ICSOUTCLK divides by 2."]
    _1 = 1,
}
impl From<OUTDIV3_A> for bool {
    #[inline(always)]
    fn from(variant: OUTDIV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTDIV3`"]
pub type OUTDIV3_R = crate::R<bool, OUTDIV3_A>;
impl OUTDIV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTDIV3_A {
        match self.bits {
            false => OUTDIV3_A::_0,
            true => OUTDIV3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUTDIV3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUTDIV3_A::_1
    }
}
#[doc = "Write proxy for field `OUTDIV3`"]
pub struct OUTDIV3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDIV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTDIV3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Same as ICSOUTCLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_0)
    }
    #[doc = "ICSOUTCLK divides by 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Clock 2 output divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDIV2_A {
    #[doc = "0: Not divided from divider1."]
    _0 = 0,
    #[doc = "1: Divide by 2 from divider1."]
    _1 = 1,
}
impl From<OUTDIV2_A> for bool {
    #[inline(always)]
    fn from(variant: OUTDIV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTDIV2`"]
pub type OUTDIV2_R = crate::R<bool, OUTDIV2_A>;
impl OUTDIV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTDIV2_A {
        match self.bits {
            false => OUTDIV2_A::_0,
            true => OUTDIV2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OUTDIV2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OUTDIV2_A::_1
    }
}
#[doc = "Write proxy for field `OUTDIV2`"]
pub struct OUTDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTDIV2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not divided from divider1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_0)
    }
    #[doc = "Divide by 2 from divider1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Clock 1 output divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTDIV1_A {
    #[doc = "0: Same as ICSOUTCLK."]
    _00 = 0,
    #[doc = "1: ICSOUTCLK divides by 2."]
    _01 = 1,
    #[doc = "2: ICSOUTCLK divides by 3."]
    _10 = 2,
    #[doc = "3: ICSOUTCLK divides by 4."]
    _11 = 3,
}
impl From<OUTDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTDIV1`"]
pub type OUTDIV1_R = crate::R<u8, OUTDIV1_A>;
impl OUTDIV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTDIV1_A {
        match self.bits {
            0 => OUTDIV1_A::_00,
            1 => OUTDIV1_A::_01,
            2 => OUTDIV1_A::_10,
            3 => OUTDIV1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OUTDIV1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OUTDIV1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OUTDIV1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OUTDIV1_A::_11
    }
}
#[doc = "Write proxy for field `OUTDIV1`"]
pub struct OUTDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTDIV1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Same as ICSOUTCLK."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_00)
    }
    #[doc = "ICSOUTCLK divides by 2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_01)
    }
    #[doc = "ICSOUTCLK divides by 3."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_10)
    }
    #[doc = "ICSOUTCLK divides by 4."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - Clock 3 output divider value"]
    #[inline(always)]
    pub fn outdiv3(&self) -> OUTDIV3_R {
        OUTDIV3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Clock 2 output divider value"]
    #[inline(always)]
    pub fn outdiv2(&self) -> OUTDIV2_R {
        OUTDIV2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Clock 1 output divider value"]
    #[inline(always)]
    pub fn outdiv1(&self) -> OUTDIV1_R {
        OUTDIV1_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 20 - Clock 3 output divider value"]
    #[inline(always)]
    pub fn outdiv3(&mut self) -> OUTDIV3_W {
        OUTDIV3_W { w: self }
    }
    #[doc = "Bit 24 - Clock 2 output divider value"]
    #[inline(always)]
    pub fn outdiv2(&mut self) -> OUTDIV2_W {
        OUTDIV2_W { w: self }
    }
    #[doc = "Bits 28:29 - Clock 1 output divider value"]
    #[inline(always)]
    pub fn outdiv1(&mut self) -> OUTDIV1_W {
        OUTDIV1_W { w: self }
    }
}

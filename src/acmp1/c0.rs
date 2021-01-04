#[doc = "Reader of register C0"]
pub type R = crate::R<u8, super::C0>;
#[doc = "Writer for register C0"]
pub type W = crate::W<u8, super::C0>;
#[doc = "Register C0 `reset()`'s with value 0"]
impl crate::ResetValue for super::C0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ACMP Negative Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACNSEL_A {
    #[doc = "0: External reference 0"]
    _00 = 0,
    #[doc = "1: External reference 1"]
    _01 = 1,
    #[doc = "2: External reference 2"]
    _10 = 2,
    #[doc = "3: DAC output"]
    _11 = 3,
}
impl From<ACNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ACNSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACNSEL`"]
pub type ACNSEL_R = crate::R<u8, ACNSEL_A>;
impl ACNSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACNSEL_A {
        match self.bits {
            0 => ACNSEL_A::_00,
            1 => ACNSEL_A::_01,
            2 => ACNSEL_A::_10,
            3 => ACNSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ACNSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ACNSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ACNSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ACNSEL_A::_11
    }
}
#[doc = "Write proxy for field `ACNSEL`"]
pub struct ACNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACNSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External reference 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ACNSEL_A::_00)
    }
    #[doc = "External reference 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ACNSEL_A::_01)
    }
    #[doc = "External reference 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ACNSEL_A::_10)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ACNSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "ACMP Positive Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACPSEL_A {
    #[doc = "0: External reference 0"]
    _00 = 0,
    #[doc = "1: External reference 1"]
    _01 = 1,
    #[doc = "2: External reference 2"]
    _10 = 2,
    #[doc = "3: DAC output"]
    _11 = 3,
}
impl From<ACPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ACPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACPSEL`"]
pub type ACPSEL_R = crate::R<u8, ACPSEL_A>;
impl ACPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACPSEL_A {
        match self.bits {
            0 => ACPSEL_A::_00,
            1 => ACPSEL_A::_01,
            2 => ACPSEL_A::_10,
            3 => ACPSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ACPSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ACPSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ACPSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ACPSEL_A::_11
    }
}
#[doc = "Write proxy for field `ACPSEL`"]
pub struct ACPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACPSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External reference 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ACPSEL_A::_00)
    }
    #[doc = "External reference 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ACPSEL_A::_01)
    }
    #[doc = "External reference 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ACPSEL_A::_10)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ACPSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ACMP Negative Input Select"]
    #[inline(always)]
    pub fn acnsel(&self) -> ACNSEL_R {
        ACNSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - ACMP Positive Input Select"]
    #[inline(always)]
    pub fn acpsel(&self) -> ACPSEL_R {
        ACPSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ACMP Negative Input Select"]
    #[inline(always)]
    pub fn acnsel(&mut self) -> ACNSEL_W {
        ACNSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - ACMP Positive Input Select"]
    #[inline(always)]
    pub fn acpsel(&mut self) -> ACPSEL_W {
        ACPSEL_W { w: self }
    }
}

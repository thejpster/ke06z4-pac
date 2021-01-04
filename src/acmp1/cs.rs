#[doc = "Reader of register CS"]
pub type R = crate::R<u8, super::CS>;
#[doc = "Writer for register CS"]
pub type W = crate::W<u8, super::CS>;
#[doc = "Register CS `reset()`'s with value 0"]
impl crate::ResetValue for super::CS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ACMP MOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACMOD_A {
    #[doc = "0: ACMP interrupt on output falling edge."]
    _00 = 0,
    #[doc = "1: ACMP interrupt on output rising edge."]
    _01 = 1,
    #[doc = "2: ACMP interrupt on output falling edge."]
    _10 = 2,
    #[doc = "3: ACMP interrupt on output falling or rising edge."]
    _11 = 3,
}
impl From<ACMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACMOD`"]
pub type ACMOD_R = crate::R<u8, ACMOD_A>;
impl ACMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMOD_A {
        match self.bits {
            0 => ACMOD_A::_00,
            1 => ACMOD_A::_01,
            2 => ACMOD_A::_10,
            3 => ACMOD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ACMOD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ACMOD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ACMOD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ACMOD_A::_11
    }
}
#[doc = "Write proxy for field `ACMOD`"]
pub struct ACMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ACMP interrupt on output falling edge."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ACMOD_A::_00)
    }
    #[doc = "ACMP interrupt on output rising edge."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ACMOD_A::_01)
    }
    #[doc = "ACMP interrupt on output falling edge."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ACMOD_A::_10)
    }
    #[doc = "ACMP interrupt on output falling or rising edge."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ACMOD_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "ACMP Output Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACOPE_A {
    #[doc = "0: ACMP output cannot be placed onto external pin."]
    _0 = 0,
    #[doc = "1: ACMP output can be placed onto external pin."]
    _1 = 1,
}
impl From<ACOPE_A> for bool {
    #[inline(always)]
    fn from(variant: ACOPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACOPE`"]
pub type ACOPE_R = crate::R<bool, ACOPE_A>;
impl ACOPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACOPE_A {
        match self.bits {
            false => ACOPE_A::_0,
            true => ACOPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACOPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACOPE_A::_1
    }
}
#[doc = "Write proxy for field `ACOPE`"]
pub struct ACOPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACOPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACMP output cannot be placed onto external pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACOPE_A::_0)
    }
    #[doc = "ACMP output can be placed onto external pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACOPE_A::_1)
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
#[doc = "Reader of field `ACO`"]
pub type ACO_R = crate::R<bool, bool>;
#[doc = "ACMP Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACIE_A {
    #[doc = "0: Disable the ACMP Interrupt."]
    _0 = 0,
    #[doc = "1: Enable the ACMP Interrupt."]
    _1 = 1,
}
impl From<ACIE_A> for bool {
    #[inline(always)]
    fn from(variant: ACIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACIE`"]
pub type ACIE_R = crate::R<bool, ACIE_A>;
impl ACIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACIE_A {
        match self.bits {
            false => ACIE_A::_0,
            true => ACIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACIE_A::_1
    }
}
#[doc = "Write proxy for field `ACIE`"]
pub struct ACIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the ACMP Interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACIE_A::_0)
    }
    #[doc = "Enable the ACMP Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACIE_A::_1)
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
#[doc = "Reader of field `ACF`"]
pub type ACF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACF`"]
pub struct ACF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACF_W<'a> {
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
#[doc = "Analog Comparator Hysterisis Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYST_A {
    #[doc = "0: 20 mV."]
    _0 = 0,
    #[doc = "1: 30 mV."]
    _1 = 1,
}
impl From<HYST_A> for bool {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HYST`"]
pub type HYST_R = crate::R<bool, HYST_A>;
impl HYST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            false => HYST_A::_0,
            true => HYST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HYST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HYST_A::_1
    }
}
#[doc = "Write proxy for field `HYST`"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "20 mV."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HYST_A::_0)
    }
    #[doc = "30 mV."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HYST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Analog Comparator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACE_A {
    #[doc = "0: The ACMP is disabled."]
    _0 = 0,
    #[doc = "1: The ACMP is enabled."]
    _1 = 1,
}
impl From<ACE_A> for bool {
    #[inline(always)]
    fn from(variant: ACE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACE`"]
pub type ACE_R = crate::R<bool, ACE_A>;
impl ACE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACE_A {
        match self.bits {
            false => ACE_A::_0,
            true => ACE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACE_A::_1
    }
}
#[doc = "Write proxy for field `ACE`"]
pub struct ACE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The ACMP is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACE_A::_0)
    }
    #[doc = "The ACMP is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACE_A::_1)
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
    #[doc = "Bits 0:1 - ACMP MOD"]
    #[inline(always)]
    pub fn acmod(&self) -> ACMOD_R {
        ACMOD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - ACMP Output Pin Enable"]
    #[inline(always)]
    pub fn acope(&self) -> ACOPE_R {
        ACOPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ACMP Output"]
    #[inline(always)]
    pub fn aco(&self) -> ACO_R {
        ACO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ACMP Interrupt Enable"]
    #[inline(always)]
    pub fn acie(&self) -> ACIE_R {
        ACIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACMP Interrupt Flag Bit"]
    #[inline(always)]
    pub fn acf(&self) -> ACF_R {
        ACF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator Hysterisis Selection"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn ace(&self) -> ACE_R {
        ACE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ACMP MOD"]
    #[inline(always)]
    pub fn acmod(&mut self) -> ACMOD_W {
        ACMOD_W { w: self }
    }
    #[doc = "Bit 2 - ACMP Output Pin Enable"]
    #[inline(always)]
    pub fn acope(&mut self) -> ACOPE_W {
        ACOPE_W { w: self }
    }
    #[doc = "Bit 4 - ACMP Interrupt Enable"]
    #[inline(always)]
    pub fn acie(&mut self) -> ACIE_W {
        ACIE_W { w: self }
    }
    #[doc = "Bit 5 - ACMP Interrupt Flag Bit"]
    #[inline(always)]
    pub fn acf(&mut self) -> ACF_W {
        ACF_W { w: self }
    }
    #[doc = "Bit 6 - Analog Comparator Hysterisis Selection"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Bit 7 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn ace(&mut self) -> ACE_W {
        ACE_W { w: self }
    }
}

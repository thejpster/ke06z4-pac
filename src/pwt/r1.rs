#[doc = "Reader of register R1"]
pub type R = crate::R<u32, super::R1>;
#[doc = "Writer for register R1"]
pub type W = crate::W<u32, super::R1>;
#[doc = "Register R1 `reset()`'s with value 0"]
impl crate::ResetValue for super::R1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PWT Counter Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWTOV_A {
    #[doc = "0: PWT counter no overflow."]
    _0 = 0,
    #[doc = "1: PWT counter runs from 0xFFFF to 0x0000."]
    _1 = 1,
}
impl From<PWTOV_A> for bool {
    #[inline(always)]
    fn from(variant: PWTOV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWTOV`"]
pub type PWTOV_R = crate::R<bool, PWTOV_A>;
impl PWTOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWTOV_A {
        match self.bits {
            false => PWTOV_A::_0,
            true => PWTOV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWTOV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWTOV_A::_1
    }
}
#[doc = "Write proxy for field `PWTOV`"]
pub struct PWTOV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWTOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWTOV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWT counter no overflow."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWTOV_A::_0)
    }
    #[doc = "PWT counter runs from 0xFFFF to 0x0000."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWTOV_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "PWT Pulse Width Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWTRDY_A {
    #[doc = "0: PWT pulse width register(s) is not up-to-date."]
    _0 = 0,
    #[doc = "1: PWT pulse width register(s) has been updated."]
    _1 = 1,
}
impl From<PWTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PWTRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWTRDY`"]
pub type PWTRDY_R = crate::R<bool, PWTRDY_A>;
impl PWTRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWTRDY_A {
        match self.bits {
            false => PWTRDY_A::_0,
            true => PWTRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWTRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWTRDY_A::_1
    }
}
#[doc = "Write proxy for field `PWTRDY`"]
pub struct PWTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> PWTRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWTRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWT pulse width register(s) is not up-to-date."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWTRDY_A::_0)
    }
    #[doc = "PWT pulse width register(s) has been updated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWTRDY_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "PWT Soft Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWTSR_AW {
    #[doc = "0: No action taken."]
    _0 = 0,
    #[doc = "1: Writing 1 to this field will perform soft reset to PWT."]
    _1 = 1,
}
impl From<PWTSR_AW> for bool {
    #[inline(always)]
    fn from(variant: PWTSR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PWTSR`"]
pub struct PWTSR_W<'a> {
    w: &'a mut W,
}
impl<'a> PWTSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWTSR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action taken."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWTSR_AW::_0)
    }
    #[doc = "Writing 1 to this field will perform soft reset to PWT."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWTSR_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "PWT Counter Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POVIE_A {
    #[doc = "0: Disable PWT to generate interrupt when PWTOV is set."]
    _0 = 0,
    #[doc = "1: Enable PWT to generate interrupt when PWTOV is set."]
    _1 = 1,
}
impl From<POVIE_A> for bool {
    #[inline(always)]
    fn from(variant: POVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POVIE`"]
pub type POVIE_R = crate::R<bool, POVIE_A>;
impl POVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POVIE_A {
        match self.bits {
            false => POVIE_A::_0,
            true => POVIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POVIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POVIE_A::_1
    }
}
#[doc = "Write proxy for field `POVIE`"]
pub struct POVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> POVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable PWT to generate interrupt when PWTOV is set."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POVIE_A::_0)
    }
    #[doc = "Enable PWT to generate interrupt when PWTOV is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POVIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "PWT Pulse Width Data Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRDYIE_A {
    #[doc = "0: Disable PWT to generate interrupt when PWTRDY is set."]
    _0 = 0,
    #[doc = "1: Enable PWT to generate interrupt when PWTRDY is set."]
    _1 = 1,
}
impl From<PRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRDYIE`"]
pub type PRDYIE_R = crate::R<bool, PRDYIE_A>;
impl PRDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRDYIE_A {
        match self.bits {
            false => PRDYIE_A::_0,
            true => PRDYIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRDYIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRDYIE_A::_1
    }
}
#[doc = "Write proxy for field `PRDYIE`"]
pub struct PRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable PWT to generate interrupt when PWTRDY is set."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDYIE_A::_0)
    }
    #[doc = "Enable PWT to generate interrupt when PWTRDY is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDYIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "PWT Module Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWTIE_A {
    #[doc = "0: Disables the PWT to generate interrupt."]
    _0 = 0,
    #[doc = "1: Enables the PWT to generate interrupt."]
    _1 = 1,
}
impl From<PWTIE_A> for bool {
    #[inline(always)]
    fn from(variant: PWTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWTIE`"]
pub type PWTIE_R = crate::R<bool, PWTIE_A>;
impl PWTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWTIE_A {
        match self.bits {
            false => PWTIE_A::_0,
            true => PWTIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWTIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWTIE_A::_1
    }
}
#[doc = "Write proxy for field `PWTIE`"]
pub struct PWTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the PWT to generate interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWTIE_A::_0)
    }
    #[doc = "Enables the PWT to generate interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWTIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "PWT Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWTEN_A {
    #[doc = "0: The PWT is disabled."]
    _0 = 0,
    #[doc = "1: The PWT is enabled."]
    _1 = 1,
}
impl From<PWTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWTEN`"]
pub type PWTEN_R = crate::R<bool, PWTEN_A>;
impl PWTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWTEN_A {
        match self.bits {
            false => PWTEN_A::_0,
            true => PWTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWTEN_A::_1
    }
}
#[doc = "Write proxy for field `PWTEN`"]
pub struct PWTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWT is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWTEN_A::_0)
    }
    #[doc = "The PWT is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "PWT Clock Prescaler (CLKPRE) Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRE_A {
    #[doc = "0: Clock divided by 1."]
    _000 = 0,
    #[doc = "1: Clock divided by 2."]
    _001 = 1,
    #[doc = "2: Clock divided by 4."]
    _010 = 2,
    #[doc = "3: Clock divided by 8."]
    _011 = 3,
    #[doc = "4: Clock divided by 16."]
    _100 = 4,
    #[doc = "5: Clock divided by 32."]
    _101 = 5,
    #[doc = "6: Clock divided by 64."]
    _110 = 6,
    #[doc = "7: Clock divided by 128."]
    _111 = 7,
}
impl From<PRE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRE`"]
pub type PRE_R = crate::R<u8, PRE_A>;
impl PRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRE_A {
        match self.bits {
            0 => PRE_A::_000,
            1 => PRE_A::_001,
            2 => PRE_A::_010,
            3 => PRE_A::_011,
            4 => PRE_A::_100,
            5 => PRE_A::_101,
            6 => PRE_A::_110,
            7 => PRE_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PRE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PRE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PRE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PRE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PRE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PRE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PRE_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PRE_A::_111
    }
}
#[doc = "Write proxy for field `PRE`"]
pub struct PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock divided by 1."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PRE_A::_000)
    }
    #[doc = "Clock divided by 2."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PRE_A::_001)
    }
    #[doc = "Clock divided by 4."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PRE_A::_010)
    }
    #[doc = "Clock divided by 8."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PRE_A::_011)
    }
    #[doc = "Clock divided by 16."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PRE_A::_100)
    }
    #[doc = "Clock divided by 32."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PRE_A::_101)
    }
    #[doc = "Clock divided by 64."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PRE_A::_110)
    }
    #[doc = "Clock divided by 128."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PRE_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "PWT Input Edge Sensitivity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGE_A {
    #[doc = "0: The first falling-edge starts the pulse width measurement, and on all the subsequent falling edges, the pulse width is captured."]
    _00 = 0,
    #[doc = "1: The first rising edge starts the pulse width measurement, and on all the subsequent rising and falling edges, the pulse width is captured."]
    _01 = 1,
    #[doc = "2: The first falling edge starts the pulse width measurement, and on all the subsequent rising and falling edges, the pulse width is captured."]
    _10 = 2,
    #[doc = "3: The first-rising edge starts the pulse width measurement, and on all the subsequent rising edges, the pulse width is captured."]
    _11 = 3,
}
impl From<EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EDGE`"]
pub type EDGE_R = crate::R<u8, EDGE_A>;
impl EDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_A {
        match self.bits {
            0 => EDGE_A::_00,
            1 => EDGE_A::_01,
            2 => EDGE_A::_10,
            3 => EDGE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EDGE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EDGE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == EDGE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == EDGE_A::_11
    }
}
#[doc = "Write proxy for field `EDGE`"]
pub struct EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The first falling-edge starts the pulse width measurement, and on all the subsequent falling edges, the pulse width is captured."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(EDGE_A::_00)
    }
    #[doc = "The first rising edge starts the pulse width measurement, and on all the subsequent rising and falling edges, the pulse width is captured."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(EDGE_A::_01)
    }
    #[doc = "The first falling edge starts the pulse width measurement, and on all the subsequent rising and falling edges, the pulse width is captured."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(EDGE_A::_10)
    }
    #[doc = "The first-rising edge starts the pulse width measurement, and on all the subsequent rising edges, the pulse width is captured."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(EDGE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "PWT Pulse Inputs Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINSEL_A {
    #[doc = "0: PWTIN\\[0\\]
is enabled."]
    _00 = 0,
    #[doc = "1: PWTIN\\[1\\]
is enabled."]
    _01 = 1,
    #[doc = "2: PWTIN\\[2\\]
enabled."]
    _10 = 2,
    #[doc = "3: PWTIN\\[3\\]
enabled."]
    _11 = 3,
}
impl From<PINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PINSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINSEL`"]
pub type PINSEL_R = crate::R<u8, PINSEL_A>;
impl PINSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINSEL_A {
        match self.bits {
            0 => PINSEL_A::_00,
            1 => PINSEL_A::_01,
            2 => PINSEL_A::_10,
            3 => PINSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PINSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PINSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PINSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PINSEL_A::_11
    }
}
#[doc = "Write proxy for field `PINSEL`"]
pub struct PINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PWTIN\\[0\\]
is enabled."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PINSEL_A::_00)
    }
    #[doc = "PWTIN\\[1\\]
is enabled."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PINSEL_A::_01)
    }
    #[doc = "PWTIN\\[2\\]
enabled."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PINSEL_A::_10)
    }
    #[doc = "PWTIN\\[3\\]
enabled."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PINSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "PWT Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLKS_A {
    #[doc = "0: TIMER_CLK is selected as the clock source of PWT counter."]
    _0 = 0,
    #[doc = "1: Alternative clock is selected as the clock source of PWT counter."]
    _1 = 1,
}
impl From<PCLKS_A> for bool {
    #[inline(always)]
    fn from(variant: PCLKS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PCLKS`"]
pub type PCLKS_R = crate::R<bool, PCLKS_A>;
impl PCLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLKS_A {
        match self.bits {
            false => PCLKS_A::_0,
            true => PCLKS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCLKS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCLKS_A::_1
    }
}
#[doc = "Write proxy for field `PCLKS`"]
pub struct PCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLKS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TIMER_CLK is selected as the clock source of PWT counter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCLKS_A::_0)
    }
    #[doc = "Alternative clock is selected as the clock source of PWT counter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCLKS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `PPW`"]
pub type PPW_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - PWT Counter Overflow"]
    #[inline(always)]
    pub fn pwtov(&self) -> PWTOV_R {
        PWTOV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWT Pulse Width Valid"]
    #[inline(always)]
    pub fn pwtrdy(&self) -> PWTRDY_R {
        PWTRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWT Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn povie(&self) -> POVIE_R {
        POVIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWT Pulse Width Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn prdyie(&self) -> PRDYIE_R {
        PRDYIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PWT Module Interrupt Enable"]
    #[inline(always)]
    pub fn pwtie(&self) -> PWTIE_R {
        PWTIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PWT Module Enable"]
    #[inline(always)]
    pub fn pwten(&self) -> PWTEN_R {
        PWTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - PWT Clock Prescaler (CLKPRE) Setting"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:12 - PWT Input Edge Sensitivity"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - PWT Pulse Inputs Selection"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - PWT Clock Source Selection"]
    #[inline(always)]
    pub fn pclks(&self) -> PCLKS_R {
        PCLKS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Positive Pulse Width"]
    #[inline(always)]
    pub fn ppw(&self) -> PPW_R {
        PPW_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PWT Counter Overflow"]
    #[inline(always)]
    pub fn pwtov(&mut self) -> PWTOV_W {
        PWTOV_W { w: self }
    }
    #[doc = "Bit 1 - PWT Pulse Width Valid"]
    #[inline(always)]
    pub fn pwtrdy(&mut self) -> PWTRDY_W {
        PWTRDY_W { w: self }
    }
    #[doc = "Bit 3 - PWT Soft Reset"]
    #[inline(always)]
    pub fn pwtsr(&mut self) -> PWTSR_W {
        PWTSR_W { w: self }
    }
    #[doc = "Bit 4 - PWT Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn povie(&mut self) -> POVIE_W {
        POVIE_W { w: self }
    }
    #[doc = "Bit 5 - PWT Pulse Width Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn prdyie(&mut self) -> PRDYIE_W {
        PRDYIE_W { w: self }
    }
    #[doc = "Bit 6 - PWT Module Interrupt Enable"]
    #[inline(always)]
    pub fn pwtie(&mut self) -> PWTIE_W {
        PWTIE_W { w: self }
    }
    #[doc = "Bit 7 - PWT Module Enable"]
    #[inline(always)]
    pub fn pwten(&mut self) -> PWTEN_W {
        PWTEN_W { w: self }
    }
    #[doc = "Bits 8:10 - PWT Clock Prescaler (CLKPRE) Setting"]
    #[inline(always)]
    pub fn pre(&mut self) -> PRE_W {
        PRE_W { w: self }
    }
    #[doc = "Bits 11:12 - PWT Input Edge Sensitivity"]
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W {
        EDGE_W { w: self }
    }
    #[doc = "Bits 13:14 - PWT Pulse Inputs Selection"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W {
        PINSEL_W { w: self }
    }
    #[doc = "Bit 15 - PWT Clock Source Selection"]
    #[inline(always)]
    pub fn pclks(&mut self) -> PCLKS_W {
        PCLKS_W { w: self }
    }
}

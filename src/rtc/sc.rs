#[doc = "Reader of register SC"]
pub type R = crate::R<u32, super::SC>;
#[doc = "Writer for register SC"]
pub type W = crate::W<u32, super::SC>;
#[doc = "Register SC `reset()`'s with value 0"]
impl crate::ResetValue for super::SC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Real-Time Counter Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCO_A {
    #[doc = "0: Real-time counter output disabled."]
    _0 = 0,
    #[doc = "1: Real-time counter output enabled."]
    _1 = 1,
}
impl From<RTCO_A> for bool {
    #[inline(always)]
    fn from(variant: RTCO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCO`"]
pub type RTCO_R = crate::R<bool, RTCO_A>;
impl RTCO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCO_A {
        match self.bits {
            false => RTCO_A::_0,
            true => RTCO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCO_A::_1
    }
}
#[doc = "Write proxy for field `RTCO`"]
pub struct RTCO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Real-time counter output disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCO_A::_0)
    }
    #[doc = "Real-time counter output enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCO_A::_1)
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
#[doc = "Real-Time Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTIE_A {
    #[doc = "0: Real-time interrupt requests are disabled. Use software polling."]
    _0 = 0,
    #[doc = "1: Real-time interrupt requests are enabled."]
    _1 = 1,
}
impl From<RTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTIE`"]
pub type RTIE_R = crate::R<bool, RTIE_A>;
impl RTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTIE_A {
        match self.bits {
            false => RTIE_A::_0,
            true => RTIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTIE_A::_1
    }
}
#[doc = "Write proxy for field `RTIE`"]
pub struct RTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Real-time interrupt requests are disabled. Use software polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTIE_A::_0)
    }
    #[doc = "Real-time interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTIE_A::_1)
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
#[doc = "Real-Time Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTIF_A {
    #[doc = "0: RTC counter has not reached the value in the RTC modulo register."]
    _0 = 0,
    #[doc = "1: RTC counter has reached the value in the RTC modulo register."]
    _1 = 1,
}
impl From<RTIF_A> for bool {
    #[inline(always)]
    fn from(variant: RTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTIF`"]
pub type RTIF_R = crate::R<bool, RTIF_A>;
impl RTIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTIF_A {
        match self.bits {
            false => RTIF_A::_0,
            true => RTIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTIF_A::_1
    }
}
#[doc = "Write proxy for field `RTIF`"]
pub struct RTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC counter has not reached the value in the RTC modulo register."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTIF_A::_0)
    }
    #[doc = "RTC counter has reached the value in the RTC modulo register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTIF_A::_1)
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
#[doc = "Real-Time Clock Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCPS_A {
    #[doc = "0: Off"]
    _000 = 0,
    #[doc = "1: If RTCLKS = x0, it is 1; if RTCLKS = x1, it is 128."]
    _001 = 1,
    #[doc = "2: If RTCLKS = x0, it is 2; if RTCLKS = x1, it is 256."]
    _010 = 2,
    #[doc = "3: If RTCLKS = x0, it is 4; if RTCLKS = x1, it is 512."]
    _011 = 3,
    #[doc = "4: If RTCLKS = x0, it is 8; if RTCLKS = x1, it is 1024."]
    _100 = 4,
    #[doc = "5: If RTCLKS = x0, it is 16; if RTCLKS = x1, it is 2048."]
    _101 = 5,
    #[doc = "6: If RTCLKS = x0, it is 32; if RTCLKS = x1, it is 100."]
    _110 = 6,
    #[doc = "7: If RTCLKS = x0, it is 64; if RTCLKS = x1, it is 1000."]
    _111 = 7,
}
impl From<RTCPS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCPS`"]
pub type RTCPS_R = crate::R<u8, RTCPS_A>;
impl RTCPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCPS_A {
        match self.bits {
            0 => RTCPS_A::_000,
            1 => RTCPS_A::_001,
            2 => RTCPS_A::_010,
            3 => RTCPS_A::_011,
            4 => RTCPS_A::_100,
            5 => RTCPS_A::_101,
            6 => RTCPS_A::_110,
            7 => RTCPS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RTCPS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RTCPS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RTCPS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RTCPS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RTCPS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RTCPS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RTCPS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RTCPS_A::_111
    }
}
#[doc = "Write proxy for field `RTCPS`"]
pub struct RTCPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCPS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RTCPS_A::_000)
    }
    #[doc = "If RTCLKS = x0, it is 1; if RTCLKS = x1, it is 128."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RTCPS_A::_001)
    }
    #[doc = "If RTCLKS = x0, it is 2; if RTCLKS = x1, it is 256."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(RTCPS_A::_010)
    }
    #[doc = "If RTCLKS = x0, it is 4; if RTCLKS = x1, it is 512."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RTCPS_A::_011)
    }
    #[doc = "If RTCLKS = x0, it is 8; if RTCLKS = x1, it is 1024."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RTCPS_A::_100)
    }
    #[doc = "If RTCLKS = x0, it is 16; if RTCLKS = x1, it is 2048."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RTCPS_A::_101)
    }
    #[doc = "If RTCLKS = x0, it is 32; if RTCLKS = x1, it is 100."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RTCPS_A::_110)
    }
    #[doc = "If RTCLKS = x0, it is 64; if RTCLKS = x1, it is 1000."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RTCPS_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Real-Time Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCLKS_A {
    #[doc = "0: External clock source."]
    _00 = 0,
    #[doc = "1: Real-time clock source is 1 kHz (LPOCLK)."]
    _01 = 1,
    #[doc = "2: Internal reference clock (ICSIRCLK)."]
    _10 = 2,
    #[doc = "3: Bus clock."]
    _11 = 3,
}
impl From<RTCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCLKS`"]
pub type RTCLKS_R = crate::R<u8, RTCLKS_A>;
impl RTCLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCLKS_A {
        match self.bits {
            0 => RTCLKS_A::_00,
            1 => RTCLKS_A::_01,
            2 => RTCLKS_A::_10,
            3 => RTCLKS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RTCLKS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RTCLKS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RTCLKS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RTCLKS_A::_11
    }
}
#[doc = "Write proxy for field `RTCLKS`"]
pub struct RTCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCLKS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External clock source."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RTCLKS_A::_00)
    }
    #[doc = "Real-time clock source is 1 kHz (LPOCLK)."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RTCLKS_A::_01)
    }
    #[doc = "Internal reference clock (ICSIRCLK)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RTCLKS_A::_10)
    }
    #[doc = "Bus clock."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RTCLKS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Real-Time Counter Output"]
    #[inline(always)]
    pub fn rtco(&self) -> RTCO_R {
        RTCO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real-Time Interrupt Enable"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Real-Time Interrupt Flag"]
    #[inline(always)]
    pub fn rtif(&self) -> RTIF_R {
        RTIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Real-Time Clock Prescaler Select"]
    #[inline(always)]
    pub fn rtcps(&self) -> RTCPS_R {
        RTCPS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 14:15 - Real-Time Clock Source Select"]
    #[inline(always)]
    pub fn rtclks(&self) -> RTCLKS_R {
        RTCLKS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Real-Time Counter Output"]
    #[inline(always)]
    pub fn rtco(&mut self) -> RTCO_W {
        RTCO_W { w: self }
    }
    #[doc = "Bit 6 - Real-Time Interrupt Enable"]
    #[inline(always)]
    pub fn rtie(&mut self) -> RTIE_W {
        RTIE_W { w: self }
    }
    #[doc = "Bit 7 - Real-Time Interrupt Flag"]
    #[inline(always)]
    pub fn rtif(&mut self) -> RTIF_W {
        RTIF_W { w: self }
    }
    #[doc = "Bits 8:10 - Real-Time Clock Prescaler Select"]
    #[inline(always)]
    pub fn rtcps(&mut self) -> RTCPS_W {
        RTCPS_W { w: self }
    }
    #[doc = "Bits 14:15 - Real-Time Clock Source Select"]
    #[inline(always)]
    pub fn rtclks(&mut self) -> RTCLKS_W {
        RTCLKS_W { w: self }
    }
}

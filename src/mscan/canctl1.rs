#[doc = "Reader of register CANCTL1"]
pub type R = crate::R<u8, super::CANCTL1>;
#[doc = "Writer for register CANCTL1"]
pub type W = crate::W<u8, super::CANCTL1>;
#[doc = "Register CANCTL1 `reset()`'s with value 0x11"]
impl crate::ResetValue for super::CANCTL1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x11
    }
}
#[doc = "Initialization Mode Acknowledge\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITAK_A {
    #[doc = "0: Running - The MSCAN operates normally."]
    _0 = 0,
    #[doc = "1: Initialization mode active - The MSCAN has entered initialization mode."]
    _1 = 1,
}
impl From<INITAK_A> for bool {
    #[inline(always)]
    fn from(variant: INITAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INITAK`"]
pub type INITAK_R = crate::R<bool, INITAK_A>;
impl INITAK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITAK_A {
        match self.bits {
            false => INITAK_A::_0,
            true => INITAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INITAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INITAK_A::_1
    }
}
#[doc = "Sleep Mode Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPAK_A {
    #[doc = "0: Running - The MSCAN operates normally."]
    _0 = 0,
    #[doc = "1: Sleep mode active - The MSCAN has entered sleep mode."]
    _1 = 1,
}
impl From<SLPAK_A> for bool {
    #[inline(always)]
    fn from(variant: SLPAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLPAK`"]
pub type SLPAK_R = crate::R<bool, SLPAK_A>;
impl SLPAK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPAK_A {
        match self.bits {
            false => SLPAK_A::_0,
            true => SLPAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLPAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLPAK_A::_1
    }
}
#[doc = "WakeUp Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPM_A {
    #[doc = "0: MSCAN wakes on any dominant level on the CAN bus."]
    _0 = 0,
    #[doc = "1: MSCAN wakes only in case of a dominant pulse on the CAN bus that has a length of Twup."]
    _1 = 1,
}
impl From<WUPM_A> for bool {
    #[inline(always)]
    fn from(variant: WUPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WUPM`"]
pub type WUPM_R = crate::R<bool, WUPM_A>;
impl WUPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPM_A {
        match self.bits {
            false => WUPM_A::_0,
            true => WUPM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUPM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUPM_A::_1
    }
}
#[doc = "Write proxy for field `WUPM`"]
pub struct WUPM_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MSCAN wakes on any dominant level on the CAN bus."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUPM_A::_0)
    }
    #[doc = "MSCAN wakes only in case of a dominant pulse on the CAN bus that has a length of Twup."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUPM_A::_1)
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
#[doc = "Bus-Off Recovery Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BORM_A {
    #[doc = "0: Automatic bus-off recovery (see Bosch CAN 2.0A/B protocol specification)."]
    _0 = 0,
    #[doc = "1: Bus-off recovery upon user request."]
    _1 = 1,
}
impl From<BORM_A> for bool {
    #[inline(always)]
    fn from(variant: BORM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BORM`"]
pub type BORM_R = crate::R<bool, BORM_A>;
impl BORM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BORM_A {
        match self.bits {
            false => BORM_A::_0,
            true => BORM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BORM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BORM_A::_1
    }
}
#[doc = "Write proxy for field `BORM`"]
pub struct BORM_W<'a> {
    w: &'a mut W,
}
impl<'a> BORM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BORM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic bus-off recovery (see Bosch CAN 2.0A/B protocol specification)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BORM_A::_0)
    }
    #[doc = "Bus-off recovery upon user request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BORM_A::_1)
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
#[doc = "Listen Only Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LISTEN_A {
    #[doc = "0: Normal operation."]
    _0 = 0,
    #[doc = "1: Listen only mode activated."]
    _1 = 1,
}
impl From<LISTEN_A> for bool {
    #[inline(always)]
    fn from(variant: LISTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LISTEN`"]
pub type LISTEN_R = crate::R<bool, LISTEN_A>;
impl LISTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LISTEN_A {
        match self.bits {
            false => LISTEN_A::_0,
            true => LISTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LISTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LISTEN_A::_1
    }
}
#[doc = "Write proxy for field `LISTEN`"]
pub struct LISTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LISTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LISTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LISTEN_A::_0)
    }
    #[doc = "Listen only mode activated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LISTEN_A::_1)
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
#[doc = "Loopback Self Test Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPB_A {
    #[doc = "0: Loopback self test disabled."]
    _0 = 0,
    #[doc = "1: Loopback self test enabled."]
    _1 = 1,
}
impl From<LOOPB_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOOPB`"]
pub type LOOPB_R = crate::R<bool, LOOPB_A>;
impl LOOPB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPB_A {
        match self.bits {
            false => LOOPB_A::_0,
            true => LOOPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOOPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOOPB_A::_1
    }
}
#[doc = "Write proxy for field `LOOPB`"]
pub struct LOOPB_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Loopback self test disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOOPB_A::_0)
    }
    #[doc = "Loopback self test enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOOPB_A::_1)
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
#[doc = "MSCAN Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRC_A {
    #[doc = "0: MSCAN clock source is the oscillator clock."]
    _0 = 0,
    #[doc = "1: MSCAN clock source is the bus clock."]
    _1 = 1,
}
impl From<CLKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKSRC`"]
pub type CLKSRC_R = crate::R<bool, CLKSRC_A>;
impl CLKSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSRC_A {
        match self.bits {
            false => CLKSRC_A::_0,
            true => CLKSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKSRC_A::_1
    }
}
#[doc = "Write proxy for field `CLKSRC`"]
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MSCAN clock source is the oscillator clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKSRC_A::_0)
    }
    #[doc = "MSCAN clock source is the bus clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKSRC_A::_1)
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
#[doc = "MSCAN Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CANE_A {
    #[doc = "0: MSCAN module is disabled."]
    _0 = 0,
    #[doc = "1: MSCAN module is enabled."]
    _1 = 1,
}
impl From<CANE_A> for bool {
    #[inline(always)]
    fn from(variant: CANE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CANE`"]
pub type CANE_R = crate::R<bool, CANE_A>;
impl CANE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CANE_A {
        match self.bits {
            false => CANE_A::_0,
            true => CANE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CANE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CANE_A::_1
    }
}
#[doc = "Write proxy for field `CANE`"]
pub struct CANE_W<'a> {
    w: &'a mut W,
}
impl<'a> CANE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CANE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MSCAN module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CANE_A::_0)
    }
    #[doc = "MSCAN module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CANE_A::_1)
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
    #[doc = "Bit 0 - Initialization Mode Acknowledge"]
    #[inline(always)]
    pub fn initak(&self) -> INITAK_R {
        INITAK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sleep Mode Acknowledge"]
    #[inline(always)]
    pub fn slpak(&self) -> SLPAK_R {
        SLPAK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WakeUp Mode"]
    #[inline(always)]
    pub fn wupm(&self) -> WUPM_R {
        WUPM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bus-Off Recovery Mode"]
    #[inline(always)]
    pub fn borm(&self) -> BORM_R {
        BORM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Listen Only Mode"]
    #[inline(always)]
    pub fn listen(&self) -> LISTEN_R {
        LISTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Loopback Self Test Mode"]
    #[inline(always)]
    pub fn loopb(&self) -> LOOPB_R {
        LOOPB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MSCAN Clock Source"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MSCAN Enable"]
    #[inline(always)]
    pub fn cane(&self) -> CANE_R {
        CANE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - WakeUp Mode"]
    #[inline(always)]
    pub fn wupm(&mut self) -> WUPM_W {
        WUPM_W { w: self }
    }
    #[doc = "Bit 3 - Bus-Off Recovery Mode"]
    #[inline(always)]
    pub fn borm(&mut self) -> BORM_W {
        BORM_W { w: self }
    }
    #[doc = "Bit 4 - Listen Only Mode"]
    #[inline(always)]
    pub fn listen(&mut self) -> LISTEN_W {
        LISTEN_W { w: self }
    }
    #[doc = "Bit 5 - Loopback Self Test Mode"]
    #[inline(always)]
    pub fn loopb(&mut self) -> LOOPB_W {
        LOOPB_W { w: self }
    }
    #[doc = "Bit 6 - MSCAN Clock Source"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
    #[doc = "Bit 7 - MSCAN Enable"]
    #[inline(always)]
    pub fn cane(&mut self) -> CANE_W {
        CANE_W { w: self }
    }
}

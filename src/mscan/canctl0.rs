#[doc = "Reader of register CANCTL0"]
pub type R = crate::R<u8, super::CANCTL0>;
#[doc = "Writer for register CANCTL0"]
pub type W = crate::W<u8, super::CANCTL0>;
#[doc = "Register CANCTL0 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CANCTL0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Initialization Mode Request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITRQ_A {
    #[doc = "0: Normal operation."]
    _0 = 0,
    #[doc = "1: MSCAN in initialization mode."]
    _1 = 1,
}
impl From<INITRQ_A> for bool {
    #[inline(always)]
    fn from(variant: INITRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INITRQ`"]
pub type INITRQ_R = crate::R<bool, INITRQ_A>;
impl INITRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITRQ_A {
        match self.bits {
            false => INITRQ_A::_0,
            true => INITRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INITRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INITRQ_A::_1
    }
}
#[doc = "Write proxy for field `INITRQ`"]
pub struct INITRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INITRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INITRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITRQ_A::_0)
    }
    #[doc = "MSCAN in initialization mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITRQ_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Sleep Mode Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPRQ_A {
    #[doc = "0: Running - The MSCAN functions normally."]
    _0 = 0,
    #[doc = "1: Sleep mode request - The MSCAN enters sleep mode when CAN bus idle."]
    _1 = 1,
}
impl From<SLPRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SLPRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLPRQ`"]
pub type SLPRQ_R = crate::R<bool, SLPRQ_A>;
impl SLPRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLPRQ_A {
        match self.bits {
            false => SLPRQ_A::_0,
            true => SLPRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLPRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLPRQ_A::_1
    }
}
#[doc = "Write proxy for field `SLPRQ`"]
pub struct SLPRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLPRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Running - The MSCAN functions normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLPRQ_A::_0)
    }
    #[doc = "Sleep mode request - The MSCAN enters sleep mode when CAN bus idle."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLPRQ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "WakeUp Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE_A {
    #[doc = "0: Wakeup disabled - The MSCAN ignores traffic on CAN."]
    _0 = 0,
    #[doc = "1: Wakeup enabled - The MSCAN is able to restart."]
    _1 = 1,
}
impl From<WUPE_A> for bool {
    #[inline(always)]
    fn from(variant: WUPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WUPE`"]
pub type WUPE_R = crate::R<bool, WUPE_A>;
impl WUPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE_A {
        match self.bits {
            false => WUPE_A::_0,
            true => WUPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUPE_A::_1
    }
}
#[doc = "Write proxy for field `WUPE`"]
pub struct WUPE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wakeup disabled - The MSCAN ignores traffic on CAN."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUPE_A::_0)
    }
    #[doc = "Wakeup enabled - The MSCAN is able to restart."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUPE_A::_1)
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
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIME_A {
    #[doc = "0: Disable internal MSCAN timer."]
    _0 = 0,
    #[doc = "1: Enable internal MSCAN timer."]
    _1 = 1,
}
impl From<TIME_A> for bool {
    #[inline(always)]
    fn from(variant: TIME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIME`"]
pub type TIME_R = crate::R<bool, TIME_A>;
impl TIME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIME_A {
        match self.bits {
            false => TIME_A::_0,
            true => TIME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIME_A::_1
    }
}
#[doc = "Write proxy for field `TIME`"]
pub struct TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable internal MSCAN timer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIME_A::_0)
    }
    #[doc = "Enable internal MSCAN timer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIME_A::_1)
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
#[doc = "Synchronized Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCH_A {
    #[doc = "0: MSCAN is not synchronized to the CAN bus."]
    _0 = 0,
    #[doc = "1: MSCAN is synchronized to the CAN bus."]
    _1 = 1,
}
impl From<SYNCH_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCH`"]
pub type SYNCH_R = crate::R<bool, SYNCH_A>;
impl SYNCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCH_A {
        match self.bits {
            false => SYNCH_A::_0,
            true => SYNCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCH_A::_1
    }
}
#[doc = "CAN Stops in Wait Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSWAI_A {
    #[doc = "0: The module is not affected during wait mode."]
    _0 = 0,
    #[doc = "1: The module ceases to be clocked during wait mode."]
    _1 = 1,
}
impl From<CSWAI_A> for bool {
    #[inline(always)]
    fn from(variant: CSWAI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSWAI`"]
pub type CSWAI_R = crate::R<bool, CSWAI_A>;
impl CSWAI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSWAI_A {
        match self.bits {
            false => CSWAI_A::_0,
            true => CSWAI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSWAI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSWAI_A::_1
    }
}
#[doc = "Write proxy for field `CSWAI`"]
pub struct CSWAI_W<'a> {
    w: &'a mut W,
}
impl<'a> CSWAI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSWAI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The module is not affected during wait mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSWAI_A::_0)
    }
    #[doc = "The module ceases to be clocked during wait mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSWAI_A::_1)
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
#[doc = "Receiver Active Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXACT_A {
    #[doc = "0: MSCAN is transmitting or idle."]
    _0 = 0,
    #[doc = "1: MSCAN is receiving a message, including when arbitration is lost."]
    _1 = 1,
}
impl From<RXACT_A> for bool {
    #[inline(always)]
    fn from(variant: RXACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXACT`"]
pub type RXACT_R = crate::R<bool, RXACT_A>;
impl RXACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXACT_A {
        match self.bits {
            false => RXACT_A::_0,
            true => RXACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXACT_A::_1
    }
}
#[doc = "Received Frame Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRM_A {
    #[doc = "0: No valid message was received since last clearing this flag."]
    _0 = 0,
    #[doc = "1: A valid message was received since last clearing of this flag."]
    _1 = 1,
}
impl From<RXFRM_A> for bool {
    #[inline(always)]
    fn from(variant: RXFRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFRM`"]
pub type RXFRM_R = crate::R<bool, RXFRM_A>;
impl RXFRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFRM_A {
        match self.bits {
            false => RXFRM_A::_0,
            true => RXFRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXFRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXFRM_A::_1
    }
}
#[doc = "Write proxy for field `RXFRM`"]
pub struct RXFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No valid message was received since last clearing this flag."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFRM_A::_0)
    }
    #[doc = "A valid message was received since last clearing of this flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFRM_A::_1)
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
    #[doc = "Bit 0 - Initialization Mode Request"]
    #[inline(always)]
    pub fn initrq(&self) -> INITRQ_R {
        INITRQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sleep Mode Request"]
    #[inline(always)]
    pub fn slprq(&self) -> SLPRQ_R {
        SLPRQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WakeUp Enable"]
    #[inline(always)]
    pub fn wupe(&self) -> WUPE_R {
        WUPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer Enable"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Synchronized Status"]
    #[inline(always)]
    pub fn synch(&self) -> SYNCH_R {
        SYNCH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CAN Stops in Wait Mode"]
    #[inline(always)]
    pub fn cswai(&self) -> CSWAI_R {
        CSWAI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receiver Active Status"]
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Received Frame Flag"]
    #[inline(always)]
    pub fn rxfrm(&self) -> RXFRM_R {
        RXFRM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization Mode Request"]
    #[inline(always)]
    pub fn initrq(&mut self) -> INITRQ_W {
        INITRQ_W { w: self }
    }
    #[doc = "Bit 1 - Sleep Mode Request"]
    #[inline(always)]
    pub fn slprq(&mut self) -> SLPRQ_W {
        SLPRQ_W { w: self }
    }
    #[doc = "Bit 2 - WakeUp Enable"]
    #[inline(always)]
    pub fn wupe(&mut self) -> WUPE_W {
        WUPE_W { w: self }
    }
    #[doc = "Bit 3 - Timer Enable"]
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W {
        TIME_W { w: self }
    }
    #[doc = "Bit 5 - CAN Stops in Wait Mode"]
    #[inline(always)]
    pub fn cswai(&mut self) -> CSWAI_W {
        CSWAI_W { w: self }
    }
    #[doc = "Bit 7 - Received Frame Flag"]
    #[inline(always)]
    pub fn rxfrm(&mut self) -> RXFRM_W {
        RXFRM_W { w: self }
    }
}

#[doc = "Reader of register CANRFLG"]
pub type R = crate::R<u8, super::CANRFLG>;
#[doc = "Writer for register CANRFLG"]
pub type W = crate::W<u8, super::CANRFLG>;
#[doc = "Register CANRFLG `reset()`'s with value 0"]
impl crate::ResetValue for super::CANRFLG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive Buffer Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXF_A {
    #[doc = "0: No new message available within the RxFG."]
    _0 = 0,
    #[doc = "1: The receiver FIFO is not empty. A new message is available in the RxFG."]
    _1 = 1,
}
impl From<RXF_A> for bool {
    #[inline(always)]
    fn from(variant: RXF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXF`"]
pub type RXF_R = crate::R<bool, RXF_A>;
impl RXF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXF_A {
        match self.bits {
            false => RXF_A::_0,
            true => RXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXF_A::_1
    }
}
#[doc = "Write proxy for field `RXF`"]
pub struct RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No new message available within the RxFG."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXF_A::_0)
    }
    #[doc = "The receiver FIFO is not empty. A new message is available in the RxFG."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXF_A::_1)
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
#[doc = "Overrun Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRIF_A {
    #[doc = "0: No data overrun condition."]
    _0 = 0,
    #[doc = "1: A data overrun detected."]
    _1 = 1,
}
impl From<OVRIF_A> for bool {
    #[inline(always)]
    fn from(variant: OVRIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVRIF`"]
pub type OVRIF_R = crate::R<bool, OVRIF_A>;
impl OVRIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRIF_A {
        match self.bits {
            false => OVRIF_A::_0,
            true => OVRIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRIF_A::_1
    }
}
#[doc = "Write proxy for field `OVRIF`"]
pub struct OVRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No data overrun condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRIF_A::_0)
    }
    #[doc = "A data overrun detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRIF_A::_1)
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
#[doc = "Transmitter Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSTAT_A {
    #[doc = "0: TxOK: 0<=transmit error counter<96"]
    _00 = 0,
    #[doc = "1: TxWRN: 96<=transmit error counter<128"]
    _01 = 1,
    #[doc = "2: TxERR: 128<=transmit error counter<256"]
    _10 = 2,
    #[doc = "3: Bus-off: 256<=transmit error counter"]
    _11 = 3,
}
impl From<TSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSTAT`"]
pub type TSTAT_R = crate::R<u8, TSTAT_A>;
impl TSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTAT_A {
        match self.bits {
            0 => TSTAT_A::_00,
            1 => TSTAT_A::_01,
            2 => TSTAT_A::_10,
            3 => TSTAT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TSTAT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TSTAT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TSTAT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TSTAT_A::_11
    }
}
#[doc = "Receiver Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTAT_A {
    #[doc = "0: RxOK: 0<=receive error counter<96"]
    _00 = 0,
    #[doc = "1: RxWRN: 96<=receive error counter<128"]
    _01 = 1,
    #[doc = "2: RxERR: 128<=receive error counter"]
    _10 = 2,
    #[doc = "3: Bus-off: 256<=transmit error counter (Redundant Information for the most critical CAN bus status which is \"bus-off\". This only occurs if the Tx error counter exceeds a number of 255 errors. Bus-off affects the receiver state. As soon as the transmitter leaves its bus-off state the receiver state skips to RxOK too. Refer also to TSTAT\\[1:0\\]
coding in this register. )"]
    _11 = 3,
}
impl From<RSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSTAT`"]
pub type RSTAT_R = crate::R<u8, RSTAT_A>;
impl RSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTAT_A {
        match self.bits {
            0 => RSTAT_A::_00,
            1 => RSTAT_A::_01,
            2 => RSTAT_A::_10,
            3 => RSTAT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RSTAT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RSTAT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RSTAT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RSTAT_A::_11
    }
}
#[doc = "CAN Status Change Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCIF_A {
    #[doc = "0: No change in CAN bus status occurred since last interrupt."]
    _0 = 0,
    #[doc = "1: MSCAN changed current CAN bus status."]
    _1 = 1,
}
impl From<CSCIF_A> for bool {
    #[inline(always)]
    fn from(variant: CSCIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSCIF`"]
pub type CSCIF_R = crate::R<bool, CSCIF_A>;
impl CSCIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCIF_A {
        match self.bits {
            false => CSCIF_A::_0,
            true => CSCIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCIF_A::_1
    }
}
#[doc = "Write proxy for field `CSCIF`"]
pub struct CSCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSCIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No change in CAN bus status occurred since last interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSCIF_A::_0)
    }
    #[doc = "MSCAN changed current CAN bus status."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSCIF_A::_1)
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
#[doc = "Wake-Up Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPIF_A {
    #[doc = "0: No wakeup activity observed while in sleep mode."]
    _0 = 0,
    #[doc = "1: MSCAN detected activity on the CAN bus and requested wakeup."]
    _1 = 1,
}
impl From<WUPIF_A> for bool {
    #[inline(always)]
    fn from(variant: WUPIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WUPIF`"]
pub type WUPIF_R = crate::R<bool, WUPIF_A>;
impl WUPIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPIF_A {
        match self.bits {
            false => WUPIF_A::_0,
            true => WUPIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUPIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUPIF_A::_1
    }
}
#[doc = "Write proxy for field `WUPIF`"]
pub struct WUPIF_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No wakeup activity observed while in sleep mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUPIF_A::_0)
    }
    #[doc = "MSCAN detected activity on the CAN bus and requested wakeup."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUPIF_A::_1)
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
    #[doc = "Bit 0 - Receive Buffer Full Flag"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Interrupt Flag"]
    #[inline(always)]
    pub fn ovrif(&self) -> OVRIF_R {
        OVRIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Transmitter Status"]
    #[inline(always)]
    pub fn tstat(&self) -> TSTAT_R {
        TSTAT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Receiver Status"]
    #[inline(always)]
    pub fn rstat(&self) -> RSTAT_R {
        RSTAT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - CAN Status Change Interrupt Flag"]
    #[inline(always)]
    pub fn cscif(&self) -> CSCIF_R {
        CSCIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wake-Up Interrupt Flag"]
    #[inline(always)]
    pub fn wupif(&self) -> WUPIF_R {
        WUPIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Buffer Full Flag"]
    #[inline(always)]
    pub fn rxf(&mut self) -> RXF_W {
        RXF_W { w: self }
    }
    #[doc = "Bit 1 - Overrun Interrupt Flag"]
    #[inline(always)]
    pub fn ovrif(&mut self) -> OVRIF_W {
        OVRIF_W { w: self }
    }
    #[doc = "Bit 6 - CAN Status Change Interrupt Flag"]
    #[inline(always)]
    pub fn cscif(&mut self) -> CSCIF_W {
        CSCIF_W { w: self }
    }
    #[doc = "Bit 7 - Wake-Up Interrupt Flag"]
    #[inline(always)]
    pub fn wupif(&mut self) -> WUPIF_W {
        WUPIF_W { w: self }
    }
}

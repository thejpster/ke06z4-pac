#[doc = "Reader of register CANRIER"]
pub type R = crate::R<u8, super::CANRIER>;
#[doc = "Writer for register CANRIER"]
pub type W = crate::W<u8, super::CANRIER>;
#[doc = "Register CANRIER `reset()`'s with value 0"]
impl crate::ResetValue for super::CANRIER {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receiver Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIE_A {
    #[doc = "0: No interrupt request is generated from this event."]
    _0 = 0,
    #[doc = "1: A receive buffer full (successful message reception) event causes a receiver interrupt request."]
    _1 = 1,
}
impl From<RXFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFIE`"]
pub type RXFIE_R = crate::R<bool, RXFIE_A>;
impl RXFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIE_A {
        match self.bits {
            false => RXFIE_A::_0,
            true => RXFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXFIE_A::_1
    }
}
#[doc = "Write proxy for field `RXFIE`"]
pub struct RXFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt request is generated from this event."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFIE_A::_0)
    }
    #[doc = "A receive buffer full (successful message reception) event causes a receiver interrupt request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFIE_A::_1)
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
#[doc = "Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRIE_A {
    #[doc = "0: No interrupt request is generated from this event."]
    _0 = 0,
    #[doc = "1: An overrun event causes an error interrupt request."]
    _1 = 1,
}
impl From<OVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVRIE`"]
pub type OVRIE_R = crate::R<bool, OVRIE_A>;
impl OVRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRIE_A {
        match self.bits {
            false => OVRIE_A::_0,
            true => OVRIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRIE_A::_1
    }
}
#[doc = "Write proxy for field `OVRIE`"]
pub struct OVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt request is generated from this event."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRIE_A::_0)
    }
    #[doc = "An overrun event causes an error interrupt request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRIE_A::_1)
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
#[doc = "Transmitter Status Change Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSTATE_A {
    #[doc = "0: Do not generate any CSCIF interrupt caused by transmitter state changes."]
    _00 = 0,
    #[doc = "1: Generate CSCIF interrupt only if the transmitter enters or leaves \"bus-off\" state. Discard other transmitter state changes for generating CSCIF interrupt."]
    _01 = 1,
    #[doc = "2: Generate CSCIF interrupt only if the transmitter enters or leaves \"TxErr\" or \"bus-off\" state. Discard other transmitter state changes for generating CSCIF interrupt."]
    _10 = 2,
    #[doc = "3: Generate CSCIF interrupt on all state changes."]
    _11 = 3,
}
impl From<TSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSTATE`"]
pub type TSTATE_R = crate::R<u8, TSTATE_A>;
impl TSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTATE_A {
        match self.bits {
            0 => TSTATE_A::_00,
            1 => TSTATE_A::_01,
            2 => TSTATE_A::_10,
            3 => TSTATE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TSTATE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TSTATE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TSTATE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TSTATE_A::_11
    }
}
#[doc = "Write proxy for field `TSTATE`"]
pub struct TSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTATE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do not generate any CSCIF interrupt caused by transmitter state changes."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TSTATE_A::_00)
    }
    #[doc = "Generate CSCIF interrupt only if the transmitter enters or leaves \"bus-off\" state. Discard other transmitter state changes for generating CSCIF interrupt."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TSTATE_A::_01)
    }
    #[doc = "Generate CSCIF interrupt only if the transmitter enters or leaves \"TxErr\" or \"bus-off\" state. Discard other transmitter state changes for generating CSCIF interrupt."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TSTATE_A::_10)
    }
    #[doc = "Generate CSCIF interrupt on all state changes."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TSTATE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u8) & 0x03) << 2);
        self.w
    }
}
#[doc = "Receiver Status Change Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTATE_A {
    #[doc = "0: Do not generate any CSCIF interrupt caused by receiver state changes."]
    _00 = 0,
    #[doc = "1: Generate CSCIF interrupt only if the receiver enters or leaves \"bus-off\" state. Discard other receiver state changes for generating CSCIF interrupt."]
    _01 = 1,
    #[doc = "2: Generate CSCIF interrupt only if the receiver enters or leaves \"RxErr\" or \"bus-off\"Bus-off state is only defined for transmitters by the CAN standard (see Bosch CAN 2.0A/B protocol specification). Because the only possible state change for the transmitter from bus-off to TxOK also forces the receiver to skip its current state to RxOK, the coding of the RXSTAT\\[1:0\\]
flags define an additional bus-off state for the receiver state. Discard other receiver state changes for generating CSCIF interrupt."]
    _10 = 2,
    #[doc = "3: Generate CSCIF interrupt on all state changes."]
    _11 = 3,
}
impl From<RSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSTATE`"]
pub type RSTATE_R = crate::R<u8, RSTATE_A>;
impl RSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTATE_A {
        match self.bits {
            0 => RSTATE_A::_00,
            1 => RSTATE_A::_01,
            2 => RSTATE_A::_10,
            3 => RSTATE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RSTATE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RSTATE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RSTATE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RSTATE_A::_11
    }
}
#[doc = "Write proxy for field `RSTATE`"]
pub struct RSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTATE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do not generate any CSCIF interrupt caused by receiver state changes."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSTATE_A::_00)
    }
    #[doc = "Generate CSCIF interrupt only if the receiver enters or leaves \"bus-off\" state. Discard other receiver state changes for generating CSCIF interrupt."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSTATE_A::_01)
    }
    #[doc = "Generate CSCIF interrupt only if the receiver enters or leaves \"RxErr\" or \"bus-off\"Bus-off state is only defined for transmitters by the CAN standard (see Bosch CAN 2.0A/B protocol specification). Because the only possible state change for the transmitter from bus-off to TxOK also forces the receiver to skip its current state to RxOK, the coding of the RXSTAT\\[1:0\\]
flags define an additional bus-off state for the receiver state. Discard other receiver state changes for generating CSCIF interrupt."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSTATE_A::_10)
    }
    #[doc = "Generate CSCIF interrupt on all state changes."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RSTATE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "CAN Status Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCIE_A {
    #[doc = "0: No interrupt request is generated from this event."]
    _0 = 0,
    #[doc = "1: A CAN Status Change event causes an error interrupt request."]
    _1 = 1,
}
impl From<CSCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CSCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSCIE`"]
pub type CSCIE_R = crate::R<bool, CSCIE_A>;
impl CSCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCIE_A {
        match self.bits {
            false => CSCIE_A::_0,
            true => CSCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCIE_A::_1
    }
}
#[doc = "Write proxy for field `CSCIE`"]
pub struct CSCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt request is generated from this event."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSCIE_A::_0)
    }
    #[doc = "A CAN Status Change event causes an error interrupt request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSCIE_A::_1)
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
#[doc = "WakeUp Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPIE_A {
    #[doc = "0: No interrupt request is generated from this event."]
    _0 = 0,
    #[doc = "1: A wake-up event causes a Wake-Up interrupt request."]
    _1 = 1,
}
impl From<WUPIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WUPIE`"]
pub type WUPIE_R = crate::R<bool, WUPIE_A>;
impl WUPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPIE_A {
        match self.bits {
            false => WUPIE_A::_0,
            true => WUPIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUPIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUPIE_A::_1
    }
}
#[doc = "Write proxy for field `WUPIE`"]
pub struct WUPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt request is generated from this event."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUPIE_A::_0)
    }
    #[doc = "A wake-up event causes a Wake-Up interrupt request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUPIE_A::_1)
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
    #[doc = "Bit 0 - Receiver Full Interrupt Enable"]
    #[inline(always)]
    pub fn rxfie(&self) -> RXFIE_R {
        RXFIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Transmitter Status Change Enable"]
    #[inline(always)]
    pub fn tstate(&self) -> TSTATE_R {
        TSTATE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Receiver Status Change Enable"]
    #[inline(always)]
    pub fn rstate(&self) -> RSTATE_R {
        RSTATE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - CAN Status Change Interrupt Enable"]
    #[inline(always)]
    pub fn cscie(&self) -> CSCIE_R {
        CSCIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - WakeUp Interrupt Enable"]
    #[inline(always)]
    pub fn wupie(&self) -> WUPIE_R {
        WUPIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Full Interrupt Enable"]
    #[inline(always)]
    pub fn rxfie(&mut self) -> RXFIE_W {
        RXFIE_W { w: self }
    }
    #[doc = "Bit 1 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W {
        OVRIE_W { w: self }
    }
    #[doc = "Bits 2:3 - Transmitter Status Change Enable"]
    #[inline(always)]
    pub fn tstate(&mut self) -> TSTATE_W {
        TSTATE_W { w: self }
    }
    #[doc = "Bits 4:5 - Receiver Status Change Enable"]
    #[inline(always)]
    pub fn rstate(&mut self) -> RSTATE_W {
        RSTATE_W { w: self }
    }
    #[doc = "Bit 6 - CAN Status Change Interrupt Enable"]
    #[inline(always)]
    pub fn cscie(&mut self) -> CSCIE_W {
        CSCIE_W { w: self }
    }
    #[doc = "Bit 7 - WakeUp Interrupt Enable"]
    #[inline(always)]
    pub fn wupie(&mut self) -> WUPIE_W {
        WUPIE_W { w: self }
    }
}

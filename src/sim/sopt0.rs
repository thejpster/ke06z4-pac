#[doc = "Reader of register SOPT0"]
pub type R = crate::R<u32, super::SOPT0>;
#[doc = "Writer for register SOPT0"]
pub type W = crate::W<u32, super::SOPT0>;
#[doc = "Register SOPT0 `reset()`'s with value 0x0e"]
impl crate::ResetValue for super::SOPT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0e
    }
}
#[doc = "NMI Pin Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIE_A {
    #[doc = "0: PTB4/KBI0_P12/FTM2_CH4/SPI0_MISO/ACMP1_IN2/NMI pin functions as PTB4, KBI0_P12, FTM2_CH4, SPI0_MISO or ACMP1_IN2."]
    _0 = 0,
    #[doc = "1: PTB4/KBI0_P12/FTM2_CH4/SPI0_MISO/ACMP1_IN2/NMI pin functions as NMI."]
    _1 = 1,
}
impl From<NMIE_A> for bool {
    #[inline(always)]
    fn from(variant: NMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NMIE`"]
pub type NMIE_R = crate::R<bool, NMIE_A>;
impl NMIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIE_A {
        match self.bits {
            false => NMIE_A::_0,
            true => NMIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMIE_A::_1
    }
}
#[doc = "Write proxy for field `NMIE`"]
pub struct NMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PTB4/KBI0_P12/FTM2_CH4/SPI0_MISO/ACMP1_IN2/NMI pin functions as PTB4, KBI0_P12, FTM2_CH4, SPI0_MISO or ACMP1_IN2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NMIE_A::_0)
    }
    #[doc = "PTB4/KBI0_P12/FTM2_CH4/SPI0_MISO/ACMP1_IN2/NMI pin functions as NMI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NMIE_A::_1)
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
#[doc = "RESET Pin Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTPE_A {
    #[doc = "0: PTA5/KBI0_P5/IRQ/TCLK0/RESET pin functions as PTA5/KBI0_P5/IRQ/TCLK0."]
    _0 = 0,
    #[doc = "1: PTA5/KBI0_P5/IRQ/TCLK0/RESET pin functions as RESET."]
    _1 = 1,
}
impl From<RSTPE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTPE`"]
pub type RSTPE_R = crate::R<bool, RSTPE_A>;
impl RSTPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTPE_A {
        match self.bits {
            false => RSTPE_A::_0,
            true => RSTPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSTPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSTPE_A::_1
    }
}
#[doc = "Write proxy for field `RSTPE`"]
pub struct RSTPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PTA5/KBI0_P5/IRQ/TCLK0/RESET pin functions as PTA5/KBI0_P5/IRQ/TCLK0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTPE_A::_0)
    }
    #[doc = "PTA5/KBI0_P5/IRQ/TCLK0/RESET pin functions as RESET."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTPE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Single Wire Debug Port Pin Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDE_A {
    #[doc = "0: PTA4/KBI0_P4/ACMP0_OUT/SWD_DIO as PTA4 or ACMP0_OUT function, PTC4/KBI0_P20/RTC_CLKOUT/FTM1_CH0/ACMP0_IN2/SWD_CLK as PTC4, KBI0_P20, RTC_CLKOUT, FTM1_CH0, OR ACMP0_IN2 function."]
    _0 = 0,
    #[doc = "1: PTA4/KBI0_P4/ACMP0_OUT/SWD_DIO as SWD_DIO function, PTC4/KBI0_P20/RTC_CLKOUT/FTM1_CH0/ACMP0_IN2/SWD_CLK as SWD_CLK function."]
    _1 = 1,
}
impl From<SWDE_A> for bool {
    #[inline(always)]
    fn from(variant: SWDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWDE`"]
pub type SWDE_R = crate::R<bool, SWDE_A>;
impl SWDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWDE_A {
        match self.bits {
            false => SWDE_A::_0,
            true => SWDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWDE_A::_1
    }
}
#[doc = "Write proxy for field `SWDE`"]
pub struct SWDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PTA4/KBI0_P4/ACMP0_OUT/SWD_DIO as PTA4 or ACMP0_OUT function, PTC4/KBI0_P20/RTC_CLKOUT/FTM1_CH0/ACMP0_IN2/SWD_CLK as PTC4, KBI0_P20, RTC_CLKOUT, FTM1_CH0, OR ACMP0_IN2 function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWDE_A::_0)
    }
    #[doc = "PTA4/KBI0_P4/ACMP0_OUT/SWD_DIO as SWD_DIO function, PTC4/KBI0_P20/RTC_CLKOUT/FTM1_CH0/ACMP0_IN2/SWD_CLK as SWD_CLK function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWDE_A::_1)
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
#[doc = "ACMP Trigger FTM2 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTRG_A {
    #[doc = "0: ACMP0 out"]
    _0 = 0,
    #[doc = "1: ACMP1 out"]
    _1 = 1,
}
impl From<ACTRG_A> for bool {
    #[inline(always)]
    fn from(variant: ACTRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACTRG`"]
pub type ACTRG_R = crate::R<bool, ACTRG_A>;
impl ACTRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTRG_A {
        match self.bits {
            false => ACTRG_A::_0,
            true => ACTRG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTRG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTRG_A::_1
    }
}
#[doc = "Write proxy for field `ACTRG`"]
pub struct ACTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTRG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACMP0 out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACTRG_A::_0)
    }
    #[doc = "ACMP1 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACTRG_A::_1)
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
#[doc = "UART0 RxD Filter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXDFE_A {
    #[doc = "0: RXD0 input signal is connected to UART0 module directly."]
    _00 = 0,
    #[doc = "1: RXD0 input signal is filtered by ACMP0, then injected to UART0."]
    _01 = 1,
    #[doc = "2: RXD0 input signal is filtered by ACMP1, then injected to UART0."]
    _10 = 2,
}
impl From<RXDFE_A> for u8 {
    #[inline(always)]
    fn from(variant: RXDFE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXDFE`"]
pub type RXDFE_R = crate::R<u8, RXDFE_A>;
impl RXDFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXDFE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXDFE_A::_00),
            1 => Val(RXDFE_A::_01),
            2 => Val(RXDFE_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RXDFE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RXDFE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RXDFE_A::_10
    }
}
#[doc = "Write proxy for field `RXDFE`"]
pub struct RXDFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDFE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "RXD0 input signal is connected to UART0 module directly."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RXDFE_A::_00)
    }
    #[doc = "RXD0 input signal is filtered by ACMP0, then injected to UART0."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RXDFE_A::_01)
    }
    #[doc = "RXD0 input signal is filtered by ACMP1, then injected to UART0."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RXDFE_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Real-Time Counter Capture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCC_A {
    #[doc = "0: RTC overflow is not connected to FTM1 input channel 1."]
    _0 = 0,
    #[doc = "1: RTC overflow is connected to FTM1 input channel 1."]
    _1 = 1,
}
impl From<RTCC_A> for bool {
    #[inline(always)]
    fn from(variant: RTCC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCC`"]
pub type RTCC_R = crate::R<bool, RTCC_A>;
impl RTCC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCC_A {
        match self.bits {
            false => RTCC_A::_0,
            true => RTCC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCC_A::_1
    }
}
#[doc = "Write proxy for field `RTCC`"]
pub struct RTCC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC overflow is not connected to FTM1 input channel 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCC_A::_0)
    }
    #[doc = "RTC overflow is connected to FTM1 input channel 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Analog Comparator to Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACIC_A {
    #[doc = "0: ACMP0 output is not connected to FTM1 input channel 0."]
    _0 = 0,
    #[doc = "1: ACMP0 output is connected to FTM1 input channel 0."]
    _1 = 1,
}
impl From<ACIC_A> for bool {
    #[inline(always)]
    fn from(variant: ACIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACIC`"]
pub type ACIC_R = crate::R<bool, ACIC_A>;
impl ACIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACIC_A {
        match self.bits {
            false => ACIC_A::_0,
            true => ACIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACIC_A::_1
    }
}
#[doc = "Write proxy for field `ACIC`"]
pub struct ACIC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACIC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACMP0 output is not connected to FTM1 input channel 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACIC_A::_0)
    }
    #[doc = "ACMP0 output is connected to FTM1 input channel 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACIC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "UART0_RX Capture Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDCE_A {
    #[doc = "0: UART0_RX input signal is connected to the UART0 module only."]
    _0 = 0,
    #[doc = "1: UART0_RX input signal is connected to the UART0 module and FTM0 channel 1."]
    _1 = 1,
}
impl From<RXDCE_A> for bool {
    #[inline(always)]
    fn from(variant: RXDCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDCE`"]
pub type RXDCE_R = crate::R<bool, RXDCE_A>;
impl RXDCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDCE_A {
        match self.bits {
            false => RXDCE_A::_0,
            true => RXDCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDCE_A::_1
    }
}
#[doc = "Write proxy for field `RXDCE`"]
pub struct RXDCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART0_RX input signal is connected to the UART0 module only."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDCE_A::_0)
    }
    #[doc = "UART0_RX input signal is connected to the UART0 module and FTM0 channel 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDCE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "FTM2 Synchronization Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTMSYNC_AW {
    #[doc = "0: No synchronization triggered."]
    _0 = 0,
    #[doc = "1: Generates a PWM synchronization trigger to the FTM2 modules."]
    _1 = 1,
}
impl From<FTMSYNC_AW> for bool {
    #[inline(always)]
    fn from(variant: FTMSYNC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FTMSYNC`"]
pub struct FTMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTMSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTMSYNC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No synchronization triggered."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTMSYNC_AW::_0)
    }
    #[doc = "Generates a PWM synchronization trigger to the FTM2 modules."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTMSYNC_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "UART0_TX Modulation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDME_A {
    #[doc = "0: UART0_TX output is connected to pinout directly."]
    _0 = 0,
    #[doc = "1: UART0_TX output is modulated by FTM0 channel 0 before mapped to pinout."]
    _1 = 1,
}
impl From<TXDME_A> for bool {
    #[inline(always)]
    fn from(variant: TXDME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXDME`"]
pub type TXDME_R = crate::R<bool, TXDME_A>;
impl TXDME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDME_A {
        match self.bits {
            false => TXDME_A::_0,
            true => TXDME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXDME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXDME_A::_1
    }
}
#[doc = "Write proxy for field `TXDME`"]
pub struct TXDME_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART0_TX output is connected to pinout directly."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDME_A::_0)
    }
    #[doc = "UART0_TX output is modulated by FTM0 channel 0 before mapped to pinout."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDME_A::_1)
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
#[doc = "BUS Clock Output select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BUSREF_A {
    #[doc = "0: Bus"]
    _000 = 0,
    #[doc = "1: Bus divided by 2"]
    _001 = 1,
    #[doc = "2: Bus divided by 4"]
    _010 = 2,
    #[doc = "3: Bus divided by 8"]
    _011 = 3,
    #[doc = "4: Bus divided by 16"]
    _100 = 4,
    #[doc = "5: Bus divided by 32"]
    _101 = 5,
    #[doc = "6: Bus divided by 64"]
    _110 = 6,
    #[doc = "7: Bus divided by 128"]
    _111 = 7,
}
impl From<BUSREF_A> for u8 {
    #[inline(always)]
    fn from(variant: BUSREF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BUSREF`"]
pub type BUSREF_R = crate::R<u8, BUSREF_A>;
impl BUSREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSREF_A {
        match self.bits {
            0 => BUSREF_A::_000,
            1 => BUSREF_A::_001,
            2 => BUSREF_A::_010,
            3 => BUSREF_A::_011,
            4 => BUSREF_A::_100,
            5 => BUSREF_A::_101,
            6 => BUSREF_A::_110,
            7 => BUSREF_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == BUSREF_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == BUSREF_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == BUSREF_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == BUSREF_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == BUSREF_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == BUSREF_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == BUSREF_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == BUSREF_A::_111
    }
}
#[doc = "Write proxy for field `BUSREF`"]
pub struct BUSREF_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSREF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bus"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(BUSREF_A::_000)
    }
    #[doc = "Bus divided by 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(BUSREF_A::_001)
    }
    #[doc = "Bus divided by 4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(BUSREF_A::_010)
    }
    #[doc = "Bus divided by 8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(BUSREF_A::_011)
    }
    #[doc = "Bus divided by 16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(BUSREF_A::_100)
    }
    #[doc = "Bus divided by 32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(BUSREF_A::_101)
    }
    #[doc = "Bus divided by 64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(BUSREF_A::_110)
    }
    #[doc = "Bus divided by 128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(BUSREF_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Bus Clock Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOE_A {
    #[doc = "0: Bus clock output is disabled on PTH2."]
    _0 = 0,
    #[doc = "1: Bus clock output is enabled on PTH2."]
    _1 = 1,
}
impl From<CLKOE_A> for bool {
    #[inline(always)]
    fn from(variant: CLKOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKOE`"]
pub type CLKOE_R = crate::R<bool, CLKOE_A>;
impl CLKOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOE_A {
        match self.bits {
            false => CLKOE_A::_0,
            true => CLKOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKOE_A::_1
    }
}
#[doc = "Write proxy for field `CLKOE`"]
pub struct CLKOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock output is disabled on PTH2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKOE_A::_0)
    }
    #[doc = "Bus clock output is enabled on PTH2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKOE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "ADC Hardware Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADHWT_A {
    #[doc = "0: RTC overflow as the ADC hardware trigger"]
    _000 = 0,
    #[doc = "1: FTM0 as the ADC hardware trigger"]
    _001 = 1,
    #[doc = "2: FTM2 init trigger with 8-bit programmable counter delay"]
    _010 = 2,
    #[doc = "3: FTM2 match trigger with 8-bit programmable counter delay"]
    _011 = 3,
    #[doc = "4: PIT channel0 overflow as the ADC hardware trigger"]
    _100 = 4,
    #[doc = "5: PIT channel1 overflow as the ADC hardware trigger"]
    _101 = 5,
    #[doc = "6: ACMP0 out as the ADC hardware trigger."]
    _110 = 6,
    #[doc = "7: ACMP1 out as the ADC hardware trigger"]
    _111 = 7,
}
impl From<ADHWT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADHWT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADHWT`"]
pub type ADHWT_R = crate::R<u8, ADHWT_A>;
impl ADHWT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADHWT_A {
        match self.bits {
            0 => ADHWT_A::_000,
            1 => ADHWT_A::_001,
            2 => ADHWT_A::_010,
            3 => ADHWT_A::_011,
            4 => ADHWT_A::_100,
            5 => ADHWT_A::_101,
            6 => ADHWT_A::_110,
            7 => ADHWT_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ADHWT_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ADHWT_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ADHWT_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ADHWT_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ADHWT_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ADHWT_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == ADHWT_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == ADHWT_A::_111
    }
}
#[doc = "Write proxy for field `ADHWT`"]
pub struct ADHWT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHWT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADHWT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RTC overflow as the ADC hardware trigger"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADHWT_A::_000)
    }
    #[doc = "FTM0 as the ADC hardware trigger"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ADHWT_A::_001)
    }
    #[doc = "FTM2 init trigger with 8-bit programmable counter delay"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ADHWT_A::_010)
    }
    #[doc = "FTM2 match trigger with 8-bit programmable counter delay"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ADHWT_A::_011)
    }
    #[doc = "PIT channel0 overflow as the ADC hardware trigger"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ADHWT_A::_100)
    }
    #[doc = "PIT channel1 overflow as the ADC hardware trigger"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ADHWT_A::_101)
    }
    #[doc = "ACMP0 out as the ADC hardware trigger."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(ADHWT_A::_110)
    }
    #[doc = "ACMP1 out as the ADC hardware trigger"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(ADHWT_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "FTM2 Trigger Delay Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLYACT_A {
    #[doc = "0: The delay is inactive."]
    _0 = 0,
    #[doc = "1: The delay is active."]
    _1 = 1,
}
impl From<DLYACT_A> for bool {
    #[inline(always)]
    fn from(variant: DLYACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLYACT`"]
pub type DLYACT_R = crate::R<bool, DLYACT_A>;
impl DLYACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYACT_A {
        match self.bits {
            false => DLYACT_A::_0,
            true => DLYACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYACT_A::_1
    }
}
#[doc = "Reader of field `DELAY`"]
pub type DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DELAY`"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - NMI Pin Enable"]
    #[inline(always)]
    pub fn nmie(&self) -> NMIE_R {
        NMIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RESET Pin Enable"]
    #[inline(always)]
    pub fn rstpe(&self) -> RSTPE_R {
        RSTPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Single Wire Debug Port Pin Enable"]
    #[inline(always)]
    pub fn swde(&self) -> SWDE_R {
        SWDE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACMP Trigger FTM2 selection"]
    #[inline(always)]
    pub fn actrg(&self) -> ACTRG_R {
        ACTRG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - UART0 RxD Filter Select"]
    #[inline(always)]
    pub fn rxdfe(&self) -> RXDFE_R {
        RXDFE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Real-Time Counter Capture"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Analog Comparator to Input Capture Enable"]
    #[inline(always)]
    pub fn acic(&self) -> ACIC_R {
        ACIC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UART0_RX Capture Select"]
    #[inline(always)]
    pub fn rxdce(&self) -> RXDCE_R {
        RXDCE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - UART0_TX Modulation Select"]
    #[inline(always)]
    pub fn txdme(&self) -> TXDME_R {
        TXDME_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - BUS Clock Output select"]
    #[inline(always)]
    pub fn busref(&self) -> BUSREF_R {
        BUSREF_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - Bus Clock Output Enable"]
    #[inline(always)]
    pub fn clkoe(&self) -> CLKOE_R {
        CLKOE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - ADC Hardware Trigger Source"]
    #[inline(always)]
    pub fn adhwt(&self) -> ADHWT_R {
        ADHWT_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - FTM2 Trigger Delay Active"]
    #[inline(always)]
    pub fn dlyact(&self) -> DLYACT_R {
        DLYACT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - FTM2 Trigger Delay"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - NMI Pin Enable"]
    #[inline(always)]
    pub fn nmie(&mut self) -> NMIE_W {
        NMIE_W { w: self }
    }
    #[doc = "Bit 2 - RESET Pin Enable"]
    #[inline(always)]
    pub fn rstpe(&mut self) -> RSTPE_W {
        RSTPE_W { w: self }
    }
    #[doc = "Bit 3 - Single Wire Debug Port Pin Enable"]
    #[inline(always)]
    pub fn swde(&mut self) -> SWDE_W {
        SWDE_W { w: self }
    }
    #[doc = "Bit 5 - ACMP Trigger FTM2 selection"]
    #[inline(always)]
    pub fn actrg(&mut self) -> ACTRG_W {
        ACTRG_W { w: self }
    }
    #[doc = "Bits 8:9 - UART0 RxD Filter Select"]
    #[inline(always)]
    pub fn rxdfe(&mut self) -> RXDFE_W {
        RXDFE_W { w: self }
    }
    #[doc = "Bit 10 - Real-Time Counter Capture"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RTCC_W {
        RTCC_W { w: self }
    }
    #[doc = "Bit 11 - Analog Comparator to Input Capture Enable"]
    #[inline(always)]
    pub fn acic(&mut self) -> ACIC_W {
        ACIC_W { w: self }
    }
    #[doc = "Bit 12 - UART0_RX Capture Select"]
    #[inline(always)]
    pub fn rxdce(&mut self) -> RXDCE_W {
        RXDCE_W { w: self }
    }
    #[doc = "Bit 14 - FTM2 Synchronization Select"]
    #[inline(always)]
    pub fn ftmsync(&mut self) -> FTMSYNC_W {
        FTMSYNC_W { w: self }
    }
    #[doc = "Bit 15 - UART0_TX Modulation Select"]
    #[inline(always)]
    pub fn txdme(&mut self) -> TXDME_W {
        TXDME_W { w: self }
    }
    #[doc = "Bits 16:18 - BUS Clock Output select"]
    #[inline(always)]
    pub fn busref(&mut self) -> BUSREF_W {
        BUSREF_W { w: self }
    }
    #[doc = "Bit 19 - Bus Clock Output Enable"]
    #[inline(always)]
    pub fn clkoe(&mut self) -> CLKOE_W {
        CLKOE_W { w: self }
    }
    #[doc = "Bits 20:22 - ADC Hardware Trigger Source"]
    #[inline(always)]
    pub fn adhwt(&mut self) -> ADHWT_W {
        ADHWT_W { w: self }
    }
    #[doc = "Bits 24:31 - FTM2 Trigger Delay"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
}

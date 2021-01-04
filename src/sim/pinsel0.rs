#[doc = "Reader of register PINSEL0"]
pub type R = crate::R<u32, super::PINSEL0>;
#[doc = "Writer for register PINSEL0"]
pub type W = crate::W<u32, super::PINSEL0>;
#[doc = "Register PINSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "IRQ Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IRQPS_A {
    #[doc = "0: IRQ is mapped on PTA5."]
    _000 = 0,
    #[doc = "1: IRQ is mapped on PTI0."]
    _001 = 1,
    #[doc = "2: IRQ is mapped on PTI1."]
    _010 = 2,
    #[doc = "3: IRQ is mapped on PTI2."]
    _011 = 3,
    #[doc = "4: IRQ is mapped on PTI3."]
    _100 = 4,
    #[doc = "5: IRQ is mapped on PTI4."]
    _101 = 5,
    #[doc = "6: IRQ is mapped on PTI5."]
    _110 = 6,
    #[doc = "7: IRQ is mapped on PTI6."]
    _111 = 7,
}
impl From<IRQPS_A> for u8 {
    #[inline(always)]
    fn from(variant: IRQPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IRQPS`"]
pub type IRQPS_R = crate::R<u8, IRQPS_A>;
impl IRQPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQPS_A {
        match self.bits {
            0 => IRQPS_A::_000,
            1 => IRQPS_A::_001,
            2 => IRQPS_A::_010,
            3 => IRQPS_A::_011,
            4 => IRQPS_A::_100,
            5 => IRQPS_A::_101,
            6 => IRQPS_A::_110,
            7 => IRQPS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == IRQPS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == IRQPS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == IRQPS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == IRQPS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == IRQPS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == IRQPS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == IRQPS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == IRQPS_A::_111
    }
}
#[doc = "Write proxy for field `IRQPS`"]
pub struct IRQPS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQPS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "IRQ is mapped on PTA5."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(IRQPS_A::_000)
    }
    #[doc = "IRQ is mapped on PTI0."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(IRQPS_A::_001)
    }
    #[doc = "IRQ is mapped on PTI1."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(IRQPS_A::_010)
    }
    #[doc = "IRQ is mapped on PTI2."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(IRQPS_A::_011)
    }
    #[doc = "IRQ is mapped on PTI3."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(IRQPS_A::_100)
    }
    #[doc = "IRQ is mapped on PTI4."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(IRQPS_A::_101)
    }
    #[doc = "IRQ is mapped on PTI5."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(IRQPS_A::_110)
    }
    #[doc = "IRQ is mapped on PTI6."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(IRQPS_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "RTCO Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCPS_A {
    #[doc = "0: RTCO is mapped on PTC4."]
    _0 = 0,
    #[doc = "1: RTCO is mapped on PTC5."]
    _1 = 1,
}
impl From<RTCPS_A> for bool {
    #[inline(always)]
    fn from(variant: RTCPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCPS`"]
pub type RTCPS_R = crate::R<bool, RTCPS_A>;
impl RTCPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCPS_A {
        match self.bits {
            false => RTCPS_A::_0,
            true => RTCPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCPS_A::_1
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
            self.bit(variant.into())
        }
    }
    #[doc = "RTCO is mapped on PTC4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCPS_A::_0)
    }
    #[doc = "RTCO is mapped on PTC5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCPS_A::_1)
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
#[doc = "I2C0 Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0PS_A {
    #[doc = "0: I2C0_SCL and I2C0_SDA are mapped on PTA3 and PTA2, respectively."]
    _0 = 0,
    #[doc = "1: I2C0_SCL and I2C0_SDA are mapped on PTB7 and PTB6, respectively."]
    _1 = 1,
}
impl From<I2C0PS_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C0PS`"]
pub type I2C0PS_R = crate::R<bool, I2C0PS_A>;
impl I2C0PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0PS_A {
        match self.bits {
            false => I2C0PS_A::_0,
            true => I2C0PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C0PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C0PS_A::_1
    }
}
#[doc = "Write proxy for field `I2C0PS`"]
pub struct I2C0PS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C0_SCL and I2C0_SDA are mapped on PTA3 and PTA2, respectively."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C0PS_A::_0)
    }
    #[doc = "I2C0_SCL and I2C0_SDA are mapped on PTB7 and PTB6, respectively."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C0PS_A::_1)
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
#[doc = "SPI0 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0PS_A {
    #[doc = "0: SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS are mapped on PTB2, PTB3, PTB4, and PTB5."]
    _0 = 0,
    #[doc = "1: SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS are mapped on PTE0, PTE1, PTE2, and PTE3."]
    _1 = 1,
}
impl From<SPI0PS_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0PS`"]
pub type SPI0PS_R = crate::R<bool, SPI0PS_A>;
impl SPI0PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0PS_A {
        match self.bits {
            false => SPI0PS_A::_0,
            true => SPI0PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI0PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI0PS_A::_1
    }
}
#[doc = "Write proxy for field `SPI0PS`"]
pub struct SPI0PS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS are mapped on PTB2, PTB3, PTB4, and PTB5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0PS_A::_0)
    }
    #[doc = "SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS are mapped on PTE0, PTE1, PTE2, and PTE3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0PS_A::_1)
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
#[doc = "UART0 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0PS_A {
    #[doc = "0: UART0_RX and UART0_TX are mapped on PTB0 and PTB1."]
    _0 = 0,
    #[doc = "1: UART0_RX and UART0_TX are mapped on PTA2 and PTA3."]
    _1 = 1,
}
impl From<UART0PS_A> for bool {
    #[inline(always)]
    fn from(variant: UART0PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0PS`"]
pub type UART0PS_R = crate::R<bool, UART0PS_A>;
impl UART0PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0PS_A {
        match self.bits {
            false => UART0PS_A::_0,
            true => UART0PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART0PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART0PS_A::_1
    }
}
#[doc = "Write proxy for field `UART0PS`"]
pub struct UART0PS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART0_RX and UART0_TX are mapped on PTB0 and PTB1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0PS_A::_0)
    }
    #[doc = "UART0_RX and UART0_TX are mapped on PTA2 and PTA3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0PS_A::_1)
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
#[doc = "FTM0_CH0 Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0PS0_A {
    #[doc = "0: FTM0_CH0 channels are mapped on PTA0."]
    _0 = 0,
    #[doc = "1: FTM0_CH0 channels are mapped on PTB2."]
    _1 = 1,
}
impl From<FTM0PS0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0PS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM0PS0`"]
pub type FTM0PS0_R = crate::R<bool, FTM0PS0_A>;
impl FTM0PS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0PS0_A {
        match self.bits {
            false => FTM0PS0_A::_0,
            true => FTM0PS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0PS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0PS0_A::_1
    }
}
#[doc = "Write proxy for field `FTM0PS0`"]
pub struct FTM0PS0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0PS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0PS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_CH0 channels are mapped on PTA0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0PS0_A::_0)
    }
    #[doc = "FTM0_CH0 channels are mapped on PTB2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0PS0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "FTM0_CH1 Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0PS1_A {
    #[doc = "0: FTM0_CH1 channels are mapped on PTA1."]
    _0 = 0,
    #[doc = "1: FTM0_CH1 channels are mapped on PTB3."]
    _1 = 1,
}
impl From<FTM0PS1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0PS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM0PS1`"]
pub type FTM0PS1_R = crate::R<bool, FTM0PS1_A>;
impl FTM0PS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0PS1_A {
        match self.bits {
            false => FTM0PS1_A::_0,
            true => FTM0PS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0PS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0PS1_A::_1
    }
}
#[doc = "Write proxy for field `FTM0PS1`"]
pub struct FTM0PS1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0PS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0PS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_CH1 channels are mapped on PTA1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0PS1_A::_0)
    }
    #[doc = "FTM0_CH1 channels are mapped on PTB3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0PS1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "FTM1_CH0 Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1PS0_A {
    #[doc = "0: FTM1_CH0 channels are mapped on PTC4."]
    _0 = 0,
    #[doc = "1: FTM1_CH0 channels are mapped on PTH2."]
    _1 = 1,
}
impl From<FTM1PS0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1PS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM1PS0`"]
pub type FTM1PS0_R = crate::R<bool, FTM1PS0_A>;
impl FTM1PS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1PS0_A {
        match self.bits {
            false => FTM1PS0_A::_0,
            true => FTM1PS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1PS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1PS0_A::_1
    }
}
#[doc = "Write proxy for field `FTM1PS0`"]
pub struct FTM1PS0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1PS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1PS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM1_CH0 channels are mapped on PTC4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1PS0_A::_0)
    }
    #[doc = "FTM1_CH0 channels are mapped on PTH2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1PS0_A::_1)
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
#[doc = "FTM1_CH1 Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1PS1_A {
    #[doc = "0: FTM1_CH1 channels are mapped on PTC5."]
    _0 = 0,
    #[doc = "1: FTM1_CH1 channels are mapped on PTE7."]
    _1 = 1,
}
impl From<FTM1PS1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1PS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM1PS1`"]
pub type FTM1PS1_R = crate::R<bool, FTM1PS1_A>;
impl FTM1PS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1PS1_A {
        match self.bits {
            false => FTM1PS1_A::_0,
            true => FTM1PS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1PS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1PS1_A::_1
    }
}
#[doc = "Write proxy for field `FTM1PS1`"]
pub struct FTM1PS1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1PS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1PS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM1_CH1 channels are mapped on PTC5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1PS1_A::_0)
    }
    #[doc = "FTM1_CH1 channels are mapped on PTE7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1PS1_A::_1)
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
#[doc = "FTM0 TCLK Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM0CLKPS_A {
    #[doc = "0: Selects TCLK0 for FTM0 module.."]
    _00 = 0,
    #[doc = "1: Selects TCLK1 for FTM0 module."]
    _01 = 1,
    #[doc = "2: Selects TCLK2 for FTM0 module."]
    _10 = 2,
}
impl From<FTM0CLKPS_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM0CLKPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM0CLKPS`"]
pub type FTM0CLKPS_R = crate::R<u8, FTM0CLKPS_A>;
impl FTM0CLKPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM0CLKPS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM0CLKPS_A::_00),
            1 => Val(FTM0CLKPS_A::_01),
            2 => Val(FTM0CLKPS_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM0CLKPS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM0CLKPS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM0CLKPS_A::_10
    }
}
#[doc = "Write proxy for field `FTM0CLKPS`"]
pub struct FTM0CLKPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0CLKPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0CLKPS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects TCLK0 for FTM0 module.."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM0CLKPS_A::_00)
    }
    #[doc = "Selects TCLK1 for FTM0 module."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM0CLKPS_A::_01)
    }
    #[doc = "Selects TCLK2 for FTM0 module."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM0CLKPS_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "FTM1 TCLK Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM1CLKPS_A {
    #[doc = "0: Selects TCLK0 for FTM1 module.."]
    _00 = 0,
    #[doc = "1: Selects TCLK1 for FTM1 module."]
    _01 = 1,
    #[doc = "2: Selects TCLK2 for FTM1 module."]
    _10 = 2,
}
impl From<FTM1CLKPS_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM1CLKPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM1CLKPS`"]
pub type FTM1CLKPS_R = crate::R<u8, FTM1CLKPS_A>;
impl FTM1CLKPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM1CLKPS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM1CLKPS_A::_00),
            1 => Val(FTM1CLKPS_A::_01),
            2 => Val(FTM1CLKPS_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM1CLKPS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM1CLKPS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM1CLKPS_A::_10
    }
}
#[doc = "Write proxy for field `FTM1CLKPS`"]
pub struct FTM1CLKPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1CLKPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1CLKPS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects TCLK0 for FTM1 module.."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM1CLKPS_A::_00)
    }
    #[doc = "Selects TCLK1 for FTM1 module."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM1CLKPS_A::_01)
    }
    #[doc = "Selects TCLK2 for FTM1 module."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM1CLKPS_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "FTM2 TCLK Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM2CLKPS_A {
    #[doc = "0: Selects TCLK0 for FTM2 module.."]
    _00 = 0,
    #[doc = "1: Selects TCLK1 for FTM2 module."]
    _01 = 1,
    #[doc = "2: Selects TCLK2 for FTM2 module."]
    _10 = 2,
}
impl From<FTM2CLKPS_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2CLKPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM2CLKPS`"]
pub type FTM2CLKPS_R = crate::R<u8, FTM2CLKPS_A>;
impl FTM2CLKPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM2CLKPS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM2CLKPS_A::_00),
            1 => Val(FTM2CLKPS_A::_01),
            2 => Val(FTM2CLKPS_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM2CLKPS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM2CLKPS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM2CLKPS_A::_10
    }
}
#[doc = "Write proxy for field `FTM2CLKPS`"]
pub struct FTM2CLKPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2CLKPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2CLKPS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects TCLK0 for FTM2 module.."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2CLKPS_A::_00)
    }
    #[doc = "Selects TCLK1 for FTM2 module."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2CLKPS_A::_01)
    }
    #[doc = "Selects TCLK2 for FTM2 module."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM2CLKPS_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "PWT TCLK Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWTCLKPS_A {
    #[doc = "0: Selects TCLK0 for PWT module."]
    _00 = 0,
    #[doc = "1: Selects TCLK1 for PWT module."]
    _01 = 1,
    #[doc = "2: Selects TCLK2 for PWT module."]
    _10 = 2,
}
impl From<PWTCLKPS_A> for u8 {
    #[inline(always)]
    fn from(variant: PWTCLKPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWTCLKPS`"]
pub type PWTCLKPS_R = crate::R<u8, PWTCLKPS_A>;
impl PWTCLKPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWTCLKPS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWTCLKPS_A::_00),
            1 => Val(PWTCLKPS_A::_01),
            2 => Val(PWTCLKPS_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PWTCLKPS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PWTCLKPS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PWTCLKPS_A::_10
    }
}
#[doc = "Write proxy for field `PWTCLKPS`"]
pub struct PWTCLKPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWTCLKPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWTCLKPS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects TCLK0 for PWT module."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PWTCLKPS_A::_00)
    }
    #[doc = "Selects TCLK1 for PWT module."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PWTCLKPS_A::_01)
    }
    #[doc = "Selects TCLK2 for PWT module."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PWTCLKPS_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - IRQ Port Pin Select"]
    #[inline(always)]
    pub fn irqps(&self) -> IRQPS_R {
        IRQPS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - RTCO Pin Select"]
    #[inline(always)]
    pub fn rtcps(&self) -> RTCPS_R {
        RTCPS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C0 Port Pin Select"]
    #[inline(always)]
    pub fn i2c0ps(&self) -> I2C0PS_R {
        I2C0PS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI0 Pin Select"]
    #[inline(always)]
    pub fn spi0ps(&self) -> SPI0PS_R {
        SPI0PS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART0 Pin Select"]
    #[inline(always)]
    pub fn uart0ps(&self) -> UART0PS_R {
        UART0PS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FTM0_CH0 Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps0(&self) -> FTM0PS0_R {
        FTM0PS0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FTM0_CH1 Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps1(&self) -> FTM0PS1_R {
        FTM0PS1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FTM1_CH0 Port Pin Select"]
    #[inline(always)]
    pub fn ftm1ps0(&self) -> FTM1PS0_R {
        FTM1PS0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FTM1_CH1 Port Pin Select"]
    #[inline(always)]
    pub fn ftm1ps1(&self) -> FTM1PS1_R {
        FTM1PS1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - FTM0 TCLK Pin Select"]
    #[inline(always)]
    pub fn ftm0clkps(&self) -> FTM0CLKPS_R {
        FTM0CLKPS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - FTM1 TCLK Pin Select"]
    #[inline(always)]
    pub fn ftm1clkps(&self) -> FTM1CLKPS_R {
        FTM1CLKPS_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - FTM2 TCLK Pin Select"]
    #[inline(always)]
    pub fn ftm2clkps(&self) -> FTM2CLKPS_R {
        FTM2CLKPS_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - PWT TCLK Pin Select"]
    #[inline(always)]
    pub fn pwtclkps(&self) -> PWTCLKPS_R {
        PWTCLKPS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - IRQ Port Pin Select"]
    #[inline(always)]
    pub fn irqps(&mut self) -> IRQPS_W {
        IRQPS_W { w: self }
    }
    #[doc = "Bit 4 - RTCO Pin Select"]
    #[inline(always)]
    pub fn rtcps(&mut self) -> RTCPS_W {
        RTCPS_W { w: self }
    }
    #[doc = "Bit 5 - I2C0 Port Pin Select"]
    #[inline(always)]
    pub fn i2c0ps(&mut self) -> I2C0PS_W {
        I2C0PS_W { w: self }
    }
    #[doc = "Bit 6 - SPI0 Pin Select"]
    #[inline(always)]
    pub fn spi0ps(&mut self) -> SPI0PS_W {
        SPI0PS_W { w: self }
    }
    #[doc = "Bit 7 - UART0 Pin Select"]
    #[inline(always)]
    pub fn uart0ps(&mut self) -> UART0PS_W {
        UART0PS_W { w: self }
    }
    #[doc = "Bit 8 - FTM0_CH0 Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps0(&mut self) -> FTM0PS0_W {
        FTM0PS0_W { w: self }
    }
    #[doc = "Bit 9 - FTM0_CH1 Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps1(&mut self) -> FTM0PS1_W {
        FTM0PS1_W { w: self }
    }
    #[doc = "Bit 10 - FTM1_CH0 Port Pin Select"]
    #[inline(always)]
    pub fn ftm1ps0(&mut self) -> FTM1PS0_W {
        FTM1PS0_W { w: self }
    }
    #[doc = "Bit 11 - FTM1_CH1 Port Pin Select"]
    #[inline(always)]
    pub fn ftm1ps1(&mut self) -> FTM1PS1_W {
        FTM1PS1_W { w: self }
    }
    #[doc = "Bits 24:25 - FTM0 TCLK Pin Select"]
    #[inline(always)]
    pub fn ftm0clkps(&mut self) -> FTM0CLKPS_W {
        FTM0CLKPS_W { w: self }
    }
    #[doc = "Bits 26:27 - FTM1 TCLK Pin Select"]
    #[inline(always)]
    pub fn ftm1clkps(&mut self) -> FTM1CLKPS_W {
        FTM1CLKPS_W { w: self }
    }
    #[doc = "Bits 28:29 - FTM2 TCLK Pin Select"]
    #[inline(always)]
    pub fn ftm2clkps(&mut self) -> FTM2CLKPS_W {
        FTM2CLKPS_W { w: self }
    }
    #[doc = "Bits 30:31 - PWT TCLK Pin Select"]
    #[inline(always)]
    pub fn pwtclkps(&mut self) -> PWTCLKPS_W {
        PWTCLKPS_W { w: self }
    }
}

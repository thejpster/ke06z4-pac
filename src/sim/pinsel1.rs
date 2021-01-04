#[doc = "Reader of register PINSEL1"]
pub type R = crate::R<u32, super::PINSEL1>;
#[doc = "Writer for register PINSEL1"]
pub type W = crate::W<u32, super::PINSEL1>;
#[doc = "Register PINSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FTM2 Channel 0 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM2PS0_A {
    #[doc = "0: FTM2 CH0 mapped on PTC0."]
    _00 = 0,
    #[doc = "1: FTM2 CH0 mapped on PTH0."]
    _01 = 1,
    #[doc = "2: FTM2 CH0 mapped on PTF0."]
    _10 = 2,
}
impl From<FTM2PS0_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2PS0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM2PS0`"]
pub type FTM2PS0_R = crate::R<u8, FTM2PS0_A>;
impl FTM2PS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM2PS0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM2PS0_A::_00),
            1 => Val(FTM2PS0_A::_01),
            2 => Val(FTM2PS0_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM2PS0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM2PS0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM2PS0_A::_10
    }
}
#[doc = "Write proxy for field `FTM2PS0`"]
pub struct FTM2PS0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM2 CH0 mapped on PTC0."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2PS0_A::_00)
    }
    #[doc = "FTM2 CH0 mapped on PTH0."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2PS0_A::_01)
    }
    #[doc = "FTM2 CH0 mapped on PTF0."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM2PS0_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "FTM2 Channel 1 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM2PS1_A {
    #[doc = "0: FTM2 CH1 mapped on PTC1."]
    _00 = 0,
    #[doc = "1: FTM2 CH1 mapped on PTH1."]
    _01 = 1,
    #[doc = "2: FTM2 CH1 mapped on PTF1."]
    _10 = 2,
}
impl From<FTM2PS1_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2PS1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM2PS1`"]
pub type FTM2PS1_R = crate::R<u8, FTM2PS1_A>;
impl FTM2PS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM2PS1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM2PS1_A::_00),
            1 => Val(FTM2PS1_A::_01),
            2 => Val(FTM2PS1_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM2PS1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM2PS1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM2PS1_A::_10
    }
}
#[doc = "Write proxy for field `FTM2PS1`"]
pub struct FTM2PS1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM2 CH1 mapped on PTC1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2PS1_A::_00)
    }
    #[doc = "FTM2 CH1 mapped on PTH1."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2PS1_A::_01)
    }
    #[doc = "FTM2 CH1 mapped on PTF1."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM2PS1_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "FTM2 Channel 2 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM2PS2_A {
    #[doc = "0: FTM2 CH2 mapped on PTC2."]
    _00 = 0,
    #[doc = "1: FTM2 CH2 mapped on PTD0."]
    _01 = 1,
    #[doc = "2: FTM2 CH2 mapped on PTG4."]
    _10 = 2,
}
impl From<FTM2PS2_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2PS2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM2PS2`"]
pub type FTM2PS2_R = crate::R<u8, FTM2PS2_A>;
impl FTM2PS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM2PS2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM2PS2_A::_00),
            1 => Val(FTM2PS2_A::_01),
            2 => Val(FTM2PS2_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM2PS2_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM2PS2_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM2PS2_A::_10
    }
}
#[doc = "Write proxy for field `FTM2PS2`"]
pub struct FTM2PS2_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM2 CH2 mapped on PTC2."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2PS2_A::_00)
    }
    #[doc = "FTM2 CH2 mapped on PTD0."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2PS2_A::_01)
    }
    #[doc = "FTM2 CH2 mapped on PTG4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM2PS2_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "FTM2 Channel 3 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM2PS3_A {
    #[doc = "0: FTM2 CH3 mapped on PTC3."]
    _00 = 0,
    #[doc = "1: FTM2 CH3 mapped on PTD1."]
    _01 = 1,
    #[doc = "2: FTM2 CH3 mapped on PTG5."]
    _10 = 2,
}
impl From<FTM2PS3_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2PS3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM2PS3`"]
pub type FTM2PS3_R = crate::R<u8, FTM2PS3_A>;
impl FTM2PS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM2PS3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM2PS3_A::_00),
            1 => Val(FTM2PS3_A::_01),
            2 => Val(FTM2PS3_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM2PS3_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM2PS3_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM2PS3_A::_10
    }
}
#[doc = "Write proxy for field `FTM2PS3`"]
pub struct FTM2PS3_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM2 CH3 mapped on PTC3."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2PS3_A::_00)
    }
    #[doc = "FTM2 CH3 mapped on PTD1."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2PS3_A::_01)
    }
    #[doc = "FTM2 CH3 mapped on PTG5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM2PS3_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "FTM2 Channel4 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2PS4_A {
    #[doc = "0: FTM2 CH4 mapped on PTB4."]
    _0 = 0,
    #[doc = "1: FTM2 CH4 mapped on PTG6."]
    _1 = 1,
}
impl From<FTM2PS4_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2PS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM2PS4`"]
pub type FTM2PS4_R = crate::R<bool, FTM2PS4_A>;
impl FTM2PS4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2PS4_A {
        match self.bits {
            false => FTM2PS4_A::_0,
            true => FTM2PS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2PS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2PS4_A::_1
    }
}
#[doc = "Write proxy for field `FTM2PS4`"]
pub struct FTM2PS4_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM2 CH4 mapped on PTB4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2PS4_A::_0)
    }
    #[doc = "FTM2 CH4 mapped on PTG6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2PS4_A::_1)
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
#[doc = "FTM2 Channel 5 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2PS5_A {
    #[doc = "0: FTM2 CH5 mapped on PTB5."]
    _0 = 0,
    #[doc = "1: FTM2 CH5 mapped on PTG7."]
    _1 = 1,
}
impl From<FTM2PS5_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2PS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM2PS5`"]
pub type FTM2PS5_R = crate::R<bool, FTM2PS5_A>;
impl FTM2PS5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2PS5_A {
        match self.bits {
            false => FTM2PS5_A::_0,
            true => FTM2PS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2PS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2PS5_A::_1
    }
}
#[doc = "Write proxy for field `FTM2PS5`"]
pub struct FTM2PS5_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM2 CH5 mapped on PTB5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2PS5_A::_0)
    }
    #[doc = "FTM2 CH5 mapped on PTG7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2PS5_A::_1)
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
#[doc = "I2C1 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1PS_A {
    #[doc = "0: I2C1_SCL on PTE1, I2C1_SDA on PTE0."]
    _0 = 0,
    #[doc = "1: I2C1_SCL on PTH4, I2C1_SDA on PTH3."]
    _1 = 1,
}
impl From<I2C1PS_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C1PS`"]
pub type I2C1PS_R = crate::R<bool, I2C1PS_A>;
impl I2C1PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1PS_A {
        match self.bits {
            false => I2C1PS_A::_0,
            true => I2C1PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C1PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C1PS_A::_1
    }
}
#[doc = "Write proxy for field `I2C1PS`"]
pub struct I2C1PS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C1_SCL on PTE1, I2C1_SDA on PTE0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C1PS_A::_0)
    }
    #[doc = "I2C1_SCL on PTH4, I2C1_SDA on PTH3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C1PS_A::_1)
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
#[doc = "SPI1 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1PS_A {
    #[doc = "0: SPI1_SCK, SPI1_MOSI, SPI1_MISO, and SPI1_PCS are mapped on PTD0, PTD1, PTD2, and PTD3."]
    _0 = 0,
    #[doc = "1: SPI1_SCK, SPI1_MOSI, SPI1_MISO, and SPI1_PCS are mapped on PTG4, PTG5, PTG6, and PTG7."]
    _1 = 1,
}
impl From<SPI1PS_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1PS`"]
pub type SPI1PS_R = crate::R<bool, SPI1PS_A>;
impl SPI1PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1PS_A {
        match self.bits {
            false => SPI1PS_A::_0,
            true => SPI1PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI1PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI1PS_A::_1
    }
}
#[doc = "Write proxy for field `SPI1PS`"]
pub struct SPI1PS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI1_SCK, SPI1_MOSI, SPI1_MISO, and SPI1_PCS are mapped on PTD0, PTD1, PTD2, and PTD3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1PS_A::_0)
    }
    #[doc = "SPI1_SCK, SPI1_MOSI, SPI1_MISO, and SPI1_PCS are mapped on PTG4, PTG5, PTG6, and PTG7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1PS_A::_1)
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
#[doc = "UART1 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1PS_A {
    #[doc = "0: UART1_TX on PTC7, UART1_RX on PTC6."]
    _0 = 0,
    #[doc = "1: UART1_TX on PTF3, UART1_RX on PTF2."]
    _1 = 1,
}
impl From<UART1PS_A> for bool {
    #[inline(always)]
    fn from(variant: UART1PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART1PS`"]
pub type UART1PS_R = crate::R<bool, UART1PS_A>;
impl UART1PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1PS_A {
        match self.bits {
            false => UART1PS_A::_0,
            true => UART1PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART1PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART1PS_A::_1
    }
}
#[doc = "Write proxy for field `UART1PS`"]
pub struct UART1PS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART1_TX on PTC7, UART1_RX on PTC6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1PS_A::_0)
    }
    #[doc = "UART1_TX on PTF3, UART1_RX on PTF2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1PS_A::_1)
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
#[doc = "UART2 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2PS_A {
    #[doc = "0: UART2_TX on PTD7, UART2_RX on PTD6."]
    _0 = 0,
    #[doc = "1: UART2_TX on PTI1, UART2_RX on PTI0."]
    _1 = 1,
}
impl From<UART2PS_A> for bool {
    #[inline(always)]
    fn from(variant: UART2PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART2PS`"]
pub type UART2PS_R = crate::R<bool, UART2PS_A>;
impl UART2PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2PS_A {
        match self.bits {
            false => UART2PS_A::_0,
            true => UART2PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART2PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART2PS_A::_1
    }
}
#[doc = "Write proxy for field `UART2PS`"]
pub struct UART2PS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART2_TX on PTD7, UART2_RX on PTD6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART2PS_A::_0)
    }
    #[doc = "UART2_TX on PTI1, UART2_RX on PTI0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART2PS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "PWTIN0 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWTIN0PS_A {
    #[doc = "0: PWTIN0 on PTD5."]
    _0 = 0,
    #[doc = "1: PWTIN0 on PTE2."]
    _1 = 1,
}
impl From<PWTIN0PS_A> for bool {
    #[inline(always)]
    fn from(variant: PWTIN0PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWTIN0PS`"]
pub type PWTIN0PS_R = crate::R<bool, PWTIN0PS_A>;
impl PWTIN0PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWTIN0PS_A {
        match self.bits {
            false => PWTIN0PS_A::_0,
            true => PWTIN0PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWTIN0PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWTIN0PS_A::_1
    }
}
#[doc = "Write proxy for field `PWTIN0PS`"]
pub struct PWTIN0PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWTIN0PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWTIN0PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWTIN0 on PTD5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWTIN0PS_A::_0)
    }
    #[doc = "PWTIN0 on PTE2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWTIN0PS_A::_1)
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
#[doc = "PWTIN1 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWTIN1PS_A {
    #[doc = "0: PWTIN1 on PTB0."]
    _0 = 0,
    #[doc = "1: PWTIN1 on PTH7."]
    _1 = 1,
}
impl From<PWTIN1PS_A> for bool {
    #[inline(always)]
    fn from(variant: PWTIN1PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWTIN1PS`"]
pub type PWTIN1PS_R = crate::R<bool, PWTIN1PS_A>;
impl PWTIN1PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWTIN1PS_A {
        match self.bits {
            false => PWTIN1PS_A::_0,
            true => PWTIN1PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWTIN1PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWTIN1PS_A::_1
    }
}
#[doc = "Write proxy for field `PWTIN1PS`"]
pub struct PWTIN1PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWTIN1PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWTIN1PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWTIN1 on PTB0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWTIN1PS_A::_0)
    }
    #[doc = "PWTIN1 on PTH7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWTIN1PS_A::_1)
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
#[doc = "MSCAN Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSCANPS_A {
    #[doc = "0: CAN_TX on PTC7, CAN_RX on PTC6."]
    _0 = 0,
    #[doc = "1: CAN_TX on PTE7, CAN_RX on PTH2."]
    _1 = 1,
}
impl From<MSCANPS_A> for bool {
    #[inline(always)]
    fn from(variant: MSCANPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSCANPS`"]
pub type MSCANPS_R = crate::R<bool, MSCANPS_A>;
impl MSCANPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSCANPS_A {
        match self.bits {
            false => MSCANPS_A::_0,
            true => MSCANPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSCANPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSCANPS_A::_1
    }
}
#[doc = "Write proxy for field `MSCANPS`"]
pub struct MSCANPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSCANPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSCANPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAN_TX on PTC7, CAN_RX on PTC6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSCANPS_A::_0)
    }
    #[doc = "CAN_TX on PTE7, CAN_RX on PTH2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSCANPS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FTM2 Channel 0 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps0(&self) -> FTM2PS0_R {
        FTM2PS0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - FTM2 Channel 1 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps1(&self) -> FTM2PS1_R {
        FTM2PS1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - FTM2 Channel 2 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps2(&self) -> FTM2PS2_R {
        FTM2PS2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - FTM2 Channel 3 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps3(&self) -> FTM2PS3_R {
        FTM2PS3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - FTM2 Channel4 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps4(&self) -> FTM2PS4_R {
        FTM2PS4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FTM2 Channel 5 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps5(&self) -> FTM2PS5_R {
        FTM2PS5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - I2C1 Pin Select"]
    #[inline(always)]
    pub fn i2c1ps(&self) -> I2C1PS_R {
        I2C1PS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SPI1 Pin Select"]
    #[inline(always)]
    pub fn spi1ps(&self) -> SPI1PS_R {
        SPI1PS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UART1 Pin Select"]
    #[inline(always)]
    pub fn uart1ps(&self) -> UART1PS_R {
        UART1PS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - UART2 Pin Select"]
    #[inline(always)]
    pub fn uart2ps(&self) -> UART2PS_R {
        UART2PS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PWTIN0 Pin Select"]
    #[inline(always)]
    pub fn pwtin0ps(&self) -> PWTIN0PS_R {
        PWTIN0PS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PWTIN1 Pin Select"]
    #[inline(always)]
    pub fn pwtin1ps(&self) -> PWTIN1PS_R {
        PWTIN1PS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MSCAN Pin Select"]
    #[inline(always)]
    pub fn mscanps(&self) -> MSCANPS_R {
        MSCANPS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FTM2 Channel 0 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps0(&mut self) -> FTM2PS0_W {
        FTM2PS0_W { w: self }
    }
    #[doc = "Bits 2:3 - FTM2 Channel 1 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps1(&mut self) -> FTM2PS1_W {
        FTM2PS1_W { w: self }
    }
    #[doc = "Bits 4:5 - FTM2 Channel 2 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps2(&mut self) -> FTM2PS2_W {
        FTM2PS2_W { w: self }
    }
    #[doc = "Bits 6:7 - FTM2 Channel 3 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps3(&mut self) -> FTM2PS3_W {
        FTM2PS3_W { w: self }
    }
    #[doc = "Bit 8 - FTM2 Channel4 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps4(&mut self) -> FTM2PS4_W {
        FTM2PS4_W { w: self }
    }
    #[doc = "Bit 9 - FTM2 Channel 5 Pin Select"]
    #[inline(always)]
    pub fn ftm2ps5(&mut self) -> FTM2PS5_W {
        FTM2PS5_W { w: self }
    }
    #[doc = "Bit 10 - I2C1 Pin Select"]
    #[inline(always)]
    pub fn i2c1ps(&mut self) -> I2C1PS_W {
        I2C1PS_W { w: self }
    }
    #[doc = "Bit 11 - SPI1 Pin Select"]
    #[inline(always)]
    pub fn spi1ps(&mut self) -> SPI1PS_W {
        SPI1PS_W { w: self }
    }
    #[doc = "Bit 12 - UART1 Pin Select"]
    #[inline(always)]
    pub fn uart1ps(&mut self) -> UART1PS_W {
        UART1PS_W { w: self }
    }
    #[doc = "Bit 13 - UART2 Pin Select"]
    #[inline(always)]
    pub fn uart2ps(&mut self) -> UART2PS_W {
        UART2PS_W { w: self }
    }
    #[doc = "Bit 14 - PWTIN0 Pin Select"]
    #[inline(always)]
    pub fn pwtin0ps(&mut self) -> PWTIN0PS_W {
        PWTIN0PS_W { w: self }
    }
    #[doc = "Bit 15 - PWTIN1 Pin Select"]
    #[inline(always)]
    pub fn pwtin1ps(&mut self) -> PWTIN1PS_W {
        PWTIN1PS_W { w: self }
    }
    #[doc = "Bit 16 - MSCAN Pin Select"]
    #[inline(always)]
    pub fn mscanps(&mut self) -> MSCANPS_W {
        MSCANPS_W { w: self }
    }
}

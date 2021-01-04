#[doc = "Reader of register SOPT1"]
pub type R = crate::R<u32, super::SOPT1>;
#[doc = "Writer for register SOPT1"]
pub type W = crate::W<u32, super::SOPT1>;
#[doc = "Register SOPT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I2C0 4-Wire Interface Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C04WEN_A {
    #[doc = "0: I2C0 4-wire interface configuration is disabled."]
    _0 = 0,
    #[doc = "1: I2C0 4-wire interface configuration is enabled."]
    _1 = 1,
}
impl From<I2C04WEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C04WEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C04WEN`"]
pub type I2C04WEN_R = crate::R<bool, I2C04WEN_A>;
impl I2C04WEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C04WEN_A {
        match self.bits {
            false => I2C04WEN_A::_0,
            true => I2C04WEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C04WEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C04WEN_A::_1
    }
}
#[doc = "Write proxy for field `I2C04WEN`"]
pub struct I2C04WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C04WEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C04WEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C0 4-wire interface configuration is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C04WEN_A::_0)
    }
    #[doc = "I2C0 4-wire interface configuration is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C04WEN_A::_1)
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
#[doc = "I2C0 Output Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0OINV_A {
    #[doc = "0: Under I2C0 4-wire interface configuration, SDA_OUT and SCL_OUT are not inverted before output"]
    _0 = 0,
    #[doc = "1: Under I2C0 4-wire interface configuration, SDA_OUT and SCL_OUT are inverted before output"]
    _1 = 1,
}
impl From<I2C0OINV_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0OINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C0OINV`"]
pub type I2C0OINV_R = crate::R<bool, I2C0OINV_A>;
impl I2C0OINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0OINV_A {
        match self.bits {
            false => I2C0OINV_A::_0,
            true => I2C0OINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C0OINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C0OINV_A::_1
    }
}
#[doc = "Write proxy for field `I2C0OINV`"]
pub struct I2C0OINV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0OINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0OINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Under I2C0 4-wire interface configuration, SDA_OUT and SCL_OUT are not inverted before output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C0OINV_A::_0)
    }
    #[doc = "Under I2C0 4-wire interface configuration, SDA_OUT and SCL_OUT are inverted before output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C0OINV_A::_1)
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
#[doc = "PWT ACMP_OUT select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACPWTS_A {
    #[doc = "0: ACMP1_OUT is connectted to PWTIN2."]
    _0 = 0,
    #[doc = "1: ACMP0_OUT is connectted to PWTIN2."]
    _1 = 1,
}
impl From<ACPWTS_A> for bool {
    #[inline(always)]
    fn from(variant: ACPWTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACPWTS`"]
pub type ACPWTS_R = crate::R<bool, ACPWTS_A>;
impl ACPWTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACPWTS_A {
        match self.bits {
            false => ACPWTS_A::_0,
            true => ACPWTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACPWTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACPWTS_A::_1
    }
}
#[doc = "Write proxy for field `ACPWTS`"]
pub struct ACPWTS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACPWTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACPWTS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACMP1_OUT is connectted to PWTIN2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACPWTS_A::_0)
    }
    #[doc = "ACMP0_OUT is connectted to PWTIN2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACPWTS_A::_1)
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
#[doc = "PWT UART RX select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UARTPWTS_A {
    #[doc = "0: UART0 RX is connectted to PWTIN3."]
    _00 = 0,
    #[doc = "1: UART1 RX is connectted to PWTIN3."]
    _01 = 1,
    #[doc = "2: UART2 RX is connectted to PWTIN3."]
    _10 = 2,
}
impl From<UARTPWTS_A> for u8 {
    #[inline(always)]
    fn from(variant: UARTPWTS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UARTPWTS`"]
pub type UARTPWTS_R = crate::R<u8, UARTPWTS_A>;
impl UARTPWTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UARTPWTS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UARTPWTS_A::_00),
            1 => Val(UARTPWTS_A::_01),
            2 => Val(UARTPWTS_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UARTPWTS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UARTPWTS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UARTPWTS_A::_10
    }
}
#[doc = "Write proxy for field `UARTPWTS`"]
pub struct UARTPWTS_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTPWTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTPWTS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "UART0 RX is connectted to PWTIN3."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UARTPWTS_A::_00)
    }
    #[doc = "UART1 RX is connectted to PWTIN3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UARTPWTS_A::_01)
    }
    #[doc = "UART2 RX is connectted to PWTIN3."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UARTPWTS_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C0 4-Wire Interface Enable"]
    #[inline(always)]
    pub fn i2c04wen(&self) -> I2C04WEN_R {
        I2C04WEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C0 Output Invert"]
    #[inline(always)]
    pub fn i2c0oinv(&self) -> I2C0OINV_R {
        I2C0OINV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWT ACMP_OUT select"]
    #[inline(always)]
    pub fn acpwts(&self) -> ACPWTS_R {
        ACPWTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - PWT UART RX select"]
    #[inline(always)]
    pub fn uartpwts(&self) -> UARTPWTS_R {
        UARTPWTS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0 4-Wire Interface Enable"]
    #[inline(always)]
    pub fn i2c04wen(&mut self) -> I2C04WEN_W {
        I2C04WEN_W { w: self }
    }
    #[doc = "Bit 1 - I2C0 Output Invert"]
    #[inline(always)]
    pub fn i2c0oinv(&mut self) -> I2C0OINV_W {
        I2C0OINV_W { w: self }
    }
    #[doc = "Bit 3 - PWT ACMP_OUT select"]
    #[inline(always)]
    pub fn acpwts(&mut self) -> ACPWTS_W {
        ACPWTS_W { w: self }
    }
    #[doc = "Bits 4:5 - PWT UART RX select"]
    #[inline(always)]
    pub fn uartpwts(&mut self) -> UARTPWTS_W {
        UARTPWTS_W { w: self }
    }
}

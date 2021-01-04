#[doc = "Reader of register PUE0"]
pub type R = crate::R<u32, super::PUE0>;
#[doc = "Writer for register PUE0"]
pub type W = crate::W<u32, super::PUE0>;
#[doc = "Register PUE0 `reset()`'s with value 0x0010_0000"]
impl crate::ResetValue for super::PUE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0000
    }
}
#[doc = "Pull Enable for Port A Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE0_A {
    #[doc = "0: Pullup is disabled for port A bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 0."]
    _1 = 1,
}
impl From<PTAPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTAPE0`"]
pub type PTAPE0_R = crate::R<bool, PTAPE0_A>;
impl PTAPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE0_A {
        match self.bits {
            false => PTAPE0_A::_0,
            true => PTAPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTAPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTAPE0_A::_1
    }
}
#[doc = "Write proxy for field `PTAPE0`"]
pub struct PTAPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port A bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE0_A::_1)
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
#[doc = "Pull Enable for Port A Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE1_A {
    #[doc = "0: Pullup is disabled for port A bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 1."]
    _1 = 1,
}
impl From<PTAPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTAPE1`"]
pub type PTAPE1_R = crate::R<bool, PTAPE1_A>;
impl PTAPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE1_A {
        match self.bits {
            false => PTAPE1_A::_0,
            true => PTAPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTAPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTAPE1_A::_1
    }
}
#[doc = "Write proxy for field `PTAPE1`"]
pub struct PTAPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port A bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE1_A::_1)
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
#[doc = "Pull Enable for Port A Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE2_A {
    #[doc = "0: Pullup is disabled for port A bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 2."]
    _1 = 1,
}
impl From<PTAPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTAPE2`"]
pub type PTAPE2_R = crate::R<bool, PTAPE2_A>;
impl PTAPE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE2_A {
        match self.bits {
            false => PTAPE2_A::_0,
            true => PTAPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTAPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTAPE2_A::_1
    }
}
#[doc = "Write proxy for field `PTAPE2`"]
pub struct PTAPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port A bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE2_A::_1)
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
#[doc = "Pull Enable for Port A Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE3_A {
    #[doc = "0: Pullup is disabled for port A bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 3."]
    _1 = 1,
}
impl From<PTAPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTAPE3`"]
pub type PTAPE3_R = crate::R<bool, PTAPE3_A>;
impl PTAPE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE3_A {
        match self.bits {
            false => PTAPE3_A::_0,
            true => PTAPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTAPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTAPE3_A::_1
    }
}
#[doc = "Write proxy for field `PTAPE3`"]
pub struct PTAPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port A bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE3_A::_1)
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
#[doc = "Pull Enable for Port A Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE4_A {
    #[doc = "0: Pullup is disabled for port A bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 4."]
    _1 = 1,
}
impl From<PTAPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTAPE4`"]
pub type PTAPE4_R = crate::R<bool, PTAPE4_A>;
impl PTAPE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE4_A {
        match self.bits {
            false => PTAPE4_A::_0,
            true => PTAPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTAPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTAPE4_A::_1
    }
}
#[doc = "Write proxy for field `PTAPE4`"]
pub struct PTAPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port A bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE4_A::_1)
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
#[doc = "Pull Enable for Port A Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE5_A {
    #[doc = "0: Pullup is disabled for port A bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 5."]
    _1 = 1,
}
impl From<PTAPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTAPE5`"]
pub type PTAPE5_R = crate::R<bool, PTAPE5_A>;
impl PTAPE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE5_A {
        match self.bits {
            false => PTAPE5_A::_0,
            true => PTAPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTAPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTAPE5_A::_1
    }
}
#[doc = "Write proxy for field `PTAPE5`"]
pub struct PTAPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port A bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE5_A::_1)
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
#[doc = "Pull Enable for Port A Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE6_A {
    #[doc = "0: Pullup is disabled for port A bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 6."]
    _1 = 1,
}
impl From<PTAPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTAPE6`"]
pub type PTAPE6_R = crate::R<bool, PTAPE6_A>;
impl PTAPE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE6_A {
        match self.bits {
            false => PTAPE6_A::_0,
            true => PTAPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTAPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTAPE6_A::_1
    }
}
#[doc = "Write proxy for field `PTAPE6`"]
pub struct PTAPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port A bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE6_A::_1)
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
#[doc = "Pull Enable for Port A Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE7_A {
    #[doc = "0: Pullup is disabled for port A bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 7."]
    _1 = 1,
}
impl From<PTAPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTAPE7`"]
pub type PTAPE7_R = crate::R<bool, PTAPE7_A>;
impl PTAPE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE7_A {
        match self.bits {
            false => PTAPE7_A::_0,
            true => PTAPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTAPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTAPE7_A::_1
    }
}
#[doc = "Write proxy for field `PTAPE7`"]
pub struct PTAPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port A bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE7_A::_1)
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
#[doc = "Pull Enable for Port B Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE0_A {
    #[doc = "0: Pullup is disabled for port B bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 0."]
    _1 = 1,
}
impl From<PTBPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTBPE0`"]
pub type PTBPE0_R = crate::R<bool, PTBPE0_A>;
impl PTBPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE0_A {
        match self.bits {
            false => PTBPE0_A::_0,
            true => PTBPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTBPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTBPE0_A::_1
    }
}
#[doc = "Write proxy for field `PTBPE0`"]
pub struct PTBPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port B bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE0_A::_1)
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
#[doc = "Pull Enable for Port B Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE1_A {
    #[doc = "0: Pullup is disabled for port B bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 1."]
    _1 = 1,
}
impl From<PTBPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTBPE1`"]
pub type PTBPE1_R = crate::R<bool, PTBPE1_A>;
impl PTBPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE1_A {
        match self.bits {
            false => PTBPE1_A::_0,
            true => PTBPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTBPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTBPE1_A::_1
    }
}
#[doc = "Write proxy for field `PTBPE1`"]
pub struct PTBPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port B bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE1_A::_1)
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
#[doc = "Pull Enable for Port B Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE2_A {
    #[doc = "0: Pullup is disabled for port B bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 2."]
    _1 = 1,
}
impl From<PTBPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTBPE2`"]
pub type PTBPE2_R = crate::R<bool, PTBPE2_A>;
impl PTBPE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE2_A {
        match self.bits {
            false => PTBPE2_A::_0,
            true => PTBPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTBPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTBPE2_A::_1
    }
}
#[doc = "Write proxy for field `PTBPE2`"]
pub struct PTBPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port B bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE2_A::_1)
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
#[doc = "Pull Enable for Port B Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE3_A {
    #[doc = "0: Pullup is disabled for port B bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 3."]
    _1 = 1,
}
impl From<PTBPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTBPE3`"]
pub type PTBPE3_R = crate::R<bool, PTBPE3_A>;
impl PTBPE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE3_A {
        match self.bits {
            false => PTBPE3_A::_0,
            true => PTBPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTBPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTBPE3_A::_1
    }
}
#[doc = "Write proxy for field `PTBPE3`"]
pub struct PTBPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port B bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE3_A::_1)
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
#[doc = "Pull Enable for Port B Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE4_A {
    #[doc = "0: Pullup is disabled for port B bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 4."]
    _1 = 1,
}
impl From<PTBPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTBPE4`"]
pub type PTBPE4_R = crate::R<bool, PTBPE4_A>;
impl PTBPE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE4_A {
        match self.bits {
            false => PTBPE4_A::_0,
            true => PTBPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTBPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTBPE4_A::_1
    }
}
#[doc = "Write proxy for field `PTBPE4`"]
pub struct PTBPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port B bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE4_A::_1)
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
#[doc = "Pull Enable for Port B Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE5_A {
    #[doc = "0: Pullup is disabled for port B bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 5."]
    _1 = 1,
}
impl From<PTBPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTBPE5`"]
pub type PTBPE5_R = crate::R<bool, PTBPE5_A>;
impl PTBPE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE5_A {
        match self.bits {
            false => PTBPE5_A::_0,
            true => PTBPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTBPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTBPE5_A::_1
    }
}
#[doc = "Write proxy for field `PTBPE5`"]
pub struct PTBPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port B bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE5_A::_1)
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
#[doc = "Pull Enable for Port B Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE6_A {
    #[doc = "0: Pullup is disabled for port B bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 6."]
    _1 = 1,
}
impl From<PTBPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTBPE6`"]
pub type PTBPE6_R = crate::R<bool, PTBPE6_A>;
impl PTBPE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE6_A {
        match self.bits {
            false => PTBPE6_A::_0,
            true => PTBPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTBPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTBPE6_A::_1
    }
}
#[doc = "Write proxy for field `PTBPE6`"]
pub struct PTBPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port B bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE6_A::_1)
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
#[doc = "Pull Enable for Port B Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE7_A {
    #[doc = "0: Pullup is disabled for port B bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 7."]
    _1 = 1,
}
impl From<PTBPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTBPE7`"]
pub type PTBPE7_R = crate::R<bool, PTBPE7_A>;
impl PTBPE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE7_A {
        match self.bits {
            false => PTBPE7_A::_0,
            true => PTBPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTBPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTBPE7_A::_1
    }
}
#[doc = "Write proxy for field `PTBPE7`"]
pub struct PTBPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port B bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE7_A::_1)
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
#[doc = "Pull Enable for Port C Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE0_A {
    #[doc = "0: Pullup is disabled for port C bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 0."]
    _1 = 1,
}
impl From<PTCPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTCPE0`"]
pub type PTCPE0_R = crate::R<bool, PTCPE0_A>;
impl PTCPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE0_A {
        match self.bits {
            false => PTCPE0_A::_0,
            true => PTCPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTCPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTCPE0_A::_1
    }
}
#[doc = "Write proxy for field `PTCPE0`"]
pub struct PTCPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port C bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE0_A::_1)
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
#[doc = "Pull Enable for Port C Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE1_A {
    #[doc = "0: Pullup is disabled for port C bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 1."]
    _1 = 1,
}
impl From<PTCPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTCPE1`"]
pub type PTCPE1_R = crate::R<bool, PTCPE1_A>;
impl PTCPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE1_A {
        match self.bits {
            false => PTCPE1_A::_0,
            true => PTCPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTCPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTCPE1_A::_1
    }
}
#[doc = "Write proxy for field `PTCPE1`"]
pub struct PTCPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port C bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE2_A {
    #[doc = "0: Pullup is disabled for port C bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 2."]
    _1 = 1,
}
impl From<PTCPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTCPE2`"]
pub type PTCPE2_R = crate::R<bool, PTCPE2_A>;
impl PTCPE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE2_A {
        match self.bits {
            false => PTCPE2_A::_0,
            true => PTCPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTCPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTCPE2_A::_1
    }
}
#[doc = "Write proxy for field `PTCPE2`"]
pub struct PTCPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port C bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE3_A {
    #[doc = "0: Pullup is disabled for port C bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 3."]
    _1 = 1,
}
impl From<PTCPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTCPE3`"]
pub type PTCPE3_R = crate::R<bool, PTCPE3_A>;
impl PTCPE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE3_A {
        match self.bits {
            false => PTCPE3_A::_0,
            true => PTCPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTCPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTCPE3_A::_1
    }
}
#[doc = "Write proxy for field `PTCPE3`"]
pub struct PTCPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port C bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE3_A::_1)
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
#[doc = "Pull Enable for Port C Bit 4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE4_A {
    #[doc = "0: Pullup is disabled for port C bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 4."]
    _1 = 1,
}
impl From<PTCPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTCPE4`"]
pub type PTCPE4_R = crate::R<bool, PTCPE4_A>;
impl PTCPE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE4_A {
        match self.bits {
            false => PTCPE4_A::_0,
            true => PTCPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTCPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTCPE4_A::_1
    }
}
#[doc = "Write proxy for field `PTCPE4`"]
pub struct PTCPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port C bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE5_A {
    #[doc = "0: Pullup is disabled for port C bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 5."]
    _1 = 1,
}
impl From<PTCPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTCPE5`"]
pub type PTCPE5_R = crate::R<bool, PTCPE5_A>;
impl PTCPE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE5_A {
        match self.bits {
            false => PTCPE5_A::_0,
            true => PTCPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTCPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTCPE5_A::_1
    }
}
#[doc = "Write proxy for field `PTCPE5`"]
pub struct PTCPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port C bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE6_A {
    #[doc = "0: Pullup is disabled for port C bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 6."]
    _1 = 1,
}
impl From<PTCPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTCPE6`"]
pub type PTCPE6_R = crate::R<bool, PTCPE6_A>;
impl PTCPE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE6_A {
        match self.bits {
            false => PTCPE6_A::_0,
            true => PTCPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTCPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTCPE6_A::_1
    }
}
#[doc = "Write proxy for field `PTCPE6`"]
pub struct PTCPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port C bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE7_A {
    #[doc = "0: Pullup is disabled for port C bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 7."]
    _1 = 1,
}
impl From<PTCPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTCPE7`"]
pub type PTCPE7_R = crate::R<bool, PTCPE7_A>;
impl PTCPE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE7_A {
        match self.bits {
            false => PTCPE7_A::_0,
            true => PTCPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTCPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTCPE7_A::_1
    }
}
#[doc = "Write proxy for field `PTCPE7`"]
pub struct PTCPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port C bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE0_A {
    #[doc = "0: Pullup is disabled for port D bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 0."]
    _1 = 1,
}
impl From<PTDPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTDPE0`"]
pub type PTDPE0_R = crate::R<bool, PTDPE0_A>;
impl PTDPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE0_A {
        match self.bits {
            false => PTDPE0_A::_0,
            true => PTDPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTDPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTDPE0_A::_1
    }
}
#[doc = "Write proxy for field `PTDPE0`"]
pub struct PTDPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port D bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE1_A {
    #[doc = "0: Pullup is disabled for port D bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 1."]
    _1 = 1,
}
impl From<PTDPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTDPE1`"]
pub type PTDPE1_R = crate::R<bool, PTDPE1_A>;
impl PTDPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE1_A {
        match self.bits {
            false => PTDPE1_A::_0,
            true => PTDPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTDPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTDPE1_A::_1
    }
}
#[doc = "Write proxy for field `PTDPE1`"]
pub struct PTDPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port D bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE2_A {
    #[doc = "0: Pullup is disabled for port D bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 2."]
    _1 = 1,
}
impl From<PTDPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTDPE2`"]
pub type PTDPE2_R = crate::R<bool, PTDPE2_A>;
impl PTDPE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE2_A {
        match self.bits {
            false => PTDPE2_A::_0,
            true => PTDPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTDPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTDPE2_A::_1
    }
}
#[doc = "Write proxy for field `PTDPE2`"]
pub struct PTDPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port D bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE3_A {
    #[doc = "0: Pullup is disabled for port D bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 3."]
    _1 = 1,
}
impl From<PTDPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTDPE3`"]
pub type PTDPE3_R = crate::R<bool, PTDPE3_A>;
impl PTDPE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE3_A {
        match self.bits {
            false => PTDPE3_A::_0,
            true => PTDPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTDPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTDPE3_A::_1
    }
}
#[doc = "Write proxy for field `PTDPE3`"]
pub struct PTDPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port D bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE4_A {
    #[doc = "0: Pullup is disabled for port D bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 4."]
    _1 = 1,
}
impl From<PTDPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTDPE4`"]
pub type PTDPE4_R = crate::R<bool, PTDPE4_A>;
impl PTDPE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE4_A {
        match self.bits {
            false => PTDPE4_A::_0,
            true => PTDPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTDPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTDPE4_A::_1
    }
}
#[doc = "Write proxy for field `PTDPE4`"]
pub struct PTDPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port D bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE5_A {
    #[doc = "0: Pullup is disabled for port D bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 5."]
    _1 = 1,
}
impl From<PTDPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTDPE5`"]
pub type PTDPE5_R = crate::R<bool, PTDPE5_A>;
impl PTDPE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE5_A {
        match self.bits {
            false => PTDPE5_A::_0,
            true => PTDPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTDPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTDPE5_A::_1
    }
}
#[doc = "Write proxy for field `PTDPE5`"]
pub struct PTDPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port D bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE6_A {
    #[doc = "0: Pullup is disabled for port D bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 6."]
    _1 = 1,
}
impl From<PTDPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTDPE6`"]
pub type PTDPE6_R = crate::R<bool, PTDPE6_A>;
impl PTDPE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE6_A {
        match self.bits {
            false => PTDPE6_A::_0,
            true => PTDPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTDPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTDPE6_A::_1
    }
}
#[doc = "Write proxy for field `PTDPE6`"]
pub struct PTDPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port D bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE7_A {
    #[doc = "0: Pullup is disabled for port D bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 7."]
    _1 = 1,
}
impl From<PTDPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTDPE7`"]
pub type PTDPE7_R = crate::R<bool, PTDPE7_A>;
impl PTDPE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE7_A {
        match self.bits {
            false => PTDPE7_A::_0,
            true => PTDPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTDPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTDPE7_A::_1
    }
}
#[doc = "Write proxy for field `PTDPE7`"]
pub struct PTDPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port D bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pull Enable for Port A Bit 0"]
    #[inline(always)]
    pub fn ptape0(&self) -> PTAPE0_R {
        PTAPE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pull Enable for Port A Bit 1"]
    #[inline(always)]
    pub fn ptape1(&self) -> PTAPE1_R {
        PTAPE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pull Enable for Port A Bit 2"]
    #[inline(always)]
    pub fn ptape2(&self) -> PTAPE2_R {
        PTAPE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pull Enable for Port A Bit 3"]
    #[inline(always)]
    pub fn ptape3(&self) -> PTAPE3_R {
        PTAPE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pull Enable for Port A Bit 4"]
    #[inline(always)]
    pub fn ptape4(&self) -> PTAPE4_R {
        PTAPE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pull Enable for Port A Bit 5"]
    #[inline(always)]
    pub fn ptape5(&self) -> PTAPE5_R {
        PTAPE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pull Enable for Port A Bit 6"]
    #[inline(always)]
    pub fn ptape6(&self) -> PTAPE6_R {
        PTAPE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pull Enable for Port A Bit 7"]
    #[inline(always)]
    pub fn ptape7(&self) -> PTAPE7_R {
        PTAPE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pull Enable for Port B Bit 0"]
    #[inline(always)]
    pub fn ptbpe0(&self) -> PTBPE0_R {
        PTBPE0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pull Enable for Port B Bit 1"]
    #[inline(always)]
    pub fn ptbpe1(&self) -> PTBPE1_R {
        PTBPE1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pull Enable for Port B Bit 2"]
    #[inline(always)]
    pub fn ptbpe2(&self) -> PTBPE2_R {
        PTBPE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pull Enable for Port B Bit 3"]
    #[inline(always)]
    pub fn ptbpe3(&self) -> PTBPE3_R {
        PTBPE3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pull Enable for Port B Bit 4"]
    #[inline(always)]
    pub fn ptbpe4(&self) -> PTBPE4_R {
        PTBPE4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pull Enable for Port B Bit 5"]
    #[inline(always)]
    pub fn ptbpe5(&self) -> PTBPE5_R {
        PTBPE5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pull Enable for Port B Bit 6"]
    #[inline(always)]
    pub fn ptbpe6(&self) -> PTBPE6_R {
        PTBPE6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pull Enable for Port B Bit 7"]
    #[inline(always)]
    pub fn ptbpe7(&self) -> PTBPE7_R {
        PTBPE7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pull Enable for Port C Bit 0"]
    #[inline(always)]
    pub fn ptcpe0(&self) -> PTCPE0_R {
        PTCPE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pull Enable for Port C Bit 1"]
    #[inline(always)]
    pub fn ptcpe1(&self) -> PTCPE1_R {
        PTCPE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pull Enable for Port C Bit 2"]
    #[inline(always)]
    pub fn ptcpe2(&self) -> PTCPE2_R {
        PTCPE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Pull Enable for Port C Bit 3"]
    #[inline(always)]
    pub fn ptcpe3(&self) -> PTCPE3_R {
        PTCPE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pull Enable for Port C Bit 4"]
    #[inline(always)]
    pub fn ptcpe4(&self) -> PTCPE4_R {
        PTCPE4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Pull Enable for Port C Bit 5"]
    #[inline(always)]
    pub fn ptcpe5(&self) -> PTCPE5_R {
        PTCPE5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Pull Enable for Port C Bit 6"]
    #[inline(always)]
    pub fn ptcpe6(&self) -> PTCPE6_R {
        PTCPE6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Pull Enable for Port C Bit 7"]
    #[inline(always)]
    pub fn ptcpe7(&self) -> PTCPE7_R {
        PTCPE7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pull Enable for Port D Bit 0"]
    #[inline(always)]
    pub fn ptdpe0(&self) -> PTDPE0_R {
        PTDPE0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pull Enable for Port D Bit 1"]
    #[inline(always)]
    pub fn ptdpe1(&self) -> PTDPE1_R {
        PTDPE1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Pull Enable for Port D Bit 2"]
    #[inline(always)]
    pub fn ptdpe2(&self) -> PTDPE2_R {
        PTDPE2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Pull Enable for Port D Bit 3"]
    #[inline(always)]
    pub fn ptdpe3(&self) -> PTDPE3_R {
        PTDPE3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Pull Enable for Port D Bit 4"]
    #[inline(always)]
    pub fn ptdpe4(&self) -> PTDPE4_R {
        PTDPE4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Pull Enable for Port D Bit 5"]
    #[inline(always)]
    pub fn ptdpe5(&self) -> PTDPE5_R {
        PTDPE5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Pull Enable for Port D Bit 6"]
    #[inline(always)]
    pub fn ptdpe6(&self) -> PTDPE6_R {
        PTDPE6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Pull Enable for Port D Bit 7"]
    #[inline(always)]
    pub fn ptdpe7(&self) -> PTDPE7_R {
        PTDPE7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull Enable for Port A Bit 0"]
    #[inline(always)]
    pub fn ptape0(&mut self) -> PTAPE0_W {
        PTAPE0_W { w: self }
    }
    #[doc = "Bit 1 - Pull Enable for Port A Bit 1"]
    #[inline(always)]
    pub fn ptape1(&mut self) -> PTAPE1_W {
        PTAPE1_W { w: self }
    }
    #[doc = "Bit 2 - Pull Enable for Port A Bit 2"]
    #[inline(always)]
    pub fn ptape2(&mut self) -> PTAPE2_W {
        PTAPE2_W { w: self }
    }
    #[doc = "Bit 3 - Pull Enable for Port A Bit 3"]
    #[inline(always)]
    pub fn ptape3(&mut self) -> PTAPE3_W {
        PTAPE3_W { w: self }
    }
    #[doc = "Bit 4 - Pull Enable for Port A Bit 4"]
    #[inline(always)]
    pub fn ptape4(&mut self) -> PTAPE4_W {
        PTAPE4_W { w: self }
    }
    #[doc = "Bit 5 - Pull Enable for Port A Bit 5"]
    #[inline(always)]
    pub fn ptape5(&mut self) -> PTAPE5_W {
        PTAPE5_W { w: self }
    }
    #[doc = "Bit 6 - Pull Enable for Port A Bit 6"]
    #[inline(always)]
    pub fn ptape6(&mut self) -> PTAPE6_W {
        PTAPE6_W { w: self }
    }
    #[doc = "Bit 7 - Pull Enable for Port A Bit 7"]
    #[inline(always)]
    pub fn ptape7(&mut self) -> PTAPE7_W {
        PTAPE7_W { w: self }
    }
    #[doc = "Bit 8 - Pull Enable for Port B Bit 0"]
    #[inline(always)]
    pub fn ptbpe0(&mut self) -> PTBPE0_W {
        PTBPE0_W { w: self }
    }
    #[doc = "Bit 9 - Pull Enable for Port B Bit 1"]
    #[inline(always)]
    pub fn ptbpe1(&mut self) -> PTBPE1_W {
        PTBPE1_W { w: self }
    }
    #[doc = "Bit 10 - Pull Enable for Port B Bit 2"]
    #[inline(always)]
    pub fn ptbpe2(&mut self) -> PTBPE2_W {
        PTBPE2_W { w: self }
    }
    #[doc = "Bit 11 - Pull Enable for Port B Bit 3"]
    #[inline(always)]
    pub fn ptbpe3(&mut self) -> PTBPE3_W {
        PTBPE3_W { w: self }
    }
    #[doc = "Bit 12 - Pull Enable for Port B Bit 4"]
    #[inline(always)]
    pub fn ptbpe4(&mut self) -> PTBPE4_W {
        PTBPE4_W { w: self }
    }
    #[doc = "Bit 13 - Pull Enable for Port B Bit 5"]
    #[inline(always)]
    pub fn ptbpe5(&mut self) -> PTBPE5_W {
        PTBPE5_W { w: self }
    }
    #[doc = "Bit 14 - Pull Enable for Port B Bit 6"]
    #[inline(always)]
    pub fn ptbpe6(&mut self) -> PTBPE6_W {
        PTBPE6_W { w: self }
    }
    #[doc = "Bit 15 - Pull Enable for Port B Bit 7"]
    #[inline(always)]
    pub fn ptbpe7(&mut self) -> PTBPE7_W {
        PTBPE7_W { w: self }
    }
    #[doc = "Bit 16 - Pull Enable for Port C Bit 0"]
    #[inline(always)]
    pub fn ptcpe0(&mut self) -> PTCPE0_W {
        PTCPE0_W { w: self }
    }
    #[doc = "Bit 17 - Pull Enable for Port C Bit 1"]
    #[inline(always)]
    pub fn ptcpe1(&mut self) -> PTCPE1_W {
        PTCPE1_W { w: self }
    }
    #[doc = "Bit 18 - Pull Enable for Port C Bit 2"]
    #[inline(always)]
    pub fn ptcpe2(&mut self) -> PTCPE2_W {
        PTCPE2_W { w: self }
    }
    #[doc = "Bit 19 - Pull Enable for Port C Bit 3"]
    #[inline(always)]
    pub fn ptcpe3(&mut self) -> PTCPE3_W {
        PTCPE3_W { w: self }
    }
    #[doc = "Bit 20 - Pull Enable for Port C Bit 4"]
    #[inline(always)]
    pub fn ptcpe4(&mut self) -> PTCPE4_W {
        PTCPE4_W { w: self }
    }
    #[doc = "Bit 21 - Pull Enable for Port C Bit 5"]
    #[inline(always)]
    pub fn ptcpe5(&mut self) -> PTCPE5_W {
        PTCPE5_W { w: self }
    }
    #[doc = "Bit 22 - Pull Enable for Port C Bit 6"]
    #[inline(always)]
    pub fn ptcpe6(&mut self) -> PTCPE6_W {
        PTCPE6_W { w: self }
    }
    #[doc = "Bit 23 - Pull Enable for Port C Bit 7"]
    #[inline(always)]
    pub fn ptcpe7(&mut self) -> PTCPE7_W {
        PTCPE7_W { w: self }
    }
    #[doc = "Bit 24 - Pull Enable for Port D Bit 0"]
    #[inline(always)]
    pub fn ptdpe0(&mut self) -> PTDPE0_W {
        PTDPE0_W { w: self }
    }
    #[doc = "Bit 25 - Pull Enable for Port D Bit 1"]
    #[inline(always)]
    pub fn ptdpe1(&mut self) -> PTDPE1_W {
        PTDPE1_W { w: self }
    }
    #[doc = "Bit 26 - Pull Enable for Port D Bit 2"]
    #[inline(always)]
    pub fn ptdpe2(&mut self) -> PTDPE2_W {
        PTDPE2_W { w: self }
    }
    #[doc = "Bit 27 - Pull Enable for Port D Bit 3"]
    #[inline(always)]
    pub fn ptdpe3(&mut self) -> PTDPE3_W {
        PTDPE3_W { w: self }
    }
    #[doc = "Bit 28 - Pull Enable for Port D Bit 4"]
    #[inline(always)]
    pub fn ptdpe4(&mut self) -> PTDPE4_W {
        PTDPE4_W { w: self }
    }
    #[doc = "Bit 29 - Pull Enable for Port D Bit 5"]
    #[inline(always)]
    pub fn ptdpe5(&mut self) -> PTDPE5_W {
        PTDPE5_W { w: self }
    }
    #[doc = "Bit 30 - Pull Enable for Port D Bit 6"]
    #[inline(always)]
    pub fn ptdpe6(&mut self) -> PTDPE6_W {
        PTDPE6_W { w: self }
    }
    #[doc = "Bit 31 - Pull Enable for Port D Bit 7"]
    #[inline(always)]
    pub fn ptdpe7(&mut self) -> PTDPE7_W {
        PTDPE7_W { w: self }
    }
}

#[doc = "Reader of register PUE1"]
pub type R = crate::R<u32, super::PUE1>;
#[doc = "Writer for register PUE1"]
pub type W = crate::W<u32, super::PUE1>;
#[doc = "Register PUE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PUE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pull Enable for Port E Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE0_A {
    #[doc = "0: Pullup is disabled for port E bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 0."]
    _1 = 1,
}
impl From<PTEPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTEPE0`"]
pub type PTEPE0_R = crate::R<bool, PTEPE0_A>;
impl PTEPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE0_A {
        match self.bits {
            false => PTEPE0_A::_0,
            true => PTEPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTEPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTEPE0_A::_1
    }
}
#[doc = "Write proxy for field `PTEPE0`"]
pub struct PTEPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port E bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE0_A::_1)
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
#[doc = "Pull Enable for Port E Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE1_A {
    #[doc = "0: Pullup is disabled for port E bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 1."]
    _1 = 1,
}
impl From<PTEPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTEPE1`"]
pub type PTEPE1_R = crate::R<bool, PTEPE1_A>;
impl PTEPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE1_A {
        match self.bits {
            false => PTEPE1_A::_0,
            true => PTEPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTEPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTEPE1_A::_1
    }
}
#[doc = "Write proxy for field `PTEPE1`"]
pub struct PTEPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port E bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE1_A::_1)
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
#[doc = "Pull Enable for Port E Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE2_A {
    #[doc = "0: Pullup is disabled for port E bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 2."]
    _1 = 1,
}
impl From<PTEPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTEPE2`"]
pub type PTEPE2_R = crate::R<bool, PTEPE2_A>;
impl PTEPE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE2_A {
        match self.bits {
            false => PTEPE2_A::_0,
            true => PTEPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTEPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTEPE2_A::_1
    }
}
#[doc = "Write proxy for field `PTEPE2`"]
pub struct PTEPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port E bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE2_A::_1)
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
#[doc = "Pull Enable for Port E Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE3_A {
    #[doc = "0: Pullup is disabled for port E bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 3."]
    _1 = 1,
}
impl From<PTEPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTEPE3`"]
pub type PTEPE3_R = crate::R<bool, PTEPE3_A>;
impl PTEPE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE3_A {
        match self.bits {
            false => PTEPE3_A::_0,
            true => PTEPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTEPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTEPE3_A::_1
    }
}
#[doc = "Write proxy for field `PTEPE3`"]
pub struct PTEPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port E bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE3_A::_1)
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
#[doc = "Pull Enable for Port E Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE4_A {
    #[doc = "0: Pullup is disabled for port E bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 4."]
    _1 = 1,
}
impl From<PTEPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTEPE4`"]
pub type PTEPE4_R = crate::R<bool, PTEPE4_A>;
impl PTEPE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE4_A {
        match self.bits {
            false => PTEPE4_A::_0,
            true => PTEPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTEPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTEPE4_A::_1
    }
}
#[doc = "Write proxy for field `PTEPE4`"]
pub struct PTEPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port E bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE4_A::_1)
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
#[doc = "Pull Enable for Port E Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE5_A {
    #[doc = "0: Pullup is disabled for port E bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 5."]
    _1 = 1,
}
impl From<PTEPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTEPE5`"]
pub type PTEPE5_R = crate::R<bool, PTEPE5_A>;
impl PTEPE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE5_A {
        match self.bits {
            false => PTEPE5_A::_0,
            true => PTEPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTEPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTEPE5_A::_1
    }
}
#[doc = "Write proxy for field `PTEPE5`"]
pub struct PTEPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port E bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE5_A::_1)
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
#[doc = "Pull Enable for Port E Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE6_A {
    #[doc = "0: Pullup is disabled for port E bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 6."]
    _1 = 1,
}
impl From<PTEPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTEPE6`"]
pub type PTEPE6_R = crate::R<bool, PTEPE6_A>;
impl PTEPE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE6_A {
        match self.bits {
            false => PTEPE6_A::_0,
            true => PTEPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTEPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTEPE6_A::_1
    }
}
#[doc = "Write proxy for field `PTEPE6`"]
pub struct PTEPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port E bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE6_A::_1)
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
#[doc = "Pull Enable for Port E Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE7_A {
    #[doc = "0: Pullup is disabled for port E bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 7."]
    _1 = 1,
}
impl From<PTEPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTEPE7`"]
pub type PTEPE7_R = crate::R<bool, PTEPE7_A>;
impl PTEPE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE7_A {
        match self.bits {
            false => PTEPE7_A::_0,
            true => PTEPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTEPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTEPE7_A::_1
    }
}
#[doc = "Write proxy for field `PTEPE7`"]
pub struct PTEPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port E bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE7_A::_1)
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
#[doc = "Pull Enable for Port F Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE0_A {
    #[doc = "0: Pullup is disabled for port F bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 0."]
    _1 = 1,
}
impl From<PTFPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTFPE0`"]
pub type PTFPE0_R = crate::R<bool, PTFPE0_A>;
impl PTFPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE0_A {
        match self.bits {
            false => PTFPE0_A::_0,
            true => PTFPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTFPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTFPE0_A::_1
    }
}
#[doc = "Write proxy for field `PTFPE0`"]
pub struct PTFPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port F bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE0_A::_1)
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
#[doc = "Pull Enable for Port F Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE1_A {
    #[doc = "0: Pullup is disabled for port F bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 1."]
    _1 = 1,
}
impl From<PTFPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTFPE1`"]
pub type PTFPE1_R = crate::R<bool, PTFPE1_A>;
impl PTFPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE1_A {
        match self.bits {
            false => PTFPE1_A::_0,
            true => PTFPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTFPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTFPE1_A::_1
    }
}
#[doc = "Write proxy for field `PTFPE1`"]
pub struct PTFPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port F bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE1_A::_1)
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
#[doc = "Pull Enable for Port F Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE2_A {
    #[doc = "0: Pullup is disabled for port F bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 2."]
    _1 = 1,
}
impl From<PTFPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTFPE2`"]
pub type PTFPE2_R = crate::R<bool, PTFPE2_A>;
impl PTFPE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE2_A {
        match self.bits {
            false => PTFPE2_A::_0,
            true => PTFPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTFPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTFPE2_A::_1
    }
}
#[doc = "Write proxy for field `PTFPE2`"]
pub struct PTFPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port F bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE2_A::_1)
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
#[doc = "Pull Enable for Port F Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE3_A {
    #[doc = "0: Pullup is disabled for port F bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 3."]
    _1 = 1,
}
impl From<PTFPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTFPE3`"]
pub type PTFPE3_R = crate::R<bool, PTFPE3_A>;
impl PTFPE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE3_A {
        match self.bits {
            false => PTFPE3_A::_0,
            true => PTFPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTFPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTFPE3_A::_1
    }
}
#[doc = "Write proxy for field `PTFPE3`"]
pub struct PTFPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port F bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE3_A::_1)
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
#[doc = "Pull Enable for Port F Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE4_A {
    #[doc = "0: Pullup is disabled for port F bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 4."]
    _1 = 1,
}
impl From<PTFPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTFPE4`"]
pub type PTFPE4_R = crate::R<bool, PTFPE4_A>;
impl PTFPE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE4_A {
        match self.bits {
            false => PTFPE4_A::_0,
            true => PTFPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTFPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTFPE4_A::_1
    }
}
#[doc = "Write proxy for field `PTFPE4`"]
pub struct PTFPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port F bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE4_A::_1)
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
#[doc = "Pull Enable for Port F Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE5_A {
    #[doc = "0: Pullup is disabled for port F bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 5."]
    _1 = 1,
}
impl From<PTFPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTFPE5`"]
pub type PTFPE5_R = crate::R<bool, PTFPE5_A>;
impl PTFPE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE5_A {
        match self.bits {
            false => PTFPE5_A::_0,
            true => PTFPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTFPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTFPE5_A::_1
    }
}
#[doc = "Write proxy for field `PTFPE5`"]
pub struct PTFPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port F bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE5_A::_1)
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
#[doc = "Pull Enable for Port F Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE6_A {
    #[doc = "0: Pullup is disabled for port F bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 6."]
    _1 = 1,
}
impl From<PTFPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTFPE6`"]
pub type PTFPE6_R = crate::R<bool, PTFPE6_A>;
impl PTFPE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE6_A {
        match self.bits {
            false => PTFPE6_A::_0,
            true => PTFPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTFPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTFPE6_A::_1
    }
}
#[doc = "Write proxy for field `PTFPE6`"]
pub struct PTFPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port F bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE6_A::_1)
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
#[doc = "Pull Enable for Port F Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE7_A {
    #[doc = "0: Pullup is disabled for port F bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 7."]
    _1 = 1,
}
impl From<PTFPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTFPE7`"]
pub type PTFPE7_R = crate::R<bool, PTFPE7_A>;
impl PTFPE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE7_A {
        match self.bits {
            false => PTFPE7_A::_0,
            true => PTFPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTFPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTFPE7_A::_1
    }
}
#[doc = "Write proxy for field `PTFPE7`"]
pub struct PTFPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port F bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE7_A::_1)
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
#[doc = "Pull Enable for Port G Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE0_A {
    #[doc = "0: Pullup is disabled for port G bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 0."]
    _1 = 1,
}
impl From<PTGPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTGPE0`"]
pub type PTGPE0_R = crate::R<bool, PTGPE0_A>;
impl PTGPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE0_A {
        match self.bits {
            false => PTGPE0_A::_0,
            true => PTGPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTGPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTGPE0_A::_1
    }
}
#[doc = "Write proxy for field `PTGPE0`"]
pub struct PTGPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port G bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE0_A::_1)
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
#[doc = "Pull Enable for Port G Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE1_A {
    #[doc = "0: Pullup is disabled for port G bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 1."]
    _1 = 1,
}
impl From<PTGPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTGPE1`"]
pub type PTGPE1_R = crate::R<bool, PTGPE1_A>;
impl PTGPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE1_A {
        match self.bits {
            false => PTGPE1_A::_0,
            true => PTGPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTGPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTGPE1_A::_1
    }
}
#[doc = "Write proxy for field `PTGPE1`"]
pub struct PTGPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port G bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE1_A::_1)
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
#[doc = "Pull Enable for Port G Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE2_A {
    #[doc = "0: Pullup is disabled for port G bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 2."]
    _1 = 1,
}
impl From<PTGPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTGPE2`"]
pub type PTGPE2_R = crate::R<bool, PTGPE2_A>;
impl PTGPE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE2_A {
        match self.bits {
            false => PTGPE2_A::_0,
            true => PTGPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTGPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTGPE2_A::_1
    }
}
#[doc = "Write proxy for field `PTGPE2`"]
pub struct PTGPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port G bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE2_A::_1)
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
#[doc = "Pull Enable for Port G Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE3_A {
    #[doc = "0: Pullup is disabled for port G bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 3."]
    _1 = 1,
}
impl From<PTGPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTGPE3`"]
pub type PTGPE3_R = crate::R<bool, PTGPE3_A>;
impl PTGPE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE3_A {
        match self.bits {
            false => PTGPE3_A::_0,
            true => PTGPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTGPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTGPE3_A::_1
    }
}
#[doc = "Write proxy for field `PTGPE3`"]
pub struct PTGPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port G bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE3_A::_1)
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
#[doc = "Pull Enable for Port G Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE4_A {
    #[doc = "0: Pullup is disabled for port G bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 4."]
    _1 = 1,
}
impl From<PTGPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTGPE4`"]
pub type PTGPE4_R = crate::R<bool, PTGPE4_A>;
impl PTGPE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE4_A {
        match self.bits {
            false => PTGPE4_A::_0,
            true => PTGPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTGPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTGPE4_A::_1
    }
}
#[doc = "Write proxy for field `PTGPE4`"]
pub struct PTGPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port G bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE4_A::_1)
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
#[doc = "Pull Enable for Port G Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE5_A {
    #[doc = "0: Pullup is disabled for port G bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 5."]
    _1 = 1,
}
impl From<PTGPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTGPE5`"]
pub type PTGPE5_R = crate::R<bool, PTGPE5_A>;
impl PTGPE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE5_A {
        match self.bits {
            false => PTGPE5_A::_0,
            true => PTGPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTGPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTGPE5_A::_1
    }
}
#[doc = "Write proxy for field `PTGPE5`"]
pub struct PTGPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port G bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE5_A::_1)
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
#[doc = "Pull Enable for Port G Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE6_A {
    #[doc = "0: Pullup is disabled for port G bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 6."]
    _1 = 1,
}
impl From<PTGPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTGPE6`"]
pub type PTGPE6_R = crate::R<bool, PTGPE6_A>;
impl PTGPE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE6_A {
        match self.bits {
            false => PTGPE6_A::_0,
            true => PTGPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTGPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTGPE6_A::_1
    }
}
#[doc = "Write proxy for field `PTGPE6`"]
pub struct PTGPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port G bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE6_A::_1)
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
#[doc = "Pull Enable for Port G Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE7_A {
    #[doc = "0: Pullup is disabled for port G bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 7."]
    _1 = 1,
}
impl From<PTGPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTGPE7`"]
pub type PTGPE7_R = crate::R<bool, PTGPE7_A>;
impl PTGPE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE7_A {
        match self.bits {
            false => PTGPE7_A::_0,
            true => PTGPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTGPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTGPE7_A::_1
    }
}
#[doc = "Write proxy for field `PTGPE7`"]
pub struct PTGPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port G bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE7_A::_1)
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
#[doc = "Pull Enable for Port H Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE0_A {
    #[doc = "0: Pullup is disabled for port H bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 0."]
    _1 = 1,
}
impl From<PTHPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTHPE0`"]
pub type PTHPE0_R = crate::R<bool, PTHPE0_A>;
impl PTHPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE0_A {
        match self.bits {
            false => PTHPE0_A::_0,
            true => PTHPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTHPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTHPE0_A::_1
    }
}
#[doc = "Write proxy for field `PTHPE0`"]
pub struct PTHPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port H bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE0_A::_1)
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
#[doc = "Pull Enable for Port H Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE1_A {
    #[doc = "0: Pullup is disabled for port H bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 1."]
    _1 = 1,
}
impl From<PTHPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTHPE1`"]
pub type PTHPE1_R = crate::R<bool, PTHPE1_A>;
impl PTHPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE1_A {
        match self.bits {
            false => PTHPE1_A::_0,
            true => PTHPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTHPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTHPE1_A::_1
    }
}
#[doc = "Write proxy for field `PTHPE1`"]
pub struct PTHPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port H bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE1_A::_1)
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
#[doc = "Pull Enable for Port H Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE2_A {
    #[doc = "0: Pullup is disabled for port H bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 2."]
    _1 = 1,
}
impl From<PTHPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTHPE2`"]
pub type PTHPE2_R = crate::R<bool, PTHPE2_A>;
impl PTHPE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE2_A {
        match self.bits {
            false => PTHPE2_A::_0,
            true => PTHPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTHPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTHPE2_A::_1
    }
}
#[doc = "Write proxy for field `PTHPE2`"]
pub struct PTHPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port H bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE2_A::_1)
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
#[doc = "Pull Enable for Port H Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE3_A {
    #[doc = "0: Pullup is disabled for port H bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 3."]
    _1 = 1,
}
impl From<PTHPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTHPE3`"]
pub type PTHPE3_R = crate::R<bool, PTHPE3_A>;
impl PTHPE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE3_A {
        match self.bits {
            false => PTHPE3_A::_0,
            true => PTHPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTHPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTHPE3_A::_1
    }
}
#[doc = "Write proxy for field `PTHPE3`"]
pub struct PTHPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port H bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE3_A::_1)
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
#[doc = "Pull Enable for Port H Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE4_A {
    #[doc = "0: Pullup is disabled for port H bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 4."]
    _1 = 1,
}
impl From<PTHPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTHPE4`"]
pub type PTHPE4_R = crate::R<bool, PTHPE4_A>;
impl PTHPE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE4_A {
        match self.bits {
            false => PTHPE4_A::_0,
            true => PTHPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTHPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTHPE4_A::_1
    }
}
#[doc = "Write proxy for field `PTHPE4`"]
pub struct PTHPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port H bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE4_A::_1)
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
#[doc = "Pull Enable for Port H Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE5_A {
    #[doc = "0: Pullup is disabled for port H bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 5."]
    _1 = 1,
}
impl From<PTHPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTHPE5`"]
pub type PTHPE5_R = crate::R<bool, PTHPE5_A>;
impl PTHPE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE5_A {
        match self.bits {
            false => PTHPE5_A::_0,
            true => PTHPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTHPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTHPE5_A::_1
    }
}
#[doc = "Write proxy for field `PTHPE5`"]
pub struct PTHPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port H bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE5_A::_1)
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
#[doc = "Pull Enable for Port H Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE6_A {
    #[doc = "0: Pullup is disabled for port H bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 6."]
    _1 = 1,
}
impl From<PTHPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTHPE6`"]
pub type PTHPE6_R = crate::R<bool, PTHPE6_A>;
impl PTHPE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE6_A {
        match self.bits {
            false => PTHPE6_A::_0,
            true => PTHPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTHPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTHPE6_A::_1
    }
}
#[doc = "Write proxy for field `PTHPE6`"]
pub struct PTHPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port H bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE6_A::_1)
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
#[doc = "Pull Enable for Port H Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE7_A {
    #[doc = "0: Pullup is disabled for port H bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 7."]
    _1 = 1,
}
impl From<PTHPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTHPE7`"]
pub type PTHPE7_R = crate::R<bool, PTHPE7_A>;
impl PTHPE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE7_A {
        match self.bits {
            false => PTHPE7_A::_0,
            true => PTHPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTHPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTHPE7_A::_1
    }
}
#[doc = "Write proxy for field `PTHPE7`"]
pub struct PTHPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port H bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE7_A::_1)
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
    #[doc = "Bit 0 - Pull Enable for Port E Bit 0"]
    #[inline(always)]
    pub fn ptepe0(&self) -> PTEPE0_R {
        PTEPE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pull Enable for Port E Bit 1"]
    #[inline(always)]
    pub fn ptepe1(&self) -> PTEPE1_R {
        PTEPE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pull Enable for Port E Bit 2"]
    #[inline(always)]
    pub fn ptepe2(&self) -> PTEPE2_R {
        PTEPE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pull Enable for Port E Bit 3"]
    #[inline(always)]
    pub fn ptepe3(&self) -> PTEPE3_R {
        PTEPE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pull Enable for Port E Bit 4"]
    #[inline(always)]
    pub fn ptepe4(&self) -> PTEPE4_R {
        PTEPE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pull Enable for Port E Bit 5"]
    #[inline(always)]
    pub fn ptepe5(&self) -> PTEPE5_R {
        PTEPE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pull Enable for Port E Bit 6"]
    #[inline(always)]
    pub fn ptepe6(&self) -> PTEPE6_R {
        PTEPE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pull Enable for Port E Bit 7"]
    #[inline(always)]
    pub fn ptepe7(&self) -> PTEPE7_R {
        PTEPE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pull Enable for Port F Bit 0"]
    #[inline(always)]
    pub fn ptfpe0(&self) -> PTFPE0_R {
        PTFPE0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pull Enable for Port F Bit 1"]
    #[inline(always)]
    pub fn ptfpe1(&self) -> PTFPE1_R {
        PTFPE1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pull Enable for Port F Bit 2"]
    #[inline(always)]
    pub fn ptfpe2(&self) -> PTFPE2_R {
        PTFPE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pull Enable for Port F Bit 3"]
    #[inline(always)]
    pub fn ptfpe3(&self) -> PTFPE3_R {
        PTFPE3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pull Enable for Port F Bit 4"]
    #[inline(always)]
    pub fn ptfpe4(&self) -> PTFPE4_R {
        PTFPE4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pull Enable for Port F Bit 5"]
    #[inline(always)]
    pub fn ptfpe5(&self) -> PTFPE5_R {
        PTFPE5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pull Enable for Port F Bit 6"]
    #[inline(always)]
    pub fn ptfpe6(&self) -> PTFPE6_R {
        PTFPE6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pull Enable for Port F Bit 7"]
    #[inline(always)]
    pub fn ptfpe7(&self) -> PTFPE7_R {
        PTFPE7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pull Enable for Port G Bit 0"]
    #[inline(always)]
    pub fn ptgpe0(&self) -> PTGPE0_R {
        PTGPE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pull Enable for Port G Bit 1"]
    #[inline(always)]
    pub fn ptgpe1(&self) -> PTGPE1_R {
        PTGPE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pull Enable for Port G Bit 2"]
    #[inline(always)]
    pub fn ptgpe2(&self) -> PTGPE2_R {
        PTGPE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Pull Enable for Port G Bit 3"]
    #[inline(always)]
    pub fn ptgpe3(&self) -> PTGPE3_R {
        PTGPE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pull Enable for Port G Bit 4"]
    #[inline(always)]
    pub fn ptgpe4(&self) -> PTGPE4_R {
        PTGPE4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Pull Enable for Port G Bit 5"]
    #[inline(always)]
    pub fn ptgpe5(&self) -> PTGPE5_R {
        PTGPE5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Pull Enable for Port G Bit 6"]
    #[inline(always)]
    pub fn ptgpe6(&self) -> PTGPE6_R {
        PTGPE6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Pull Enable for Port G Bit 7"]
    #[inline(always)]
    pub fn ptgpe7(&self) -> PTGPE7_R {
        PTGPE7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pull Enable for Port H Bit 0"]
    #[inline(always)]
    pub fn pthpe0(&self) -> PTHPE0_R {
        PTHPE0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pull Enable for Port H Bit 1"]
    #[inline(always)]
    pub fn pthpe1(&self) -> PTHPE1_R {
        PTHPE1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Pull Enable for Port H Bit 2"]
    #[inline(always)]
    pub fn pthpe2(&self) -> PTHPE2_R {
        PTHPE2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Pull Enable for Port H Bit 3"]
    #[inline(always)]
    pub fn pthpe3(&self) -> PTHPE3_R {
        PTHPE3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Pull Enable for Port H Bit 4"]
    #[inline(always)]
    pub fn pthpe4(&self) -> PTHPE4_R {
        PTHPE4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Pull Enable for Port H Bit 5"]
    #[inline(always)]
    pub fn pthpe5(&self) -> PTHPE5_R {
        PTHPE5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Pull Enable for Port H Bit 6"]
    #[inline(always)]
    pub fn pthpe6(&self) -> PTHPE6_R {
        PTHPE6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Pull Enable for Port H Bit 7"]
    #[inline(always)]
    pub fn pthpe7(&self) -> PTHPE7_R {
        PTHPE7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull Enable for Port E Bit 0"]
    #[inline(always)]
    pub fn ptepe0(&mut self) -> PTEPE0_W {
        PTEPE0_W { w: self }
    }
    #[doc = "Bit 1 - Pull Enable for Port E Bit 1"]
    #[inline(always)]
    pub fn ptepe1(&mut self) -> PTEPE1_W {
        PTEPE1_W { w: self }
    }
    #[doc = "Bit 2 - Pull Enable for Port E Bit 2"]
    #[inline(always)]
    pub fn ptepe2(&mut self) -> PTEPE2_W {
        PTEPE2_W { w: self }
    }
    #[doc = "Bit 3 - Pull Enable for Port E Bit 3"]
    #[inline(always)]
    pub fn ptepe3(&mut self) -> PTEPE3_W {
        PTEPE3_W { w: self }
    }
    #[doc = "Bit 4 - Pull Enable for Port E Bit 4"]
    #[inline(always)]
    pub fn ptepe4(&mut self) -> PTEPE4_W {
        PTEPE4_W { w: self }
    }
    #[doc = "Bit 5 - Pull Enable for Port E Bit 5"]
    #[inline(always)]
    pub fn ptepe5(&mut self) -> PTEPE5_W {
        PTEPE5_W { w: self }
    }
    #[doc = "Bit 6 - Pull Enable for Port E Bit 6"]
    #[inline(always)]
    pub fn ptepe6(&mut self) -> PTEPE6_W {
        PTEPE6_W { w: self }
    }
    #[doc = "Bit 7 - Pull Enable for Port E Bit 7"]
    #[inline(always)]
    pub fn ptepe7(&mut self) -> PTEPE7_W {
        PTEPE7_W { w: self }
    }
    #[doc = "Bit 8 - Pull Enable for Port F Bit 0"]
    #[inline(always)]
    pub fn ptfpe0(&mut self) -> PTFPE0_W {
        PTFPE0_W { w: self }
    }
    #[doc = "Bit 9 - Pull Enable for Port F Bit 1"]
    #[inline(always)]
    pub fn ptfpe1(&mut self) -> PTFPE1_W {
        PTFPE1_W { w: self }
    }
    #[doc = "Bit 10 - Pull Enable for Port F Bit 2"]
    #[inline(always)]
    pub fn ptfpe2(&mut self) -> PTFPE2_W {
        PTFPE2_W { w: self }
    }
    #[doc = "Bit 11 - Pull Enable for Port F Bit 3"]
    #[inline(always)]
    pub fn ptfpe3(&mut self) -> PTFPE3_W {
        PTFPE3_W { w: self }
    }
    #[doc = "Bit 12 - Pull Enable for Port F Bit 4"]
    #[inline(always)]
    pub fn ptfpe4(&mut self) -> PTFPE4_W {
        PTFPE4_W { w: self }
    }
    #[doc = "Bit 13 - Pull Enable for Port F Bit 5"]
    #[inline(always)]
    pub fn ptfpe5(&mut self) -> PTFPE5_W {
        PTFPE5_W { w: self }
    }
    #[doc = "Bit 14 - Pull Enable for Port F Bit 6"]
    #[inline(always)]
    pub fn ptfpe6(&mut self) -> PTFPE6_W {
        PTFPE6_W { w: self }
    }
    #[doc = "Bit 15 - Pull Enable for Port F Bit 7"]
    #[inline(always)]
    pub fn ptfpe7(&mut self) -> PTFPE7_W {
        PTFPE7_W { w: self }
    }
    #[doc = "Bit 16 - Pull Enable for Port G Bit 0"]
    #[inline(always)]
    pub fn ptgpe0(&mut self) -> PTGPE0_W {
        PTGPE0_W { w: self }
    }
    #[doc = "Bit 17 - Pull Enable for Port G Bit 1"]
    #[inline(always)]
    pub fn ptgpe1(&mut self) -> PTGPE1_W {
        PTGPE1_W { w: self }
    }
    #[doc = "Bit 18 - Pull Enable for Port G Bit 2"]
    #[inline(always)]
    pub fn ptgpe2(&mut self) -> PTGPE2_W {
        PTGPE2_W { w: self }
    }
    #[doc = "Bit 19 - Pull Enable for Port G Bit 3"]
    #[inline(always)]
    pub fn ptgpe3(&mut self) -> PTGPE3_W {
        PTGPE3_W { w: self }
    }
    #[doc = "Bit 20 - Pull Enable for Port G Bit 4"]
    #[inline(always)]
    pub fn ptgpe4(&mut self) -> PTGPE4_W {
        PTGPE4_W { w: self }
    }
    #[doc = "Bit 21 - Pull Enable for Port G Bit 5"]
    #[inline(always)]
    pub fn ptgpe5(&mut self) -> PTGPE5_W {
        PTGPE5_W { w: self }
    }
    #[doc = "Bit 22 - Pull Enable for Port G Bit 6"]
    #[inline(always)]
    pub fn ptgpe6(&mut self) -> PTGPE6_W {
        PTGPE6_W { w: self }
    }
    #[doc = "Bit 23 - Pull Enable for Port G Bit 7"]
    #[inline(always)]
    pub fn ptgpe7(&mut self) -> PTGPE7_W {
        PTGPE7_W { w: self }
    }
    #[doc = "Bit 24 - Pull Enable for Port H Bit 0"]
    #[inline(always)]
    pub fn pthpe0(&mut self) -> PTHPE0_W {
        PTHPE0_W { w: self }
    }
    #[doc = "Bit 25 - Pull Enable for Port H Bit 1"]
    #[inline(always)]
    pub fn pthpe1(&mut self) -> PTHPE1_W {
        PTHPE1_W { w: self }
    }
    #[doc = "Bit 26 - Pull Enable for Port H Bit 2"]
    #[inline(always)]
    pub fn pthpe2(&mut self) -> PTHPE2_W {
        PTHPE2_W { w: self }
    }
    #[doc = "Bit 27 - Pull Enable for Port H Bit 3"]
    #[inline(always)]
    pub fn pthpe3(&mut self) -> PTHPE3_W {
        PTHPE3_W { w: self }
    }
    #[doc = "Bit 28 - Pull Enable for Port H Bit 4"]
    #[inline(always)]
    pub fn pthpe4(&mut self) -> PTHPE4_W {
        PTHPE4_W { w: self }
    }
    #[doc = "Bit 29 - Pull Enable for Port H Bit 5"]
    #[inline(always)]
    pub fn pthpe5(&mut self) -> PTHPE5_W {
        PTHPE5_W { w: self }
    }
    #[doc = "Bit 30 - Pull Enable for Port H Bit 6"]
    #[inline(always)]
    pub fn pthpe6(&mut self) -> PTHPE6_W {
        PTHPE6_W { w: self }
    }
    #[doc = "Bit 31 - Pull Enable for Port H Bit 7"]
    #[inline(always)]
    pub fn pthpe7(&mut self) -> PTHPE7_W {
        PTHPE7_W { w: self }
    }
}

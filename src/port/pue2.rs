#[doc = "Reader of register PUE2"]
pub type R = crate::R<u32, super::PUE2>;
#[doc = "Writer for register PUE2"]
pub type W = crate::W<u32, super::PUE2>;
#[doc = "Register PUE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PUE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pull Enable for Port I Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTIPE0_A {
    #[doc = "0: Pullup is disabled for port I bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port I bit 0."]
    _1 = 1,
}
impl From<PTIPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTIPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTIPE0`"]
pub type PTIPE0_R = crate::R<bool, PTIPE0_A>;
impl PTIPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTIPE0_A {
        match self.bits {
            false => PTIPE0_A::_0,
            true => PTIPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTIPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTIPE0_A::_1
    }
}
#[doc = "Write proxy for field `PTIPE0`"]
pub struct PTIPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTIPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTIPE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port I bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTIPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port I bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTIPE0_A::_1)
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
#[doc = "Pull Enable for Port I Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTIPE1_A {
    #[doc = "0: Pullup is disabled for port I bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port I bit 1."]
    _1 = 1,
}
impl From<PTIPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTIPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTIPE1`"]
pub type PTIPE1_R = crate::R<bool, PTIPE1_A>;
impl PTIPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTIPE1_A {
        match self.bits {
            false => PTIPE1_A::_0,
            true => PTIPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTIPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTIPE1_A::_1
    }
}
#[doc = "Write proxy for field `PTIPE1`"]
pub struct PTIPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTIPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTIPE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port I bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTIPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port I bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTIPE1_A::_1)
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
#[doc = "Pull Enable for Port I Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTIPE2_A {
    #[doc = "0: Pullup is disabled for port I bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port I bit 2."]
    _1 = 1,
}
impl From<PTIPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTIPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTIPE2`"]
pub type PTIPE2_R = crate::R<bool, PTIPE2_A>;
impl PTIPE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTIPE2_A {
        match self.bits {
            false => PTIPE2_A::_0,
            true => PTIPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTIPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTIPE2_A::_1
    }
}
#[doc = "Write proxy for field `PTIPE2`"]
pub struct PTIPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTIPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTIPE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port I bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTIPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port I bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTIPE2_A::_1)
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
#[doc = "Pull Enable for Port I Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTIPE3_A {
    #[doc = "0: Pullup is disabled for port I bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port I bit 3."]
    _1 = 1,
}
impl From<PTIPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTIPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTIPE3`"]
pub type PTIPE3_R = crate::R<bool, PTIPE3_A>;
impl PTIPE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTIPE3_A {
        match self.bits {
            false => PTIPE3_A::_0,
            true => PTIPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTIPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTIPE3_A::_1
    }
}
#[doc = "Write proxy for field `PTIPE3`"]
pub struct PTIPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTIPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTIPE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port I bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTIPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port I bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTIPE3_A::_1)
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
#[doc = "Pull Enable for Port I Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTIPE4_A {
    #[doc = "0: Pullup is disabled for port I bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port I bit 4."]
    _1 = 1,
}
impl From<PTIPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTIPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTIPE4`"]
pub type PTIPE4_R = crate::R<bool, PTIPE4_A>;
impl PTIPE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTIPE4_A {
        match self.bits {
            false => PTIPE4_A::_0,
            true => PTIPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTIPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTIPE4_A::_1
    }
}
#[doc = "Write proxy for field `PTIPE4`"]
pub struct PTIPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTIPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTIPE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port I bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTIPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port I bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTIPE4_A::_1)
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
#[doc = "Pull Enable for Port I Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTIPE5_A {
    #[doc = "0: Pullup is disabled for port I bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port I bit 5."]
    _1 = 1,
}
impl From<PTIPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTIPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTIPE5`"]
pub type PTIPE5_R = crate::R<bool, PTIPE5_A>;
impl PTIPE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTIPE5_A {
        match self.bits {
            false => PTIPE5_A::_0,
            true => PTIPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTIPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTIPE5_A::_1
    }
}
#[doc = "Write proxy for field `PTIPE5`"]
pub struct PTIPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTIPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTIPE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port I bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTIPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port I bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTIPE5_A::_1)
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
#[doc = "Pull Enable for Port I Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTIPE6_A {
    #[doc = "0: Pullup is disabled for port I bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port I bit 6."]
    _1 = 1,
}
impl From<PTIPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTIPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTIPE6`"]
pub type PTIPE6_R = crate::R<bool, PTIPE6_A>;
impl PTIPE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTIPE6_A {
        match self.bits {
            false => PTIPE6_A::_0,
            true => PTIPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTIPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTIPE6_A::_1
    }
}
#[doc = "Write proxy for field `PTIPE6`"]
pub struct PTIPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTIPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTIPE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup is disabled for port I bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTIPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port I bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTIPE6_A::_1)
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
impl R {
    #[doc = "Bit 0 - Pull Enable for Port I Bit 0"]
    #[inline(always)]
    pub fn ptipe0(&self) -> PTIPE0_R {
        PTIPE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pull Enable for Port I Bit 1"]
    #[inline(always)]
    pub fn ptipe1(&self) -> PTIPE1_R {
        PTIPE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pull Enable for Port I Bit 2"]
    #[inline(always)]
    pub fn ptipe2(&self) -> PTIPE2_R {
        PTIPE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pull Enable for Port I Bit 3"]
    #[inline(always)]
    pub fn ptipe3(&self) -> PTIPE3_R {
        PTIPE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pull Enable for Port I Bit 4"]
    #[inline(always)]
    pub fn ptipe4(&self) -> PTIPE4_R {
        PTIPE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pull Enable for Port I Bit 5"]
    #[inline(always)]
    pub fn ptipe5(&self) -> PTIPE5_R {
        PTIPE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pull Enable for Port I Bit 6"]
    #[inline(always)]
    pub fn ptipe6(&self) -> PTIPE6_R {
        PTIPE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull Enable for Port I Bit 0"]
    #[inline(always)]
    pub fn ptipe0(&mut self) -> PTIPE0_W {
        PTIPE0_W { w: self }
    }
    #[doc = "Bit 1 - Pull Enable for Port I Bit 1"]
    #[inline(always)]
    pub fn ptipe1(&mut self) -> PTIPE1_W {
        PTIPE1_W { w: self }
    }
    #[doc = "Bit 2 - Pull Enable for Port I Bit 2"]
    #[inline(always)]
    pub fn ptipe2(&mut self) -> PTIPE2_W {
        PTIPE2_W { w: self }
    }
    #[doc = "Bit 3 - Pull Enable for Port I Bit 3"]
    #[inline(always)]
    pub fn ptipe3(&mut self) -> PTIPE3_W {
        PTIPE3_W { w: self }
    }
    #[doc = "Bit 4 - Pull Enable for Port I Bit 4"]
    #[inline(always)]
    pub fn ptipe4(&mut self) -> PTIPE4_W {
        PTIPE4_W { w: self }
    }
    #[doc = "Bit 5 - Pull Enable for Port I Bit 5"]
    #[inline(always)]
    pub fn ptipe5(&mut self) -> PTIPE5_W {
        PTIPE5_W { w: self }
    }
    #[doc = "Bit 6 - Pull Enable for Port I Bit 6"]
    #[inline(always)]
    pub fn ptipe6(&mut self) -> PTIPE6_W {
        PTIPE6_W { w: self }
    }
}

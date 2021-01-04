#[doc = "Reader of register HDRVE"]
pub type R = crate::R<u32, super::HDRVE>;
#[doc = "Writer for register HDRVE"]
pub type W = crate::W<u32, super::HDRVE>;
#[doc = "Register HDRVE `reset()`'s with value 0"]
impl crate::ResetValue for super::HDRVE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "High Current Drive Capability of PTB4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTB4_A {
    #[doc = "0: PTB4 is disabled to offer high current drive capability."]
    _0 = 0,
    #[doc = "1: PTB4 is enabled to offer high current drive capability."]
    _1 = 1,
}
impl From<PTB4_A> for bool {
    #[inline(always)]
    fn from(variant: PTB4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTB4`"]
pub type PTB4_R = crate::R<bool, PTB4_A>;
impl PTB4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTB4_A {
        match self.bits {
            false => PTB4_A::_0,
            true => PTB4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTB4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTB4_A::_1
    }
}
#[doc = "Write proxy for field `PTB4`"]
pub struct PTB4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTB4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTB4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PTB4 is disabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTB4_A::_0)
    }
    #[doc = "PTB4 is enabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTB4_A::_1)
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
#[doc = "High Current Drive Capability of PTB5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTB5_A {
    #[doc = "0: PTB5 is disabled to offer high current drive capability."]
    _0 = 0,
    #[doc = "1: PTB5 is enabled to offer high current drive capability."]
    _1 = 1,
}
impl From<PTB5_A> for bool {
    #[inline(always)]
    fn from(variant: PTB5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTB5`"]
pub type PTB5_R = crate::R<bool, PTB5_A>;
impl PTB5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTB5_A {
        match self.bits {
            false => PTB5_A::_0,
            true => PTB5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTB5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTB5_A::_1
    }
}
#[doc = "Write proxy for field `PTB5`"]
pub struct PTB5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTB5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTB5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PTB5 is disabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTB5_A::_0)
    }
    #[doc = "PTB5 is enabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTB5_A::_1)
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
#[doc = "High Current Drive Capability of PTD0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTD0_A {
    #[doc = "0: PTD0 is disabled to offer high current drive capability."]
    _0 = 0,
    #[doc = "1: PTD0 is enabled to offer high current drive capability."]
    _1 = 1,
}
impl From<PTD0_A> for bool {
    #[inline(always)]
    fn from(variant: PTD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTD0`"]
pub type PTD0_R = crate::R<bool, PTD0_A>;
impl PTD0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTD0_A {
        match self.bits {
            false => PTD0_A::_0,
            true => PTD0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTD0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTD0_A::_1
    }
}
#[doc = "Write proxy for field `PTD0`"]
pub struct PTD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTD0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PTD0 is disabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTD0_A::_0)
    }
    #[doc = "PTD0 is enabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTD0_A::_1)
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
#[doc = "High Current Drive Capability of PTD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTD1_A {
    #[doc = "0: PTD1 is disabled to offer high current drive capability."]
    _0 = 0,
    #[doc = "1: PTD1 is enable to offer high current drive capability."]
    _1 = 1,
}
impl From<PTD1_A> for bool {
    #[inline(always)]
    fn from(variant: PTD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTD1`"]
pub type PTD1_R = crate::R<bool, PTD1_A>;
impl PTD1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTD1_A {
        match self.bits {
            false => PTD1_A::_0,
            true => PTD1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTD1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTD1_A::_1
    }
}
#[doc = "Write proxy for field `PTD1`"]
pub struct PTD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTD1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PTD1 is disabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTD1_A::_0)
    }
    #[doc = "PTD1 is enable to offer high current drive capability."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTD1_A::_1)
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
#[doc = "High Current Drive Capability of PTE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTE0_A {
    #[doc = "0: PTE0 is disabled to offer high current drive capability."]
    _0 = 0,
    #[doc = "1: PTE0 is enable to offer high current drive capability."]
    _1 = 1,
}
impl From<PTE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTE0`"]
pub type PTE0_R = crate::R<bool, PTE0_A>;
impl PTE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTE0_A {
        match self.bits {
            false => PTE0_A::_0,
            true => PTE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTE0_A::_1
    }
}
#[doc = "Write proxy for field `PTE0`"]
pub struct PTE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PTE0 is disabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTE0_A::_0)
    }
    #[doc = "PTE0 is enable to offer high current drive capability."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTE0_A::_1)
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
#[doc = "High Current Drive Capability of PTE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTE1_A {
    #[doc = "0: PTE1 is disabled to offer high current drive capability."]
    _0 = 0,
    #[doc = "1: PTE1 is enabled to offer high current drive capability."]
    _1 = 1,
}
impl From<PTE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTE1`"]
pub type PTE1_R = crate::R<bool, PTE1_A>;
impl PTE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTE1_A {
        match self.bits {
            false => PTE1_A::_0,
            true => PTE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTE1_A::_1
    }
}
#[doc = "Write proxy for field `PTE1`"]
pub struct PTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PTE1 is disabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTE1_A::_0)
    }
    #[doc = "PTE1 is enabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTE1_A::_1)
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
#[doc = "High Current Drive Capability of PTH0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTH0_A {
    #[doc = "0: PTH0 is disabled to offer high current drive capability."]
    _0 = 0,
    #[doc = "1: PTH0 is enabled to offer high current drive capability."]
    _1 = 1,
}
impl From<PTH0_A> for bool {
    #[inline(always)]
    fn from(variant: PTH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTH0`"]
pub type PTH0_R = crate::R<bool, PTH0_A>;
impl PTH0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTH0_A {
        match self.bits {
            false => PTH0_A::_0,
            true => PTH0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTH0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTH0_A::_1
    }
}
#[doc = "Write proxy for field `PTH0`"]
pub struct PTH0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTH0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PTH0 is disabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTH0_A::_0)
    }
    #[doc = "PTH0 is enabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTH0_A::_1)
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
#[doc = "High Current Drive Capability of PTH1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTH1_A {
    #[doc = "0: PTH1 is disabled to offer high current drive capability."]
    _0 = 0,
    #[doc = "1: PTH1 is enabled to offer high current drive capability."]
    _1 = 1,
}
impl From<PTH1_A> for bool {
    #[inline(always)]
    fn from(variant: PTH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTH1`"]
pub type PTH1_R = crate::R<bool, PTH1_A>;
impl PTH1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTH1_A {
        match self.bits {
            false => PTH1_A::_0,
            true => PTH1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTH1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTH1_A::_1
    }
}
#[doc = "Write proxy for field `PTH1`"]
pub struct PTH1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTH1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PTH1 is disabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTH1_A::_0)
    }
    #[doc = "PTH1 is enabled to offer high current drive capability."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTH1_A::_1)
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
impl R {
    #[doc = "Bit 0 - High Current Drive Capability of PTB4"]
    #[inline(always)]
    pub fn ptb4(&self) -> PTB4_R {
        PTB4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - High Current Drive Capability of PTB5"]
    #[inline(always)]
    pub fn ptb5(&self) -> PTB5_R {
        PTB5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - High Current Drive Capability of PTD0"]
    #[inline(always)]
    pub fn ptd0(&self) -> PTD0_R {
        PTD0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - High Current Drive Capability of PTD1"]
    #[inline(always)]
    pub fn ptd1(&self) -> PTD1_R {
        PTD1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - High Current Drive Capability of PTE0"]
    #[inline(always)]
    pub fn pte0(&self) -> PTE0_R {
        PTE0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - High Current Drive Capability of PTE1"]
    #[inline(always)]
    pub fn pte1(&self) -> PTE1_R {
        PTE1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - High Current Drive Capability of PTH0"]
    #[inline(always)]
    pub fn pth0(&self) -> PTH0_R {
        PTH0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - High Current Drive Capability of PTH1"]
    #[inline(always)]
    pub fn pth1(&self) -> PTH1_R {
        PTH1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High Current Drive Capability of PTB4"]
    #[inline(always)]
    pub fn ptb4(&mut self) -> PTB4_W {
        PTB4_W { w: self }
    }
    #[doc = "Bit 1 - High Current Drive Capability of PTB5"]
    #[inline(always)]
    pub fn ptb5(&mut self) -> PTB5_W {
        PTB5_W { w: self }
    }
    #[doc = "Bit 2 - High Current Drive Capability of PTD0"]
    #[inline(always)]
    pub fn ptd0(&mut self) -> PTD0_W {
        PTD0_W { w: self }
    }
    #[doc = "Bit 3 - High Current Drive Capability of PTD1"]
    #[inline(always)]
    pub fn ptd1(&mut self) -> PTD1_W {
        PTD1_W { w: self }
    }
    #[doc = "Bit 4 - High Current Drive Capability of PTE0"]
    #[inline(always)]
    pub fn pte0(&mut self) -> PTE0_W {
        PTE0_W { w: self }
    }
    #[doc = "Bit 5 - High Current Drive Capability of PTE1"]
    #[inline(always)]
    pub fn pte1(&mut self) -> PTE1_W {
        PTE1_W { w: self }
    }
    #[doc = "Bit 6 - High Current Drive Capability of PTH0"]
    #[inline(always)]
    pub fn pth0(&mut self) -> PTH0_W {
        PTH0_W { w: self }
    }
    #[doc = "Bit 7 - High Current Drive Capability of PTH1"]
    #[inline(always)]
    pub fn pth1(&mut self) -> PTH1_W {
        PTH1_W { w: self }
    }
}

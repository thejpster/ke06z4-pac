#[doc = "Reader of register C%sSC"]
pub type R = crate::R<u32, super::CSC>;
#[doc = "Writer for register C%sSC"]
pub type W = crate::W<u32, super::CSC>;
#[doc = "Register C%sSC `reset()`'s with value 0"]
impl crate::ResetValue for super::CSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ELSA`"]
pub type ELSA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELSA`"]
pub struct ELSA_W<'a> {
    w: &'a mut W,
}
impl<'a> ELSA_W<'a> {
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
#[doc = "Reader of field `ELSB`"]
pub type ELSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELSB`"]
pub struct ELSB_W<'a> {
    w: &'a mut W,
}
impl<'a> ELSB_W<'a> {
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
#[doc = "Reader of field `MSA`"]
pub type MSA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSA`"]
pub struct MSA_W<'a> {
    w: &'a mut W,
}
impl<'a> MSA_W<'a> {
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
#[doc = "Reader of field `MSB`"]
pub type MSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSB`"]
pub struct MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MSB_W<'a> {
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
#[doc = "Channel Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHIE_A {
    #[doc = "0: Disable channel interrupts. Use software polling."]
    _0 = 0,
    #[doc = "1: Enable channel interrupts."]
    _1 = 1,
}
impl From<CHIE_A> for bool {
    #[inline(always)]
    fn from(variant: CHIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHIE`"]
pub type CHIE_R = crate::R<bool, CHIE_A>;
impl CHIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIE_A {
        match self.bits {
            false => CHIE_A::_0,
            true => CHIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHIE_A::_1
    }
}
#[doc = "Write proxy for field `CHIE`"]
pub struct CHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable channel interrupts. Use software polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHIE_A::_0)
    }
    #[doc = "Enable channel interrupts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHIE_A::_1)
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
#[doc = "Channel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHF_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CHF_A> for bool {
    #[inline(always)]
    fn from(variant: CHF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHF`"]
pub type CHF_R = crate::R<bool, CHF_A>;
impl CHF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHF_A {
        match self.bits {
            false => CHF_A::_0,
            true => CHF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHF_A::_1
    }
}
#[doc = "Write proxy for field `CHF`"]
pub struct CHF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHF_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHF_A::_1)
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
    #[doc = "Bit 2 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsa(&self) -> ELSA_R {
        ELSA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsb(&self) -> ELSB_R {
        ELSB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel Mode Select"]
    #[inline(always)]
    pub fn msa(&self) -> MSA_R {
        MSA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel Mode Select"]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel Interrupt Enable"]
    #[inline(always)]
    pub fn chie(&self) -> CHIE_R {
        CHIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel Flag"]
    #[inline(always)]
    pub fn chf(&self) -> CHF_R {
        CHF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsa(&mut self) -> ELSA_W {
        ELSA_W { w: self }
    }
    #[doc = "Bit 3 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsb(&mut self) -> ELSB_W {
        ELSB_W { w: self }
    }
    #[doc = "Bit 4 - Channel Mode Select"]
    #[inline(always)]
    pub fn msa(&mut self) -> MSA_W {
        MSA_W { w: self }
    }
    #[doc = "Bit 5 - Channel Mode Select"]
    #[inline(always)]
    pub fn msb(&mut self) -> MSB_W {
        MSB_W { w: self }
    }
    #[doc = "Bit 6 - Channel Interrupt Enable"]
    #[inline(always)]
    pub fn chie(&mut self) -> CHIE_W {
        CHIE_W { w: self }
    }
    #[doc = "Bit 7 - Channel Flag"]
    #[inline(always)]
    pub fn chf(&mut self) -> CHF_W {
        CHF_W { w: self }
    }
}

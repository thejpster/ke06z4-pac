#[doc = "Reader of register SC"]
pub type R = crate::R<u32, super::SC>;
#[doc = "Writer for register SC"]
pub type W = crate::W<u32, super::SC>;
#[doc = "Register SC `reset()`'s with value 0"]
impl crate::ResetValue for super::SC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "KBI Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KBMOD_A {
    #[doc = "0: Keyboard detects edges only."]
    _0 = 0,
    #[doc = "1: Keyboard detects both edges and levels."]
    _1 = 1,
}
impl From<KBMOD_A> for bool {
    #[inline(always)]
    fn from(variant: KBMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KBMOD`"]
pub type KBMOD_R = crate::R<bool, KBMOD_A>;
impl KBMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KBMOD_A {
        match self.bits {
            false => KBMOD_A::_0,
            true => KBMOD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KBMOD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KBMOD_A::_1
    }
}
#[doc = "Write proxy for field `KBMOD`"]
pub struct KBMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> KBMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBMOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Keyboard detects edges only."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBMOD_A::_0)
    }
    #[doc = "Keyboard detects both edges and levels."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBMOD_A::_1)
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
#[doc = "KBI Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KBIE_A {
    #[doc = "0: KBI interrupt not enabled."]
    _0 = 0,
    #[doc = "1: KBI interrupt enabled."]
    _1 = 1,
}
impl From<KBIE_A> for bool {
    #[inline(always)]
    fn from(variant: KBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KBIE`"]
pub type KBIE_R = crate::R<bool, KBIE_A>;
impl KBIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KBIE_A {
        match self.bits {
            false => KBIE_A::_0,
            true => KBIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KBIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KBIE_A::_1
    }
}
#[doc = "Write proxy for field `KBIE`"]
pub struct KBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> KBIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "KBI interrupt not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBIE_A::_0)
    }
    #[doc = "KBI interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBIE_A::_1)
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
#[doc = "Write proxy for field `KBACK`"]
pub struct KBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> KBACK_W<'a> {
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
#[doc = "KBI Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KBF_A {
    #[doc = "0: KBI interrupt request not detected."]
    _0 = 0,
    #[doc = "1: KBI interrupt request detected."]
    _1 = 1,
}
impl From<KBF_A> for bool {
    #[inline(always)]
    fn from(variant: KBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KBF`"]
pub type KBF_R = crate::R<bool, KBF_A>;
impl KBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KBF_A {
        match self.bits {
            false => KBF_A::_0,
            true => KBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KBF_A::_1
    }
}
#[doc = "Real KBI_SP register enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KBSPEN_A {
    #[doc = "0: The real time value of Keyboard source pin to be read."]
    _0 = 0,
    #[doc = "1: The latched value in KBxSP register while interrupt flag occur to be read."]
    _1 = 1,
}
impl From<KBSPEN_A> for bool {
    #[inline(always)]
    fn from(variant: KBSPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KBSPEN`"]
pub type KBSPEN_R = crate::R<bool, KBSPEN_A>;
impl KBSPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KBSPEN_A {
        match self.bits {
            false => KBSPEN_A::_0,
            true => KBSPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KBSPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KBSPEN_A::_1
    }
}
#[doc = "Write proxy for field `KBSPEN`"]
pub struct KBSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBSPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBSPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The real time value of Keyboard source pin to be read."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBSPEN_A::_0)
    }
    #[doc = "The latched value in KBxSP register while interrupt flag occur to be read."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBSPEN_A::_1)
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
#[doc = "Reader of field `RSTKBSP`"]
pub type RSTKBSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTKBSP`"]
pub struct RSTKBSP_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTKBSP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - KBI Detection Mode"]
    #[inline(always)]
    pub fn kbmod(&self) -> KBMOD_R {
        KBMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - KBI Interrupt Enable"]
    #[inline(always)]
    pub fn kbie(&self) -> KBIE_R {
        KBIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - KBI Interrupt Flag"]
    #[inline(always)]
    pub fn kbf(&self) -> KBF_R {
        KBF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real KBI_SP register enable"]
    #[inline(always)]
    pub fn kbspen(&self) -> KBSPEN_R {
        KBSPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reset KBI_SP registe"]
    #[inline(always)]
    pub fn rstkbsp(&self) -> RSTKBSP_R {
        RSTKBSP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - KBI Detection Mode"]
    #[inline(always)]
    pub fn kbmod(&mut self) -> KBMOD_W {
        KBMOD_W { w: self }
    }
    #[doc = "Bit 1 - KBI Interrupt Enable"]
    #[inline(always)]
    pub fn kbie(&mut self) -> KBIE_W {
        KBIE_W { w: self }
    }
    #[doc = "Bit 2 - KBI Acknowledge"]
    #[inline(always)]
    pub fn kback(&mut self) -> KBACK_W {
        KBACK_W { w: self }
    }
    #[doc = "Bit 4 - Real KBI_SP register enable"]
    #[inline(always)]
    pub fn kbspen(&mut self) -> KBSPEN_W {
        KBSPEN_W { w: self }
    }
    #[doc = "Bit 5 - Reset KBI_SP registe"]
    #[inline(always)]
    pub fn rstkbsp(&mut self) -> RSTKBSP_W {
        RSTKBSP_W { w: self }
    }
}

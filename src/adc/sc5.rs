#[doc = "Reader of register SC5"]
pub type R = crate::R<u32, super::SC5>;
#[doc = "Writer for register SC5"]
pub type W = crate::W<u32, super::SC5>;
#[doc = "Register SC5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SC5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Hardware Trigger Mask Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTRGMASKSEL_A {
    #[doc = "0: Hardware trigger mask with HTRGMASKE."]
    _0 = 0,
    #[doc = "1: Hardware trigger mask automatically when data fifo is not empty."]
    _1 = 1,
}
impl From<HTRGMASKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HTRGMASKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HTRGMASKSEL`"]
pub type HTRGMASKSEL_R = crate::R<bool, HTRGMASKSEL_A>;
impl HTRGMASKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTRGMASKSEL_A {
        match self.bits {
            false => HTRGMASKSEL_A::_0,
            true => HTRGMASKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTRGMASKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTRGMASKSEL_A::_1
    }
}
#[doc = "Write proxy for field `HTRGMASKSEL`"]
pub struct HTRGMASKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HTRGMASKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTRGMASKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware trigger mask with HTRGMASKE."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HTRGMASKSEL_A::_0)
    }
    #[doc = "Hardware trigger mask automatically when data fifo is not empty."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HTRGMASKSEL_A::_1)
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
#[doc = "Hardware Trigger Mask Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTRGMASKE_A {
    #[doc = "0: Hardware trigger mask disable."]
    _0 = 0,
    #[doc = "1: Hardware trigger mask enable and hardware trigger cannot trigger ADC conversion.."]
    _1 = 1,
}
impl From<HTRGMASKE_A> for bool {
    #[inline(always)]
    fn from(variant: HTRGMASKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HTRGMASKE`"]
pub type HTRGMASKE_R = crate::R<bool, HTRGMASKE_A>;
impl HTRGMASKE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTRGMASKE_A {
        match self.bits {
            false => HTRGMASKE_A::_0,
            true => HTRGMASKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTRGMASKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTRGMASKE_A::_1
    }
}
#[doc = "Write proxy for field `HTRGMASKE`"]
pub struct HTRGMASKE_W<'a> {
    w: &'a mut W,
}
impl<'a> HTRGMASKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTRGMASKE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware trigger mask disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HTRGMASKE_A::_0)
    }
    #[doc = "Hardware trigger mask enable and hardware trigger cannot trigger ADC conversion.."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HTRGMASKE_A::_1)
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
impl R {
    #[doc = "Bit 0 - Hardware Trigger Mask Mode Select"]
    #[inline(always)]
    pub fn htrgmasksel(&self) -> HTRGMASKSEL_R {
        HTRGMASKSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hardware Trigger Mask Enable"]
    #[inline(always)]
    pub fn htrgmaske(&self) -> HTRGMASKE_R {
        HTRGMASKE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware Trigger Mask Mode Select"]
    #[inline(always)]
    pub fn htrgmasksel(&mut self) -> HTRGMASKSEL_W {
        HTRGMASKSEL_W { w: self }
    }
    #[doc = "Bit 1 - Hardware Trigger Mask Enable"]
    #[inline(always)]
    pub fn htrgmaske(&mut self) -> HTRGMASKE_W {
        HTRGMASKE_W { w: self }
    }
}

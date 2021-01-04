#[doc = "Reader of register CANMISC"]
pub type R = crate::R<u8, super::CANMISC>;
#[doc = "Writer for register CANMISC"]
pub type W = crate::W<u8, super::CANMISC>;
#[doc = "Register CANMISC `reset()`'s with value 0"]
impl crate::ResetValue for super::CANMISC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Bus-off State Hold Until User Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOHOLD_A {
    #[doc = "0: Module is not bus-off or recovery has been requested by user in bus-off state."]
    _0 = 0,
    #[doc = "1: Module is bus-off and holds this state until user request."]
    _1 = 1,
}
impl From<BOHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: BOHOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOHOLD`"]
pub type BOHOLD_R = crate::R<bool, BOHOLD_A>;
impl BOHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOHOLD_A {
        match self.bits {
            false => BOHOLD_A::_0,
            true => BOHOLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOHOLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOHOLD_A::_1
    }
}
#[doc = "Write proxy for field `BOHOLD`"]
pub struct BOHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOHOLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Module is not bus-off or recovery has been requested by user in bus-off state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOHOLD_A::_0)
    }
    #[doc = "Module is bus-off and holds this state until user request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOHOLD_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bus-off State Hold Until User Request"]
    #[inline(always)]
    pub fn bohold(&self) -> BOHOLD_R {
        BOHOLD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus-off State Hold Until User Request"]
    #[inline(always)]
    pub fn bohold(&mut self) -> BOHOLD_W {
        BOHOLD_W { w: self }
    }
}

#[doc = "Reader of register CANTIER"]
pub type R = crate::R<u8, super::CANTIER>;
#[doc = "Writer for register CANTIER"]
pub type W = crate::W<u8, super::CANTIER>;
#[doc = "Register CANTIER `reset()`'s with value 0"]
impl crate::ResetValue for super::CANTIER {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmitter Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXEIE_A {
    #[doc = "0: No interrupt request is generated from this event."]
    _0 = 0,
    #[doc = "1: A transmitter empty (transmit buffer available for transmission) event causes a transmitter empty interrupt request."]
    _1 = 1,
}
impl From<TXEIE_A> for u8 {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXEIE`"]
pub type TXEIE_R = crate::R<u8, TXEIE_A>;
impl TXEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXEIE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXEIE_A::_0),
            1 => Val(TXEIE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXEIE_A::_1
    }
}
#[doc = "Write proxy for field `TXEIE`"]
pub struct TXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEIE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt request is generated from this event."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEIE_A::_0)
    }
    #[doc = "A transmitter empty (transmit buffer available for transmission) event causes a transmitter empty interrupt request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXEIE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmitter Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmitter Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W {
        TXEIE_W { w: self }
    }
}

#[doc = "Reader of register CANTFLG"]
pub type R = crate::R<u8, super::CANTFLG>;
#[doc = "Writer for register CANTFLG"]
pub type W = crate::W<u8, super::CANTFLG>;
#[doc = "Register CANTFLG `reset()`'s with value 0x07"]
impl crate::ResetValue for super::CANTFLG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Transmitter Buffer Empty\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXE_A {
    #[doc = "0: The associated message buffer is full (loaded with a message due for transmission)."]
    _0 = 0,
    #[doc = "1: The associated message buffer is empty (not scheduled)."]
    _1 = 1,
}
impl From<TXE_A> for u8 {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<u8, TXE_A>;
impl TXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXE_A::_0),
            1 => Val(TXE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXE_A::_1
    }
}
#[doc = "Write proxy for field `TXE`"]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The associated message buffer is full (loaded with a message due for transmission)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXE_A::_0)
    }
    #[doc = "The associated message buffer is empty (not scheduled)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmitter Buffer Empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmitter Buffer Empty"]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
}

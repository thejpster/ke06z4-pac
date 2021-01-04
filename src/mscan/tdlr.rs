#[doc = "Reader of register TDLR"]
pub type R = crate::R<u8, super::TDLR>;
#[doc = "Writer for register TDLR"]
pub type W = crate::W<u8, super::TDLR>;
#[doc = "Register TDLR `reset()`'s with value 0"]
impl crate::ResetValue for super::TDLR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Data Length Code Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TDLC_A {
    #[doc = "0: 0"]
    _0000 = 0,
    #[doc = "1: 1"]
    _0001 = 1,
    #[doc = "2: 2"]
    _0010 = 2,
    #[doc = "3: 3"]
    _0011 = 3,
    #[doc = "4: 4"]
    _0100 = 4,
    #[doc = "5: 5"]
    _0101 = 5,
    #[doc = "6: 6"]
    _0110 = 6,
    #[doc = "7: 7"]
    _0111 = 7,
    #[doc = "8: 8"]
    _1000 = 8,
}
impl From<TDLC_A> for u8 {
    #[inline(always)]
    fn from(variant: TDLC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TDLC`"]
pub type TDLC_R = crate::R<u8, TDLC_A>;
impl TDLC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TDLC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TDLC_A::_0000),
            1 => Val(TDLC_A::_0001),
            2 => Val(TDLC_A::_0010),
            3 => Val(TDLC_A::_0011),
            4 => Val(TDLC_A::_0100),
            5 => Val(TDLC_A::_0101),
            6 => Val(TDLC_A::_0110),
            7 => Val(TDLC_A::_0111),
            8 => Val(TDLC_A::_1000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TDLC_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TDLC_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == TDLC_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == TDLC_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == TDLC_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == TDLC_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == TDLC_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == TDLC_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == TDLC_A::_1000
    }
}
#[doc = "Write proxy for field `TDLC`"]
pub struct TDLC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDLC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDLC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TDLC_A::_0000)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TDLC_A::_0001)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TDLC_A::_0010)
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TDLC_A::_0011)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TDLC_A::_0100)
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TDLC_A::_0101)
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TDLC_A::_0110)
    }
    #[doc = "7"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TDLC_A::_0111)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(TDLC_A::_1000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Length Code Bits"]
    #[inline(always)]
    pub fn tdlc(&self) -> TDLC_R {
        TDLC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Length Code Bits"]
    #[inline(always)]
    pub fn tdlc(&mut self) -> TDLC_W {
        TDLC_W { w: self }
    }
}

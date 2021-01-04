#[doc = "Reader of register RDLR"]
pub type R = crate::R<u8, super::RDLR>;
#[doc = "Data Length Code Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDLC_A {
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
impl From<RDLC_A> for u8 {
    #[inline(always)]
    fn from(variant: RDLC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RDLC`"]
pub type RDLC_R = crate::R<u8, RDLC_A>;
impl RDLC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RDLC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RDLC_A::_0000),
            1 => Val(RDLC_A::_0001),
            2 => Val(RDLC_A::_0010),
            3 => Val(RDLC_A::_0011),
            4 => Val(RDLC_A::_0100),
            5 => Val(RDLC_A::_0101),
            6 => Val(RDLC_A::_0110),
            7 => Val(RDLC_A::_0111),
            8 => Val(RDLC_A::_1000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == RDLC_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == RDLC_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == RDLC_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == RDLC_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == RDLC_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == RDLC_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == RDLC_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == RDLC_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == RDLC_A::_1000
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Length Code Bits"]
    #[inline(always)]
    pub fn rdlc(&self) -> RDLC_R {
        RDLC_R::new((self.bits & 0x0f) as u8)
    }
}

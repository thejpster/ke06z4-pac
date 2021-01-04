#[doc = "Reader of register CANTAAK"]
pub type R = crate::R<u8, super::CANTAAK>;
#[doc = "Abort Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ABTAK_A {
    #[doc = "0: The message was not aborted."]
    _0 = 0,
    #[doc = "1: The message was aborted."]
    _1 = 1,
}
impl From<ABTAK_A> for u8 {
    #[inline(always)]
    fn from(variant: ABTAK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ABTAK`"]
pub type ABTAK_R = crate::R<u8, ABTAK_A>;
impl ABTAK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ABTAK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ABTAK_A::_0),
            1 => Val(ABTAK_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABTAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABTAK_A::_1
    }
}
impl R {
    #[doc = "Bits 0:2 - Abort Acknowledge"]
    #[inline(always)]
    pub fn abtak(&self) -> ABTAK_R {
        ABTAK_R::new((self.bits & 0x07) as u8)
    }
}

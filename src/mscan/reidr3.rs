#[doc = "Reader of register REIDR3"]
pub type R = crate::R<u8, super::REIDR3>;
#[doc = "Remote Transmission Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RERTR_A {
    #[doc = "0: Data frame."]
    _0 = 0,
    #[doc = "1: Remote frame."]
    _1 = 1,
}
impl From<RERTR_A> for bool {
    #[inline(always)]
    fn from(variant: RERTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RERTR`"]
pub type RERTR_R = crate::R<bool, RERTR_A>;
impl RERTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RERTR_A {
        match self.bits {
            false => RERTR_A::_0,
            true => RERTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RERTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RERTR_A::_1
    }
}
#[doc = "Reader of field `REID6_REID0`"]
pub type REID6_REID0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Remote Transmission Request"]
    #[inline(always)]
    pub fn rertr(&self) -> RERTR_R {
        RERTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - Extended Format Identifier 6-0"]
    #[inline(always)]
    pub fn reid6_reid0(&self) -> REID6_REID0_R {
        REID6_REID0_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}

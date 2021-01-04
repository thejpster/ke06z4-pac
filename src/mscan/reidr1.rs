#[doc = "Reader of register REIDR1"]
pub type R = crate::R<u8, super::REIDR1>;
#[doc = "Reader of field `REID17_REID15`"]
pub type REID17_REID15_R = crate::R<u8, u8>;
#[doc = "ID Extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REIDE_A {
    #[doc = "0: Standard format (11 bit)."]
    _0 = 0,
    #[doc = "1: Extended format (29 bit)."]
    _1 = 1,
}
impl From<REIDE_A> for bool {
    #[inline(always)]
    fn from(variant: REIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REIDE`"]
pub type REIDE_R = crate::R<bool, REIDE_A>;
impl REIDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REIDE_A {
        match self.bits {
            false => REIDE_A::_0,
            true => REIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REIDE_A::_1
    }
}
#[doc = "Reader of field `RSRR`"]
pub type RSRR_R = crate::R<bool, bool>;
#[doc = "Reader of field `REID20_REID18`"]
pub type REID20_REID18_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Extended Format Identifier 17-15"]
    #[inline(always)]
    pub fn reid17_reid15(&self) -> REID17_REID15_R {
        REID17_REID15_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - ID Extended"]
    #[inline(always)]
    pub fn reide(&self) -> REIDE_R {
        REIDE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Substitute Remote Request"]
    #[inline(always)]
    pub fn rsrr(&self) -> RSRR_R {
        RSRR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Extended Format Identifier 20-18"]
    #[inline(always)]
    pub fn reid20_reid18(&self) -> REID20_REID18_R {
        REID20_REID18_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}

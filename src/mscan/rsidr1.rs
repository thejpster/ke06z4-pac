#[doc = "Reader of register RSIDR1"]
pub type R = crate::R<u8, super::RSIDR1>;
#[doc = "ID Extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIDE_A {
    #[doc = "0: Standard format (11 bit)."]
    _0 = 0,
    #[doc = "1: Extended format (29 bit)."]
    _1 = 1,
}
impl From<RSIDE_A> for bool {
    #[inline(always)]
    fn from(variant: RSIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSIDE`"]
pub type RSIDE_R = crate::R<bool, RSIDE_A>;
impl RSIDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSIDE_A {
        match self.bits {
            false => RSIDE_A::_0,
            true => RSIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSIDE_A::_1
    }
}
#[doc = "Remote Transmission Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSRTR_A {
    #[doc = "0: Data frame."]
    _0 = 0,
    #[doc = "1: Remote frame."]
    _1 = 1,
}
impl From<RSRTR_A> for bool {
    #[inline(always)]
    fn from(variant: RSRTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSRTR`"]
pub type RSRTR_R = crate::R<bool, RSRTR_A>;
impl RSRTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSRTR_A {
        match self.bits {
            false => RSRTR_A::_0,
            true => RSRTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSRTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSRTR_A::_1
    }
}
#[doc = "Reader of field `RSID2_RSID0`"]
pub type RSID2_RSID0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 3 - ID Extended"]
    #[inline(always)]
    pub fn rside(&self) -> RSIDE_R {
        RSIDE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Remote Transmission Request"]
    #[inline(always)]
    pub fn rsrtr(&self) -> RSRTR_R {
        RSRTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Standard Format Identifier 2-0"]
    #[inline(always)]
    pub fn rsid2_rsid0(&self) -> RSID2_RSID0_R {
        RSID2_RSID0_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}

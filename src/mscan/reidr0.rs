#[doc = "Reader of register REIDR0"]
pub type R = crate::R<u8, super::REIDR0>;
#[doc = "Reader of field `REID28_REID21`"]
pub type REID28_REID21_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Extended Format Identifier"]
    #[inline(always)]
    pub fn reid28_reid21(&self) -> REID28_REID21_R {
        REID28_REID21_R::new((self.bits & 0xff) as u8)
    }
}

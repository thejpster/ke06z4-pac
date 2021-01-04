#[doc = "Reader of register RSIDR0"]
pub type R = crate::R<u8, super::RSIDR0>;
#[doc = "Reader of field `RSID10_RSID3`"]
pub type RSID10_RSID3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Standard Format Identifier"]
    #[inline(always)]
    pub fn rsid10_rsid3(&self) -> RSID10_RSID3_R {
        RSID10_RSID3_R::new((self.bits & 0xff) as u8)
    }
}

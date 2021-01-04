#[doc = "Reader of register CANTXERR"]
pub type R = crate::R<u8, super::CANTXERR>;
#[doc = "Reader of field `TXERR`"]
pub type TXERR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 0xff) as u8)
    }
}

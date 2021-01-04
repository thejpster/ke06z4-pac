#[doc = "Reader of register CANRXERR"]
pub type R = crate::R<u8, super::CANRXERR>;
#[doc = "Reader of field `RXERR`"]
pub type RXERR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive Error Counter"]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new((self.bits & 0xff) as u8)
    }
}

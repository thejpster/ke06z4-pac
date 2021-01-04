#[doc = "Reader of register REDSR%s"]
pub type R = crate::R<u8, super::REDSR>;
#[doc = "Reader of field `RDB`"]
pub type RDB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Data Bits"]
    #[inline(always)]
    pub fn rdb(&self) -> RDB_R {
        RDB_R::new((self.bits & 0xff) as u8)
    }
}

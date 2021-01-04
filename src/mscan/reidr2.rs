#[doc = "Reader of register REIDR2"]
pub type R = crate::R<u8, super::REIDR2>;
#[doc = "Reader of field `REID14_REID7`"]
pub type REID14_REID7_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Extended Format Identifier 14-7"]
    #[inline(always)]
    pub fn reid14_reid7(&self) -> REID14_REID7_R {
        REID14_REID7_R::new((self.bits & 0xff) as u8)
    }
}

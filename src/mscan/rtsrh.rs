#[doc = "Reader of register RTSRH"]
pub type R = crate::R<u8, super::RTSRH>;
#[doc = "Reader of field `RTS`"]
pub type RTS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Time Stamp"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new((self.bits & 0xff) as u8)
    }
}

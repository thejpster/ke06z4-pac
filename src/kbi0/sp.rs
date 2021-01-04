#[doc = "Reader of register SP"]
pub type R = crate::R<u32, super::SP>;
#[doc = "Reader of field `SP`"]
pub type SP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - KBI Source Pin"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

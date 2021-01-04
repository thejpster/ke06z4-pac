#[doc = "Reader of register FOPT"]
pub type R = crate::R<u8, super::FOPT>;
#[doc = "Reader of field `NV`"]
pub type NV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Nonvolatile Bits"]
    #[inline(always)]
    pub fn nv(&self) -> NV_R {
        NV_R::new((self.bits & 0xff) as u8)
    }
}

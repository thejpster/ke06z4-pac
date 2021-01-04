#[doc = "Reader of register R"]
pub type R = crate::R<u32, super::R>;
#[doc = "Reader of field `ADR`"]
pub type ADR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Conversion Result"]
    #[inline(always)]
    pub fn adr(&self) -> ADR_R {
        ADR_R::new((self.bits & 0x0fff) as u16)
    }
}

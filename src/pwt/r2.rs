#[doc = "Reader of register R2"]
pub type R = crate::R<u32, super::R2>;
#[doc = "Reader of field `NPW`"]
pub type NPW_R = crate::R<u16, u16>;
#[doc = "Reader of field `PWTC`"]
pub type PWTC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Negative Pulse Width. It is suggested to use half-word (16-bit) or word (32-bit) to read out this value."]
    #[inline(always)]
    pub fn npw(&self) -> NPW_R {
        NPW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PWT Counter. It is suggested to use half-word (16-bit) or word (32-bit) to read out this value."]
    #[inline(always)]
    pub fn pwtc(&self) -> PWTC_R {
        PWTC_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}

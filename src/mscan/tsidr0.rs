#[doc = "Reader of register TSIDR0"]
pub type R = crate::R<u8, super::TSIDR0>;
#[doc = "Writer for register TSIDR0"]
pub type W = crate::W<u8, super::TSIDR0>;
#[doc = "Register TSIDR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TSIDR0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSID10_TSID3`"]
pub type TSID10_TSID3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSID10_TSID3`"]
pub struct TSID10_TSID3_W<'a> {
    w: &'a mut W,
}
impl<'a> TSID10_TSID3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Standard Format Identifier"]
    #[inline(always)]
    pub fn tsid10_tsid3(&self) -> TSID10_TSID3_R {
        TSID10_TSID3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Standard Format Identifier"]
    #[inline(always)]
    pub fn tsid10_tsid3(&mut self) -> TSID10_TSID3_W {
        TSID10_TSID3_W { w: self }
    }
}

#[doc = "Reader of register TEIDR0"]
pub type R = crate::R<u8, super::TEIDR0>;
#[doc = "Writer for register TEIDR0"]
pub type W = crate::W<u8, super::TEIDR0>;
#[doc = "Register TEIDR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TEIDR0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TEID28_TEID21`"]
pub type TEID28_TEID21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEID28_TEID21`"]
pub struct TEID28_TEID21_W<'a> {
    w: &'a mut W,
}
impl<'a> TEID28_TEID21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Extended Format Identifier"]
    #[inline(always)]
    pub fn teid28_teid21(&self) -> TEID28_TEID21_R {
        TEID28_TEID21_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Extended Format Identifier"]
    #[inline(always)]
    pub fn teid28_teid21(&mut self) -> TEID28_TEID21_W {
        TEID28_TEID21_W { w: self }
    }
}

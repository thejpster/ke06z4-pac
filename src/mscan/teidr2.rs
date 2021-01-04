#[doc = "Reader of register TEIDR2"]
pub type R = crate::R<u8, super::TEIDR2>;
#[doc = "Writer for register TEIDR2"]
pub type W = crate::W<u8, super::TEIDR2>;
#[doc = "Register TEIDR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TEIDR2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TEID14_TEID7`"]
pub type TEID14_TEID7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEID14_TEID7`"]
pub struct TEID14_TEID7_W<'a> {
    w: &'a mut W,
}
impl<'a> TEID14_TEID7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Extended Format Identifier 14-7"]
    #[inline(always)]
    pub fn teid14_teid7(&self) -> TEID14_TEID7_R {
        TEID14_TEID7_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Extended Format Identifier 14-7"]
    #[inline(always)]
    pub fn teid14_teid7(&mut self) -> TEID14_TEID7_W {
        TEID14_TEID7_W { w: self }
    }
}

#[doc = "Reader of register CANIDAR4"]
pub type R = crate::R<u8, super::CANIDAR4>;
#[doc = "Writer for register CANIDAR4"]
pub type W = crate::W<u8, super::CANIDAR4>;
#[doc = "Register CANIDAR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CANIDAR4 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AC`"]
pub type AC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AC`"]
pub struct AC_W<'a> {
    w: &'a mut W,
}
impl<'a> AC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Acceptance Code Bits"]
    #[inline(always)]
    pub fn ac(&self) -> AC_R {
        AC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance Code Bits"]
    #[inline(always)]
    pub fn ac(&mut self) -> AC_W {
        AC_W { w: self }
    }
}

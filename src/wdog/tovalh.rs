#[doc = "Reader of register TOVALH"]
pub type R = crate::R<u8, super::TOVALH>;
#[doc = "Writer for register TOVALH"]
pub type W = crate::W<u8, super::TOVALH>;
#[doc = "Register TOVALH `reset()`'s with value 0"]
impl crate::ResetValue for super::TOVALH {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOVALHIGH`"]
pub type TOVALHIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOVALHIGH`"]
pub struct TOVALHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVALHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - High byte of the timeout value"]
    #[inline(always)]
    pub fn tovalhigh(&self) -> TOVALHIGH_R {
        TOVALHIGH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High byte of the timeout value"]
    #[inline(always)]
    pub fn tovalhigh(&mut self) -> TOVALHIGH_W {
        TOVALHIGH_W { w: self }
    }
}

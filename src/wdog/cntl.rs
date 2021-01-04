#[doc = "Reader of register CNTL"]
pub type R = crate::R<u8, super::CNTL>;
#[doc = "Writer for register CNTL"]
pub type W = crate::W<u8, super::CNTL>;
#[doc = "Register CNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTLOW`"]
pub type CNTLOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNTLOW`"]
pub struct CNTLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cntlow(&self) -> CNTLOW_R {
        CNTLOW_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cntlow(&mut self) -> CNTLOW_W {
        CNTLOW_W { w: self }
    }
}

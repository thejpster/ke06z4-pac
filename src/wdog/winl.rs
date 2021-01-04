#[doc = "Reader of register WINL"]
pub type R = crate::R<u8, super::WINL>;
#[doc = "Writer for register WINL"]
pub type W = crate::W<u8, super::WINL>;
#[doc = "Register WINL `reset()`'s with value 0"]
impl crate::ResetValue for super::WINL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WINLOW`"]
pub type WINLOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WINLOW`"]
pub struct WINLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> WINLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low byte of Watchdog Window"]
    #[inline(always)]
    pub fn winlow(&self) -> WINLOW_R {
        WINLOW_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of Watchdog Window"]
    #[inline(always)]
    pub fn winlow(&mut self) -> WINLOW_W {
        WINLOW_W { w: self }
    }
}

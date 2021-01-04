#[doc = "Reader of register TOVALL"]
pub type R = crate::R<u8, super::TOVALL>;
#[doc = "Writer for register TOVALL"]
pub type W = crate::W<u8, super::TOVALL>;
#[doc = "Register TOVALL `reset()`'s with value 0x04"]
impl crate::ResetValue for super::TOVALL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `TOVALLOW`"]
pub type TOVALLOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOVALLOW`"]
pub struct TOVALLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVALLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low byte of the timeout value"]
    #[inline(always)]
    pub fn tovallow(&self) -> TOVALLOW_R {
        TOVALLOW_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of the timeout value"]
    #[inline(always)]
    pub fn tovallow(&mut self) -> TOVALLOW_W {
        TOVALLOW_W { w: self }
    }
}

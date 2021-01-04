#[doc = "Reader of register FCCOBIX"]
pub type R = crate::R<u8, super::FCCOBIX>;
#[doc = "Writer for register FCCOBIX"]
pub type W = crate::W<u8, super::FCCOBIX>;
#[doc = "Register FCCOBIX `reset()`'s with value 0"]
impl crate::ResetValue for super::FCCOBIX {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCOBIX`"]
pub type CCOBIX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCOBIX`"]
pub struct CCOBIX_W<'a> {
    w: &'a mut W,
}
impl<'a> CCOBIX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Common Command Register Index"]
    #[inline(always)]
    pub fn ccobix(&self) -> CCOBIX_R {
        CCOBIX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Common Command Register Index"]
    #[inline(always)]
    pub fn ccobix(&mut self) -> CCOBIX_W {
        CCOBIX_W { w: self }
    }
}

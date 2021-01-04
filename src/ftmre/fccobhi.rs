#[doc = "Reader of register FCCOBHI"]
pub type R = crate::R<u8, super::FCCOBHI>;
#[doc = "Writer for register FCCOBHI"]
pub type W = crate::W<u8, super::FCCOBHI>;
#[doc = "Register FCCOBHI `reset()`'s with value 0"]
impl crate::ResetValue for super::FCCOBHI {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCOB`"]
pub type CCOB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCOB`"]
pub struct CCOB_W<'a> {
    w: &'a mut W,
}
impl<'a> CCOB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Common Command Object Bit 15:8"]
    #[inline(always)]
    pub fn ccob(&self) -> CCOB_R {
        CCOB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Common Command Object Bit 15:8"]
    #[inline(always)]
    pub fn ccob(&mut self) -> CCOB_W {
        CCOB_W { w: self }
    }
}

#[doc = "Reader of register CV"]
pub type R = crate::R<u32, super::CV>;
#[doc = "Writer for register CV"]
pub type W = crate::W<u32, super::CV>;
#[doc = "Register CV `reset()`'s with value 0"]
impl crate::ResetValue for super::CV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CV`"]
pub type CV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CV`"]
pub struct CV_W<'a> {
    w: &'a mut W,
}
impl<'a> CV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Conversion Result\\[11:0\\]"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Conversion Result\\[11:0\\]"]
    #[inline(always)]
    pub fn cv(&mut self) -> CV_W {
        CV_W { w: self }
    }
}

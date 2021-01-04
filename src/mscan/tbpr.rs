#[doc = "Reader of register TBPR"]
pub type R = crate::R<u8, super::TBPR>;
#[doc = "Writer for register TBPR"]
pub type W = crate::W<u8, super::TBPR>;
#[doc = "Register TBPR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBPR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRIO`"]
pub type PRIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIO`"]
pub struct PRIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority"]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W {
        PRIO_W { w: self }
    }
}

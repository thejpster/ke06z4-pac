#[doc = "Reader of register CNTH"]
pub type R = crate::R<u8, super::CNTH>;
#[doc = "Writer for register CNTH"]
pub type W = crate::W<u8, super::CNTH>;
#[doc = "Register CNTH `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTH {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTHIGH`"]
pub type CNTHIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNTHIGH`"]
pub struct CNTHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - High byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cnthigh(&self) -> CNTHIGH_R {
        CNTHIGH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cnthigh(&mut self) -> CNTHIGH_W {
        CNTHIGH_W { w: self }
    }
}

#[doc = "Reader of register TEDSR%s"]
pub type R = crate::R<u8, super::TEDSR>;
#[doc = "Writer for register TEDSR%s"]
pub type W = crate::W<u8, super::TEDSR>;
#[doc = "Register TEDSR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::TEDSR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDB`"]
pub type TDB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDB`"]
pub struct TDB_W<'a> {
    w: &'a mut W,
}
impl<'a> TDB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Bits"]
    #[inline(always)]
    pub fn tdb(&self) -> TDB_R {
        TDB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bits"]
    #[inline(always)]
    pub fn tdb(&mut self) -> TDB_W {
        TDB_W { w: self }
    }
}

#[doc = "Reader of register CANBTR0"]
pub type R = crate::R<u8, super::CANBTR0>;
#[doc = "Writer for register CANBTR0"]
pub type W = crate::W<u8, super::CANBTR0>;
#[doc = "Register CANBTR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CANBTR0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Baud Rate Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRP_A {
    #[doc = "0: 1"]
    _000000 = 0,
    #[doc = "1: 2"]
    _000001 = 1,
    #[doc = "2: ......"]
    _000010 = 2,
    #[doc = "3: ......"]
    _000011 = 3,
    #[doc = "62: 63"]
    _111110 = 62,
    #[doc = "63: 64"]
    _111111 = 63,
}
impl From<BRP_A> for u8 {
    #[inline(always)]
    fn from(variant: BRP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BRP`"]
pub type BRP_R = crate::R<u8, BRP_A>;
impl BRP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BRP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BRP_A::_000000),
            1 => Val(BRP_A::_000001),
            2 => Val(BRP_A::_000010),
            3 => Val(BRP_A::_000011),
            62 => Val(BRP_A::_111110),
            63 => Val(BRP_A::_111111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline(always)]
    pub fn is_000000(&self) -> bool {
        *self == BRP_A::_000000
    }
    #[doc = "Checks if the value of the field is `_000001`"]
    #[inline(always)]
    pub fn is_000001(&self) -> bool {
        *self == BRP_A::_000001
    }
    #[doc = "Checks if the value of the field is `_000010`"]
    #[inline(always)]
    pub fn is_000010(&self) -> bool {
        *self == BRP_A::_000010
    }
    #[doc = "Checks if the value of the field is `_000011`"]
    #[inline(always)]
    pub fn is_000011(&self) -> bool {
        *self == BRP_A::_000011
    }
    #[doc = "Checks if the value of the field is `_111110`"]
    #[inline(always)]
    pub fn is_111110(&self) -> bool {
        *self == BRP_A::_111110
    }
    #[doc = "Checks if the value of the field is `_111111`"]
    #[inline(always)]
    pub fn is_111111(&self) -> bool {
        *self == BRP_A::_111111
    }
}
#[doc = "Write proxy for field `BRP`"]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn _000000(self) -> &'a mut W {
        self.variant(BRP_A::_000000)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn _000001(self) -> &'a mut W {
        self.variant(BRP_A::_000001)
    }
    #[doc = "......"]
    #[inline(always)]
    pub fn _000010(self) -> &'a mut W {
        self.variant(BRP_A::_000010)
    }
    #[doc = "......"]
    #[inline(always)]
    pub fn _000011(self) -> &'a mut W {
        self.variant(BRP_A::_000011)
    }
    #[doc = "63"]
    #[inline(always)]
    pub fn _111110(self) -> &'a mut W {
        self.variant(BRP_A::_111110)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn _111111(self) -> &'a mut W {
        self.variant(BRP_A::_111111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
#[doc = "Synchronization Jump Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SJW_A {
    #[doc = "0: 1 Tq clock cycle."]
    _00 = 0,
    #[doc = "1: 2 Tq clock cycles."]
    _01 = 1,
    #[doc = "2: 3 Tq clock cycle."]
    _10 = 2,
    #[doc = "3: 4 Tq clock cycles."]
    _11 = 3,
}
impl From<SJW_A> for u8 {
    #[inline(always)]
    fn from(variant: SJW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SJW`"]
pub type SJW_R = crate::R<u8, SJW_A>;
impl SJW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SJW_A {
        match self.bits {
            0 => SJW_A::_00,
            1 => SJW_A::_01,
            2 => SJW_A::_10,
            3 => SJW_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SJW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SJW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SJW_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SJW_A::_11
    }
}
#[doc = "Write proxy for field `SJW`"]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SJW_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 Tq clock cycle."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SJW_A::_00)
    }
    #[doc = "2 Tq clock cycles."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SJW_A::_01)
    }
    #[doc = "3 Tq clock cycle."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SJW_A::_10)
    }
    #[doc = "4 Tq clock cycles."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SJW_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Bits 6:7 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
}

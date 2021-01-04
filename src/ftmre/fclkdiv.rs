#[doc = "Reader of register FCLKDIV"]
pub type R = crate::R<u8, super::FCLKDIV>;
#[doc = "Writer for register FCLKDIV"]
pub type W = crate::W<u8, super::FCLKDIV>;
#[doc = "Register FCLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::FCLKDIV {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FDIV`"]
pub type FDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDIV`"]
pub struct FDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
#[doc = "Clock Divider Locked\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDIVLCK_A {
    #[doc = "0: FDIV field is open for writing."]
    _0 = 0,
    #[doc = "1: FDIV value is locked and cannot be changed. After the lock bit is set high, only reset can clear this bit and restore writability to the FDIV field in user mode."]
    _1 = 1,
}
impl From<FDIVLCK_A> for bool {
    #[inline(always)]
    fn from(variant: FDIVLCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDIVLCK`"]
pub type FDIVLCK_R = crate::R<bool, FDIVLCK_A>;
impl FDIVLCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDIVLCK_A {
        match self.bits {
            false => FDIVLCK_A::_0,
            true => FDIVLCK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDIVLCK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDIVLCK_A::_1
    }
}
#[doc = "Write proxy for field `FDIVLCK`"]
pub struct FDIVLCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIVLCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDIVLCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FDIV field is open for writing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDIVLCK_A::_0)
    }
    #[doc = "FDIV value is locked and cannot be changed. After the lock bit is set high, only reset can clear this bit and restore writability to the FDIV field in user mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDIVLCK_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Clock Divider Loaded\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDIVLD_A {
    #[doc = "0: FCLKDIV register has not been written since the last reset."]
    _0 = 0,
    #[doc = "1: FCLKDIV register has been written since the last reset."]
    _1 = 1,
}
impl From<FDIVLD_A> for bool {
    #[inline(always)]
    fn from(variant: FDIVLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDIVLD`"]
pub type FDIVLD_R = crate::R<bool, FDIVLD_A>;
impl FDIVLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDIVLD_A {
        match self.bits {
            false => FDIVLD_A::_0,
            true => FDIVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDIVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDIVLD_A::_1
    }
}
impl R {
    #[doc = "Bits 0:5 - Clock Divider Bits"]
    #[inline(always)]
    pub fn fdiv(&self) -> FDIV_R {
        FDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Clock Divider Locked"]
    #[inline(always)]
    pub fn fdivlck(&self) -> FDIVLCK_R {
        FDIVLCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock Divider Loaded"]
    #[inline(always)]
    pub fn fdivld(&self) -> FDIVLD_R {
        FDIVLD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clock Divider Bits"]
    #[inline(always)]
    pub fn fdiv(&mut self) -> FDIV_W {
        FDIV_W { w: self }
    }
    #[doc = "Bit 6 - Clock Divider Locked"]
    #[inline(always)]
    pub fn fdivlck(&mut self) -> FDIVLCK_W {
        FDIVLCK_W { w: self }
    }
}

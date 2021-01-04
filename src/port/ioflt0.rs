#[doc = "Reader of register IOFLT0"]
pub type R = crate::R<u32, super::IOFLT0>;
#[doc = "Writer for register IOFLT0"]
pub type W = crate::W<u32, super::IOFLT0>;
#[doc = "Register IOFLT0 `reset()`'s with value 0x00c0_0000"]
impl crate::ResetValue for super::IOFLT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00c0_0000
    }
}
#[doc = "Filter Selection for Input from PTA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTA_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTA_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTA`"]
pub type FLTA_R = crate::R<u8, FLTA_A>;
impl FLTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTA_A {
        match self.bits {
            0 => FLTA_A::_00,
            1 => FLTA_A::_01,
            2 => FLTA_A::_10,
            3 => FLTA_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTA_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTA_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTA_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTA_A::_11
    }
}
#[doc = "Write proxy for field `FLTA`"]
pub struct FLTA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTA_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTA_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTA_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTA_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Filter Selection for Input from PTB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTB_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTB_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTB`"]
pub type FLTB_R = crate::R<u8, FLTB_A>;
impl FLTB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTB_A {
        match self.bits {
            0 => FLTB_A::_00,
            1 => FLTB_A::_01,
            2 => FLTB_A::_10,
            3 => FLTB_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTB_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTB_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTB_A::_11
    }
}
#[doc = "Write proxy for field `FLTB`"]
pub struct FLTB_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTB_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTB_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTB_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTB_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Filter Selection for Input from PTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTC_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTC_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTC`"]
pub type FLTC_R = crate::R<u8, FLTC_A>;
impl FLTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTC_A {
        match self.bits {
            0 => FLTC_A::_00,
            1 => FLTC_A::_01,
            2 => FLTC_A::_10,
            3 => FLTC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTC_A::_11
    }
}
#[doc = "Write proxy for field `FLTC`"]
pub struct FLTC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTC_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTC_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTC_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Filter Selection for Input from PTD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTD_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTD_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTD`"]
pub type FLTD_R = crate::R<u8, FLTD_A>;
impl FLTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTD_A {
        match self.bits {
            0 => FLTD_A::_00,
            1 => FLTD_A::_01,
            2 => FLTD_A::_10,
            3 => FLTD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTD_A::_11
    }
}
#[doc = "Write proxy for field `FLTD`"]
pub struct FLTD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTD_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTD_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTD_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTD_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Filter Selection for Input from PTD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTE_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTE`"]
pub type FLTE_R = crate::R<u8, FLTE_A>;
impl FLTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTE_A {
        match self.bits {
            0 => FLTE_A::_00,
            1 => FLTE_A::_01,
            2 => FLTE_A::_10,
            3 => FLTE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTE_A::_11
    }
}
#[doc = "Write proxy for field `FLTE`"]
pub struct FLTE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTE_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTE_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTE_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Filter Selection for Input from PTF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTF_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTF_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTF`"]
pub type FLTF_R = crate::R<u8, FLTF_A>;
impl FLTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTF_A {
        match self.bits {
            0 => FLTF_A::_00,
            1 => FLTF_A::_01,
            2 => FLTF_A::_10,
            3 => FLTF_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTF_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTF_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTF_A::_11
    }
}
#[doc = "Write proxy for field `FLTF`"]
pub struct FLTF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTF_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTF_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTF_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTF_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Filter Selection for Input from PTG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTG_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTG_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTG`"]
pub type FLTG_R = crate::R<u8, FLTG_A>;
impl FLTG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTG_A {
        match self.bits {
            0 => FLTG_A::_00,
            1 => FLTG_A::_01,
            2 => FLTG_A::_10,
            3 => FLTG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTG_A::_11
    }
}
#[doc = "Write proxy for field `FLTG`"]
pub struct FLTG_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTG_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTG_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTG_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTG_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Filter Selection for Input from PTH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTH_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTH`"]
pub type FLTH_R = crate::R<u8, FLTH_A>;
impl FLTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTH_A {
        match self.bits {
            0 => FLTH_A::_00,
            1 => FLTH_A::_01,
            2 => FLTH_A::_10,
            3 => FLTH_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTH_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTH_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTH_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTH_A::_11
    }
}
#[doc = "Write proxy for field `FLTH`"]
pub struct FLTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTH_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTH_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTH_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTH_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Filter Selection for Input from RESET/IRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTRST_A {
    #[doc = "0: No filter."]
    _00 = 0,
    #[doc = "1: Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    _01 = 1,
    #[doc = "2: Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTRST_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTRST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTRST`"]
pub type FLTRST_R = crate::R<u8, FLTRST_A>;
impl FLTRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTRST_A {
        match self.bits {
            0 => FLTRST_A::_00,
            1 => FLTRST_A::_01,
            2 => FLTRST_A::_10,
            3 => FLTRST_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTRST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTRST_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTRST_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTRST_A::_11
    }
}
#[doc = "Write proxy for field `FLTRST`"]
pub struct FLTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTRST_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTRST_A::_00)
    }
    #[doc = "Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTRST_A::_01)
    }
    #[doc = "Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTRST_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTRST_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Filter selection for Input from KBI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTKBI0_A {
    #[doc = "0: No filter."]
    _00 = 0,
    #[doc = "1: Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    _01 = 1,
    #[doc = "2: Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTKBI0_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTKBI0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTKBI0`"]
pub type FLTKBI0_R = crate::R<u8, FLTKBI0_A>;
impl FLTKBI0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTKBI0_A {
        match self.bits {
            0 => FLTKBI0_A::_00,
            1 => FLTKBI0_A::_01,
            2 => FLTKBI0_A::_10,
            3 => FLTKBI0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTKBI0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTKBI0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTKBI0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTKBI0_A::_11
    }
}
#[doc = "Write proxy for field `FLTKBI0`"]
pub struct FLTKBI0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTKBI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTKBI0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTKBI0_A::_00)
    }
    #[doc = "Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTKBI0_A::_01)
    }
    #[doc = "Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTKBI0_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTKBI0_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Filter Selection for Input from KBI1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTKBI1_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    _01 = 1,
    #[doc = "2: Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTKBI1_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTKBI1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTKBI1`"]
pub type FLTKBI1_R = crate::R<u8, FLTKBI1_A>;
impl FLTKBI1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTKBI1_A {
        match self.bits {
            0 => FLTKBI1_A::_00,
            1 => FLTKBI1_A::_01,
            2 => FLTKBI1_A::_10,
            3 => FLTKBI1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTKBI1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTKBI1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTKBI1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTKBI1_A::_11
    }
}
#[doc = "Write proxy for field `FLTKBI1`"]
pub struct FLTKBI1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTKBI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTKBI1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTKBI1_A::_00)
    }
    #[doc = "Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTKBI1_A::_01)
    }
    #[doc = "Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTKBI1_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTKBI1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Filter Selection for Input from NMI\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTNMI_A {
    #[doc = "0: No filter."]
    _00 = 0,
    #[doc = "1: Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    _01 = 1,
    #[doc = "2: Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTNMI_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTNMI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTNMI`"]
pub type FLTNMI_R = crate::R<u8, FLTNMI_A>;
impl FLTNMI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTNMI_A {
        match self.bits {
            0 => FLTNMI_A::_00,
            1 => FLTNMI_A::_01,
            2 => FLTNMI_A::_10,
            3 => FLTNMI_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTNMI_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTNMI_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTNMI_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTNMI_A::_11
    }
}
#[doc = "Write proxy for field `FLTNMI`"]
pub struct FLTNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTNMI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTNMI_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTNMI_A::_00)
    }
    #[doc = "Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTNMI_A::_01)
    }
    #[doc = "Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTNMI_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTNMI_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Filter Division Set 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTDIV1_A {
    #[doc = "0: BUSCLK/2"]
    _00 = 0,
    #[doc = "1: BUSCLK/4"]
    _01 = 1,
    #[doc = "2: BUSCLK/8"]
    _10 = 2,
    #[doc = "3: BUSCLK/16"]
    _11 = 3,
}
impl From<FLTDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTDIV1`"]
pub type FLTDIV1_R = crate::R<u8, FLTDIV1_A>;
impl FLTDIV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTDIV1_A {
        match self.bits {
            0 => FLTDIV1_A::_00,
            1 => FLTDIV1_A::_01,
            2 => FLTDIV1_A::_10,
            3 => FLTDIV1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTDIV1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTDIV1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTDIV1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTDIV1_A::_11
    }
}
#[doc = "Write proxy for field `FLTDIV1`"]
pub struct FLTDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTDIV1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "BUSCLK/2"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTDIV1_A::_00)
    }
    #[doc = "BUSCLK/4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTDIV1_A::_01)
    }
    #[doc = "BUSCLK/8"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTDIV1_A::_10)
    }
    #[doc = "BUSCLK/16"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTDIV1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Filter Division Set 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTDIV2_A {
    #[doc = "0: BUSCLK/32"]
    _000 = 0,
    #[doc = "1: BUSCLK/64"]
    _001 = 1,
    #[doc = "2: BUSCLK/128"]
    _010 = 2,
    #[doc = "3: BUSCLK/256"]
    _011 = 3,
    #[doc = "4: BUSCLK/512"]
    _100 = 4,
    #[doc = "5: BUSCLK/1024"]
    _101 = 5,
    #[doc = "6: BUSCLK/2048"]
    _110 = 6,
    #[doc = "7: BUSCLK/4096"]
    _111 = 7,
}
impl From<FLTDIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTDIV2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTDIV2`"]
pub type FLTDIV2_R = crate::R<u8, FLTDIV2_A>;
impl FLTDIV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTDIV2_A {
        match self.bits {
            0 => FLTDIV2_A::_000,
            1 => FLTDIV2_A::_001,
            2 => FLTDIV2_A::_010,
            3 => FLTDIV2_A::_011,
            4 => FLTDIV2_A::_100,
            5 => FLTDIV2_A::_101,
            6 => FLTDIV2_A::_110,
            7 => FLTDIV2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FLTDIV2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FLTDIV2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FLTDIV2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FLTDIV2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FLTDIV2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FLTDIV2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FLTDIV2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FLTDIV2_A::_111
    }
}
#[doc = "Write proxy for field `FLTDIV2`"]
pub struct FLTDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTDIV2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "BUSCLK/32"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_000)
    }
    #[doc = "BUSCLK/64"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_001)
    }
    #[doc = "BUSCLK/128"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_010)
    }
    #[doc = "BUSCLK/256"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_011)
    }
    #[doc = "BUSCLK/512"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_100)
    }
    #[doc = "BUSCLK/1024"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_101)
    }
    #[doc = "BUSCLK/2048"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_110)
    }
    #[doc = "BUSCLK/4096"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FLTDIV2_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Filter Division Set 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTDIV3_A {
    #[doc = "0: LPOCLK"]
    _000 = 0,
    #[doc = "1: LPOCLK/2"]
    _001 = 1,
    #[doc = "2: LPOCLK/4"]
    _010 = 2,
    #[doc = "3: LPOCLK/8"]
    _011 = 3,
    #[doc = "4: LPOCLK/16"]
    _100 = 4,
    #[doc = "5: LPOCLK/32"]
    _101 = 5,
    #[doc = "6: LPOCLK/64"]
    _110 = 6,
    #[doc = "7: LPOCLK/128"]
    _111 = 7,
}
impl From<FLTDIV3_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTDIV3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTDIV3`"]
pub type FLTDIV3_R = crate::R<u8, FLTDIV3_A>;
impl FLTDIV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTDIV3_A {
        match self.bits {
            0 => FLTDIV3_A::_000,
            1 => FLTDIV3_A::_001,
            2 => FLTDIV3_A::_010,
            3 => FLTDIV3_A::_011,
            4 => FLTDIV3_A::_100,
            5 => FLTDIV3_A::_101,
            6 => FLTDIV3_A::_110,
            7 => FLTDIV3_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FLTDIV3_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FLTDIV3_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FLTDIV3_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FLTDIV3_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FLTDIV3_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FLTDIV3_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FLTDIV3_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FLTDIV3_A::_111
    }
}
#[doc = "Write proxy for field `FLTDIV3`"]
pub struct FLTDIV3_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTDIV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTDIV3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LPOCLK"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_000)
    }
    #[doc = "LPOCLK/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_001)
    }
    #[doc = "LPOCLK/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_010)
    }
    #[doc = "LPOCLK/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_011)
    }
    #[doc = "LPOCLK/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_100)
    }
    #[doc = "LPOCLK/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_101)
    }
    #[doc = "LPOCLK/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_110)
    }
    #[doc = "LPOCLK/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FLTDIV3_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Filter Selection for Input from PTA"]
    #[inline(always)]
    pub fn flta(&self) -> FLTA_R {
        FLTA_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Filter Selection for Input from PTB"]
    #[inline(always)]
    pub fn fltb(&self) -> FLTB_R {
        FLTB_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Filter Selection for Input from PTC"]
    #[inline(always)]
    pub fn fltc(&self) -> FLTC_R {
        FLTC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Filter Selection for Input from PTD"]
    #[inline(always)]
    pub fn fltd(&self) -> FLTD_R {
        FLTD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Filter Selection for Input from PTD"]
    #[inline(always)]
    pub fn flte(&self) -> FLTE_R {
        FLTE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Filter Selection for Input from PTF"]
    #[inline(always)]
    pub fn fltf(&self) -> FLTF_R {
        FLTF_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Filter Selection for Input from PTG"]
    #[inline(always)]
    pub fn fltg(&self) -> FLTG_R {
        FLTG_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Filter Selection for Input from PTH"]
    #[inline(always)]
    pub fn flth(&self) -> FLTH_R {
        FLTH_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Filter Selection for Input from RESET/IRQ"]
    #[inline(always)]
    pub fn fltrst(&self) -> FLTRST_R {
        FLTRST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Filter selection for Input from KBI0"]
    #[inline(always)]
    pub fn fltkbi0(&self) -> FLTKBI0_R {
        FLTKBI0_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Filter Selection for Input from KBI1"]
    #[inline(always)]
    pub fn fltkbi1(&self) -> FLTKBI1_R {
        FLTKBI1_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Filter Selection for Input from NMI"]
    #[inline(always)]
    pub fn fltnmi(&self) -> FLTNMI_R {
        FLTNMI_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Filter Division Set 1"]
    #[inline(always)]
    pub fn fltdiv1(&self) -> FLTDIV1_R {
        FLTDIV1_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28 - Filter Division Set 2"]
    #[inline(always)]
    pub fn fltdiv2(&self) -> FLTDIV2_R {
        FLTDIV2_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - Filter Division Set 3"]
    #[inline(always)]
    pub fn fltdiv3(&self) -> FLTDIV3_R {
        FLTDIV3_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Filter Selection for Input from PTA"]
    #[inline(always)]
    pub fn flta(&mut self) -> FLTA_W {
        FLTA_W { w: self }
    }
    #[doc = "Bits 2:3 - Filter Selection for Input from PTB"]
    #[inline(always)]
    pub fn fltb(&mut self) -> FLTB_W {
        FLTB_W { w: self }
    }
    #[doc = "Bits 4:5 - Filter Selection for Input from PTC"]
    #[inline(always)]
    pub fn fltc(&mut self) -> FLTC_W {
        FLTC_W { w: self }
    }
    #[doc = "Bits 6:7 - Filter Selection for Input from PTD"]
    #[inline(always)]
    pub fn fltd(&mut self) -> FLTD_W {
        FLTD_W { w: self }
    }
    #[doc = "Bits 8:9 - Filter Selection for Input from PTD"]
    #[inline(always)]
    pub fn flte(&mut self) -> FLTE_W {
        FLTE_W { w: self }
    }
    #[doc = "Bits 10:11 - Filter Selection for Input from PTF"]
    #[inline(always)]
    pub fn fltf(&mut self) -> FLTF_W {
        FLTF_W { w: self }
    }
    #[doc = "Bits 12:13 - Filter Selection for Input from PTG"]
    #[inline(always)]
    pub fn fltg(&mut self) -> FLTG_W {
        FLTG_W { w: self }
    }
    #[doc = "Bits 14:15 - Filter Selection for Input from PTH"]
    #[inline(always)]
    pub fn flth(&mut self) -> FLTH_W {
        FLTH_W { w: self }
    }
    #[doc = "Bits 16:17 - Filter Selection for Input from RESET/IRQ"]
    #[inline(always)]
    pub fn fltrst(&mut self) -> FLTRST_W {
        FLTRST_W { w: self }
    }
    #[doc = "Bits 18:19 - Filter selection for Input from KBI0"]
    #[inline(always)]
    pub fn fltkbi0(&mut self) -> FLTKBI0_W {
        FLTKBI0_W { w: self }
    }
    #[doc = "Bits 20:21 - Filter Selection for Input from KBI1"]
    #[inline(always)]
    pub fn fltkbi1(&mut self) -> FLTKBI1_W {
        FLTKBI1_W { w: self }
    }
    #[doc = "Bits 22:23 - Filter Selection for Input from NMI"]
    #[inline(always)]
    pub fn fltnmi(&mut self) -> FLTNMI_W {
        FLTNMI_W { w: self }
    }
    #[doc = "Bits 24:25 - Filter Division Set 1"]
    #[inline(always)]
    pub fn fltdiv1(&mut self) -> FLTDIV1_W {
        FLTDIV1_W { w: self }
    }
    #[doc = "Bits 26:28 - Filter Division Set 2"]
    #[inline(always)]
    pub fn fltdiv2(&mut self) -> FLTDIV2_W {
        FLTDIV2_W { w: self }
    }
    #[doc = "Bits 29:31 - Filter Division Set 3"]
    #[inline(always)]
    pub fn fltdiv3(&mut self) -> FLTDIV3_W {
        FLTDIV3_W { w: self }
    }
}

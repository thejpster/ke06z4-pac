#[doc = "Reader of register IOFLT1"]
pub type R = crate::R<u32, super::IOFLT1>;
#[doc = "Writer for register IOFLT1"]
pub type W = crate::W<u32, super::IOFLT1>;
#[doc = "Register IOFLT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IOFLT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Filter Selection for Input from PTI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTI_A {
    #[doc = "0: BUSCLK"]
    _00 = 0,
    #[doc = "1: FLTDIV1"]
    _01 = 1,
    #[doc = "2: FLTDIV2"]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTI_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTI`"]
pub type FLTI_R = crate::R<u8, FLTI_A>;
impl FLTI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTI_A {
        match self.bits {
            0 => FLTI_A::_00,
            1 => FLTI_A::_01,
            2 => FLTI_A::_10,
            3 => FLTI_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTI_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTI_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTI_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTI_A::_11
    }
}
#[doc = "Write proxy for field `FLTI`"]
pub struct FLTI_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTI_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTI_A::_00)
    }
    #[doc = "FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTI_A::_01)
    }
    #[doc = "FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTI_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTI_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Filter Selection for Input from IRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTIRQ_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    _01 = 1,
    #[doc = "2: Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    _10 = 2,
    #[doc = "3: FLTDIV3"]
    _11 = 3,
}
impl From<FLTIRQ_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTIRQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTIRQ`"]
pub type FLTIRQ_R = crate::R<u8, FLTIRQ_A>;
impl FLTIRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTIRQ_A {
        match self.bits {
            0 => FLTIRQ_A::_00,
            1 => FLTIRQ_A::_01,
            2 => FLTIRQ_A::_10,
            3 => FLTIRQ_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTIRQ_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTIRQ_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTIRQ_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTIRQ_A::_11
    }
}
#[doc = "Write proxy for field `FLTIRQ`"]
pub struct FLTIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTIRQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTIRQ_A::_00)
    }
    #[doc = "Selects FLTDIV1, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTIRQ_A::_01)
    }
    #[doc = "Selects FLTDIV2, and will switch to FLTDIV3 in Stop mode automatically."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTIRQ_A::_10)
    }
    #[doc = "FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTIRQ_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Filter Selection For Input from FTM0CH0/FTM0CH1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTFTM0_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Select FLTDIV1"]
    _01 = 1,
    #[doc = "2: Select FLTDIV2"]
    _10 = 2,
    #[doc = "3: Select FLTDIV3"]
    _11 = 3,
}
impl From<FLTFTM0_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTFTM0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTFTM0`"]
pub type FLTFTM0_R = crate::R<u8, FLTFTM0_A>;
impl FLTFTM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTFTM0_A {
        match self.bits {
            0 => FLTFTM0_A::_00,
            1 => FLTFTM0_A::_01,
            2 => FLTFTM0_A::_10,
            3 => FLTFTM0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTFTM0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTFTM0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTFTM0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTFTM0_A::_11
    }
}
#[doc = "Write proxy for field `FLTFTM0`"]
pub struct FLTFTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTFTM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTFTM0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTFTM0_A::_00)
    }
    #[doc = "Select FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTFTM0_A::_01)
    }
    #[doc = "Select FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTFTM0_A::_10)
    }
    #[doc = "Select FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTFTM0_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Filter Selection For Input from FTM1CH0/FTM1CH1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTFTM1_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Select FLTDIV1"]
    _01 = 1,
    #[doc = "2: Select FLTDIV2"]
    _10 = 2,
    #[doc = "3: Select FLTDIV3"]
    _11 = 3,
}
impl From<FLTFTM1_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTFTM1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTFTM1`"]
pub type FLTFTM1_R = crate::R<u8, FLTFTM1_A>;
impl FLTFTM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTFTM1_A {
        match self.bits {
            0 => FLTFTM1_A::_00,
            1 => FLTFTM1_A::_01,
            2 => FLTFTM1_A::_10,
            3 => FLTFTM1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTFTM1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTFTM1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTFTM1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTFTM1_A::_11
    }
}
#[doc = "Write proxy for field `FLTFTM1`"]
pub struct FLTFTM1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTFTM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTFTM1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTFTM1_A::_00)
    }
    #[doc = "Select FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTFTM1_A::_01)
    }
    #[doc = "Select FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTFTM1_A::_10)
    }
    #[doc = "Select FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTFTM1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Filter Selection For Input from PWT_IN1/PWT_IN0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTPWT_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Select FLTDIV1"]
    _01 = 1,
    #[doc = "2: Select FLTDIV2"]
    _10 = 2,
    #[doc = "3: Select FLTDIV3"]
    _11 = 3,
}
impl From<FLTPWT_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTPWT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTPWT`"]
pub type FLTPWT_R = crate::R<u8, FLTPWT_A>;
impl FLTPWT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTPWT_A {
        match self.bits {
            0 => FLTPWT_A::_00,
            1 => FLTPWT_A::_01,
            2 => FLTPWT_A::_10,
            3 => FLTPWT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTPWT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTPWT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTPWT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTPWT_A::_11
    }
}
#[doc = "Write proxy for field `FLTPWT`"]
pub struct FLTPWT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTPWT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTPWT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTPWT_A::_00)
    }
    #[doc = "Select FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTPWT_A::_01)
    }
    #[doc = "Select FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTPWT_A::_10)
    }
    #[doc = "Select FLTDIV3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTPWT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Filter Selection For Input from SCL0/SDA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTI2C0_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Select FLTDIV1"]
    _01 = 1,
    #[doc = "2: Select FLTDIV2"]
    _10 = 2,
    #[doc = "3: Select BUSCLK"]
    _11 = 3,
}
impl From<FLTI2C0_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTI2C0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTI2C0`"]
pub type FLTI2C0_R = crate::R<u8, FLTI2C0_A>;
impl FLTI2C0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTI2C0_A {
        match self.bits {
            0 => FLTI2C0_A::_00,
            1 => FLTI2C0_A::_01,
            2 => FLTI2C0_A::_10,
            3 => FLTI2C0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTI2C0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTI2C0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTI2C0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTI2C0_A::_11
    }
}
#[doc = "Write proxy for field `FLTI2C0`"]
pub struct FLTI2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTI2C0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTI2C0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTI2C0_A::_00)
    }
    #[doc = "Select FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTI2C0_A::_01)
    }
    #[doc = "Select FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTI2C0_A::_10)
    }
    #[doc = "Select BUSCLK"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTI2C0_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Filter Selection For Input from SCL1/SDA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTI2C1_A {
    #[doc = "0: No filter"]
    _00 = 0,
    #[doc = "1: Select FLTDIV1"]
    _01 = 1,
    #[doc = "2: Select FLTDIV2"]
    _10 = 2,
    #[doc = "3: Select BUSCLK"]
    _11 = 3,
}
impl From<FLTI2C1_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTI2C1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTI2C1`"]
pub type FLTI2C1_R = crate::R<u8, FLTI2C1_A>;
impl FLTI2C1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTI2C1_A {
        match self.bits {
            0 => FLTI2C1_A::_00,
            1 => FLTI2C1_A::_01,
            2 => FLTI2C1_A::_10,
            3 => FLTI2C1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTI2C1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTI2C1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLTI2C1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLTI2C1_A::_11
    }
}
#[doc = "Write proxy for field `FLTI2C1`"]
pub struct FLTI2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTI2C1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTI2C1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLTI2C1_A::_00)
    }
    #[doc = "Select FLTDIV1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLTI2C1_A::_01)
    }
    #[doc = "Select FLTDIV2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLTI2C1_A::_10)
    }
    #[doc = "Select BUSCLK"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLTI2C1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Filter Selection for Input from PTI"]
    #[inline(always)]
    pub fn flti(&self) -> FLTI_R {
        FLTI_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Filter Selection for Input from IRQ"]
    #[inline(always)]
    pub fn fltirq(&self) -> FLTIRQ_R {
        FLTIRQ_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Filter Selection For Input from FTM0CH0/FTM0CH1"]
    #[inline(always)]
    pub fn fltftm0(&self) -> FLTFTM0_R {
        FLTFTM0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Filter Selection For Input from FTM1CH0/FTM1CH1"]
    #[inline(always)]
    pub fn fltftm1(&self) -> FLTFTM1_R {
        FLTFTM1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Filter Selection For Input from PWT_IN1/PWT_IN0"]
    #[inline(always)]
    pub fn fltpwt(&self) -> FLTPWT_R {
        FLTPWT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Filter Selection For Input from SCL0/SDA0"]
    #[inline(always)]
    pub fn flti2c0(&self) -> FLTI2C0_R {
        FLTI2C0_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Filter Selection For Input from SCL1/SDA1"]
    #[inline(always)]
    pub fn flti2c1(&self) -> FLTI2C1_R {
        FLTI2C1_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Filter Selection for Input from PTI"]
    #[inline(always)]
    pub fn flti(&mut self) -> FLTI_W {
        FLTI_W { w: self }
    }
    #[doc = "Bits 4:5 - Filter Selection for Input from IRQ"]
    #[inline(always)]
    pub fn fltirq(&mut self) -> FLTIRQ_W {
        FLTIRQ_W { w: self }
    }
    #[doc = "Bits 6:7 - Filter Selection For Input from FTM0CH0/FTM0CH1"]
    #[inline(always)]
    pub fn fltftm0(&mut self) -> FLTFTM0_W {
        FLTFTM0_W { w: self }
    }
    #[doc = "Bits 8:9 - Filter Selection For Input from FTM1CH0/FTM1CH1"]
    #[inline(always)]
    pub fn fltftm1(&mut self) -> FLTFTM1_W {
        FLTFTM1_W { w: self }
    }
    #[doc = "Bits 10:11 - Filter Selection For Input from PWT_IN1/PWT_IN0"]
    #[inline(always)]
    pub fn fltpwt(&mut self) -> FLTPWT_W {
        FLTPWT_W { w: self }
    }
    #[doc = "Bits 12:13 - Filter Selection For Input from SCL0/SDA0"]
    #[inline(always)]
    pub fn flti2c0(&mut self) -> FLTI2C0_W {
        FLTI2C0_W { w: self }
    }
    #[doc = "Bits 14:15 - Filter Selection For Input from SCL1/SDA1"]
    #[inline(always)]
    pub fn flti2c1(&mut self) -> FLTI2C1_W {
        FLTI2C1_W { w: self }
    }
}

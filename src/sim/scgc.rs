#[doc = "Reader of register SCGC"]
pub type R = crate::R<u32, super::SCGC>;
#[doc = "Writer for register SCGC"]
pub type W = crate::W<u32, super::SCGC>;
#[doc = "Register SCGC `reset()`'s with value 0x3000"]
impl crate::ResetValue for super::SCGC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3000
    }
}
#[doc = "RTC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_A {
    #[doc = "0: Bus clock to the RTC module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the RTC module is enabled."]
    _1 = 1,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, RTC_A>;
impl RTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::_0,
            true => RTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTC_A::_1
    }
}
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the RTC module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTC_A::_0)
    }
    #[doc = "Bus clock to the RTC module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTC_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "PIT Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIT_A {
    #[doc = "0: Bus clock to the PIT module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the PIT module is enabled."]
    _1 = 1,
}
impl From<PIT_A> for bool {
    #[inline(always)]
    fn from(variant: PIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIT`"]
pub type PIT_R = crate::R<bool, PIT_A>;
impl PIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT_A {
        match self.bits {
            false => PIT_A::_0,
            true => PIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIT_A::_1
    }
}
#[doc = "Write proxy for field `PIT`"]
pub struct PIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the PIT module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIT_A::_0)
    }
    #[doc = "Bus clock to the PIT module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "PWT Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWT_A {
    #[doc = "0: Timer clock to the PWT module is disabled."]
    _0 = 0,
    #[doc = "1: Timer clock to the PWT module is enabled."]
    _1 = 1,
}
impl From<PWT_A> for bool {
    #[inline(always)]
    fn from(variant: PWT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWT`"]
pub type PWT_R = crate::R<bool, PWT_A>;
impl PWT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWT_A {
        match self.bits {
            false => PWT_A::_0,
            true => PWT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWT_A::_1
    }
}
#[doc = "Write proxy for field `PWT`"]
pub struct PWT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer clock to the PWT module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWT_A::_0)
    }
    #[doc = "Timer clock to the PWT module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "FTM0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0_A {
    #[doc = "0: Bus clock to the FTM0 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the FTM0 module is enabled."]
    _1 = 1,
}
impl From<FTM0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM0`"]
pub type FTM0_R = crate::R<bool, FTM0_A>;
impl FTM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0_A {
        match self.bits {
            false => FTM0_A::_0,
            true => FTM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0_A::_1
    }
}
#[doc = "Write proxy for field `FTM0`"]
pub struct FTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the FTM0 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0_A::_0)
    }
    #[doc = "Bus clock to the FTM0 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "FTM1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1_A {
    #[doc = "0: Bus clock to the FTM1 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the FTM1 module is enabled."]
    _1 = 1,
}
impl From<FTM1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM1`"]
pub type FTM1_R = crate::R<bool, FTM1_A>;
impl FTM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1_A {
        match self.bits {
            false => FTM1_A::_0,
            true => FTM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1_A::_1
    }
}
#[doc = "Write proxy for field `FTM1`"]
pub struct FTM1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the FTM1 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1_A::_0)
    }
    #[doc = "Bus clock to the FTM1 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "FTM2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2_A {
    #[doc = "0: Bus clock to the FTM2 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the FTM2 module is enabled."]
    _1 = 1,
}
impl From<FTM2_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM2`"]
pub type FTM2_R = crate::R<bool, FTM2_A>;
impl FTM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2_A {
        match self.bits {
            false => FTM2_A::_0,
            true => FTM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2_A::_1
    }
}
#[doc = "Write proxy for field `FTM2`"]
pub struct FTM2_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the FTM2 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2_A::_0)
    }
    #[doc = "Bus clock to the FTM2 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "CRC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_A {
    #[doc = "0: Bus clock to the CRC module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the CRC module is enabled."]
    _1 = 1,
}
impl From<CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<bool, CRC_A>;
impl CRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::_0,
            true => CRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRC_A::_1
    }
}
#[doc = "Write proxy for field `CRC`"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the CRC module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_A::_0)
    }
    #[doc = "Bus clock to the CRC module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Flash Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_A {
    #[doc = "0: Bus clock to the flash module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the flash module is enabled."]
    _1 = 1,
}
impl From<FLASH_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH`"]
pub type FLASH_R = crate::R<bool, FLASH_A>;
impl FLASH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_A {
        match self.bits {
            false => FLASH_A::_0,
            true => FLASH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLASH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLASH_A::_1
    }
}
#[doc = "Write proxy for field `FLASH`"]
pub struct FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the flash module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLASH_A::_0)
    }
    #[doc = "Bus clock to the flash module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLASH_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "SWD (single wire debugger) Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWD_A {
    #[doc = "0: Bus clock to the SWD module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the SWD module is enabled."]
    _1 = 1,
}
impl From<SWD_A> for bool {
    #[inline(always)]
    fn from(variant: SWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWD`"]
pub type SWD_R = crate::R<bool, SWD_A>;
impl SWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWD_A {
        match self.bits {
            false => SWD_A::_0,
            true => SWD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWD_A::_1
    }
}
#[doc = "Write proxy for field `SWD`"]
pub struct SWD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the SWD module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWD_A::_0)
    }
    #[doc = "Bus clock to the SWD module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "MSCAN Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSCAN_A {
    #[doc = "0: Bus clock to the MSCAN module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the MSCAN module is enabled."]
    _1 = 1,
}
impl From<MSCAN_A> for bool {
    #[inline(always)]
    fn from(variant: MSCAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSCAN`"]
pub type MSCAN_R = crate::R<bool, MSCAN_A>;
impl MSCAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSCAN_A {
        match self.bits {
            false => MSCAN_A::_0,
            true => MSCAN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSCAN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSCAN_A::_1
    }
}
#[doc = "Write proxy for field `MSCAN`"]
pub struct MSCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSCAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSCAN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the MSCAN module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSCAN_A::_0)
    }
    #[doc = "Bus clock to the MSCAN module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSCAN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "I2C0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_A {
    #[doc = "0: Bus clock to the I2C0 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the I2C0 module is enabled."]
    _1 = 1,
}
impl From<I2C0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C0`"]
pub type I2C0_R = crate::R<bool, I2C0_A>;
impl I2C0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_A {
        match self.bits {
            false => I2C0_A::_0,
            true => I2C0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C0_A::_1
    }
}
#[doc = "Write proxy for field `I2C0`"]
pub struct I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the I2C0 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C0_A::_0)
    }
    #[doc = "Bus clock to the I2C0 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "I2C1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_A {
    #[doc = "0: Bus clock to the I2C1 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the I2C1 module is enabled."]
    _1 = 1,
}
impl From<I2C1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C1`"]
pub type I2C1_R = crate::R<bool, I2C1_A>;
impl I2C1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_A {
        match self.bits {
            false => I2C1_A::_0,
            true => I2C1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C1_A::_1
    }
}
#[doc = "Write proxy for field `I2C1`"]
pub struct I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the I2C1 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C1_A::_0)
    }
    #[doc = "Bus clock to the I2C1 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "SPI0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_A {
    #[doc = "0: Bus clock to the SPI0 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the SPI0 module is enabled."]
    _1 = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0`"]
pub type SPI0_R = crate::R<bool, SPI0_A>;
impl SPI0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::_0,
            true => SPI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI0_A::_1
    }
}
#[doc = "Write proxy for field `SPI0`"]
pub struct SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the SPI0 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0_A::_0)
    }
    #[doc = "Bus clock to the SPI0 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "SPI1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_A {
    #[doc = "0: Bus clock to the SPI1 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the SPI1 module is enabled."]
    _1 = 1,
}
impl From<SPI1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1`"]
pub type SPI1_R = crate::R<bool, SPI1_A>;
impl SPI1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::_0,
            true => SPI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI1_A::_1
    }
}
#[doc = "Write proxy for field `SPI1`"]
pub struct SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the SPI1 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1_A::_0)
    }
    #[doc = "Bus clock to the SPI1 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "UART0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_A {
    #[doc = "0: Bus clock to the UART0 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the UART0 module is enabled."]
    _1 = 1,
}
impl From<UART0_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0`"]
pub type UART0_R = crate::R<bool, UART0_A>;
impl UART0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_A {
        match self.bits {
            false => UART0_A::_0,
            true => UART0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART0_A::_1
    }
}
#[doc = "Write proxy for field `UART0`"]
pub struct UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the UART0 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0_A::_0)
    }
    #[doc = "Bus clock to the UART0 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "UART1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_A {
    #[doc = "0: Bus clock to the UART1 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the UART1 module is enabled."]
    _1 = 1,
}
impl From<UART1_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART1`"]
pub type UART1_R = crate::R<bool, UART1_A>;
impl UART1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_A {
        match self.bits {
            false => UART1_A::_0,
            true => UART1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART1_A::_1
    }
}
#[doc = "Write proxy for field `UART1`"]
pub struct UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the UART1 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1_A::_0)
    }
    #[doc = "Bus clock to the UART1 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "UART2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_A {
    #[doc = "0: Bus clock to the UART2 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the UART2 module is enabled."]
    _1 = 1,
}
impl From<UART2_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART2`"]
pub type UART2_R = crate::R<bool, UART2_A>;
impl UART2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2_A {
        match self.bits {
            false => UART2_A::_0,
            true => UART2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART2_A::_1
    }
}
#[doc = "Write proxy for field `UART2`"]
pub struct UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the UART2 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART2_A::_0)
    }
    #[doc = "Bus clock to the UART2 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "KBI0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KBI0_A {
    #[doc = "0: Bus clock to the KBI0 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the KBI0 module is enabled."]
    _1 = 1,
}
impl From<KBI0_A> for bool {
    #[inline(always)]
    fn from(variant: KBI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KBI0`"]
pub type KBI0_R = crate::R<bool, KBI0_A>;
impl KBI0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KBI0_A {
        match self.bits {
            false => KBI0_A::_0,
            true => KBI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KBI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KBI0_A::_1
    }
}
#[doc = "Write proxy for field `KBI0`"]
pub struct KBI0_W<'a> {
    w: &'a mut W,
}
impl<'a> KBI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBI0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the KBI0 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBI0_A::_0)
    }
    #[doc = "Bus clock to the KBI0 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBI0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "KBI1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KBI1_A {
    #[doc = "0: Bus clock to the KBI1 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the KBI1 module is enabled."]
    _1 = 1,
}
impl From<KBI1_A> for bool {
    #[inline(always)]
    fn from(variant: KBI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KBI1`"]
pub type KBI1_R = crate::R<bool, KBI1_A>;
impl KBI1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KBI1_A {
        match self.bits {
            false => KBI1_A::_0,
            true => KBI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KBI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KBI1_A::_1
    }
}
#[doc = "Write proxy for field `KBI1`"]
pub struct KBI1_W<'a> {
    w: &'a mut W,
}
impl<'a> KBI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBI1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the KBI1 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBI1_A::_0)
    }
    #[doc = "Bus clock to the KBI1 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBI1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "IRQ Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ_A {
    #[doc = "0: Bus clock to the IRQ module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the IRQ module is enabled."]
    _1 = 1,
}
impl From<IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRQ`"]
pub type IRQ_R = crate::R<bool, IRQ_A>;
impl IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ_A {
        match self.bits {
            false => IRQ_A::_0,
            true => IRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQ_A::_1
    }
}
#[doc = "Write proxy for field `IRQ`"]
pub struct IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the IRQ module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQ_A::_0)
    }
    #[doc = "Bus clock to the IRQ module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "ADC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: Bus clock to the ADC module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the ADC module is enabled."]
    _1 = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<bool, ADC_A>;
impl ADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::_0,
            true => ADC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC_A::_1
    }
}
#[doc = "Write proxy for field `ADC`"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the ADC module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC_A::_0)
    }
    #[doc = "Bus clock to the ADC module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "ACMP0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP0_A {
    #[doc = "0: Bus clock to the ACMP0 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the ACMP0 module is enabled."]
    _1 = 1,
}
impl From<ACMP0_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMP0`"]
pub type ACMP0_R = crate::R<bool, ACMP0_A>;
impl ACMP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP0_A {
        match self.bits {
            false => ACMP0_A::_0,
            true => ACMP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACMP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACMP0_A::_1
    }
}
#[doc = "Write proxy for field `ACMP0`"]
pub struct ACMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the ACMP0 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMP0_A::_0)
    }
    #[doc = "Bus clock to the ACMP0 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMP0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "ACMP1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1_A {
    #[doc = "0: Bus clock to the ACMP1 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the ACMP1 module is enabled."]
    _1 = 1,
}
impl From<ACMP1_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMP1`"]
pub type ACMP1_R = crate::R<bool, ACMP1_A>;
impl ACMP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP1_A {
        match self.bits {
            false => ACMP1_A::_0,
            true => ACMP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACMP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACMP1_A::_1
    }
}
#[doc = "Write proxy for field `ACMP1`"]
pub struct ACMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock to the ACMP1 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMP1_A::_0)
    }
    #[doc = "Bus clock to the ACMP1 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMP1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Clock Gate Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&self) -> PIT_R {
        PIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWT Clock Gate Control"]
    #[inline(always)]
    pub fn pwt(&self) -> PWT_R {
        PWT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FTM0 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm0(&self) -> FTM0_R {
        FTM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FTM1 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm1(&self) -> FTM1_R {
        FTM1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&self) -> FTM2_R {
        FTM2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Flash Clock Gate Control"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SWD (single wire debugger) Clock Gate Control"]
    #[inline(always)]
    pub fn swd(&self) -> SWD_R {
        SWD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MSCAN Clock Gate Control"]
    #[inline(always)]
    pub fn mscan(&self) -> MSCAN_R {
        MSCAN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C0 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - I2C1 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SPI1 Clock Gate Control"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - UART0 Clock Gate Control"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - UART1 Clock Gate Control"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - UART2 Clock Gate Control"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - KBI0 Clock Gate Control"]
    #[inline(always)]
    pub fn kbi0(&self) -> KBI0_R {
        KBI0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - KBI1 Clock Gate Control"]
    #[inline(always)]
    pub fn kbi1(&self) -> KBI1_R {
        KBI1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IRQ Clock Gate Control"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ADC Clock Gate Control"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ACMP0 Clock Gate Control"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ACMP1 Clock Gate Control"]
    #[inline(always)]
    pub fn acmp1(&self) -> ACMP1_R {
        ACMP1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Clock Gate Control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 1 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&mut self) -> PIT_W {
        PIT_W { w: self }
    }
    #[doc = "Bit 4 - PWT Clock Gate Control"]
    #[inline(always)]
    pub fn pwt(&mut self) -> PWT_W {
        PWT_W { w: self }
    }
    #[doc = "Bit 5 - FTM0 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm0(&mut self) -> FTM0_W {
        FTM0_W { w: self }
    }
    #[doc = "Bit 6 - FTM1 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm1(&mut self) -> FTM1_W {
        FTM1_W { w: self }
    }
    #[doc = "Bit 7 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&mut self) -> FTM2_W {
        FTM2_W { w: self }
    }
    #[doc = "Bit 10 - CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 12 - Flash Clock Gate Control"]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W {
        FLASH_W { w: self }
    }
    #[doc = "Bit 13 - SWD (single wire debugger) Clock Gate Control"]
    #[inline(always)]
    pub fn swd(&mut self) -> SWD_W {
        SWD_W { w: self }
    }
    #[doc = "Bit 15 - MSCAN Clock Gate Control"]
    #[inline(always)]
    pub fn mscan(&mut self) -> MSCAN_W {
        MSCAN_W { w: self }
    }
    #[doc = "Bit 16 - I2C0 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W { w: self }
    }
    #[doc = "Bit 17 - I2C1 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W { w: self }
    }
    #[doc = "Bit 18 - SPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W {
        SPI0_W { w: self }
    }
    #[doc = "Bit 19 - SPI1 Clock Gate Control"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W {
        SPI1_W { w: self }
    }
    #[doc = "Bit 20 - UART0 Clock Gate Control"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W { w: self }
    }
    #[doc = "Bit 21 - UART1 Clock Gate Control"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W { w: self }
    }
    #[doc = "Bit 22 - UART2 Clock Gate Control"]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART2_W {
        UART2_W { w: self }
    }
    #[doc = "Bit 24 - KBI0 Clock Gate Control"]
    #[inline(always)]
    pub fn kbi0(&mut self) -> KBI0_W {
        KBI0_W { w: self }
    }
    #[doc = "Bit 25 - KBI1 Clock Gate Control"]
    #[inline(always)]
    pub fn kbi1(&mut self) -> KBI1_W {
        KBI1_W { w: self }
    }
    #[doc = "Bit 27 - IRQ Clock Gate Control"]
    #[inline(always)]
    pub fn irq(&mut self) -> IRQ_W {
        IRQ_W { w: self }
    }
    #[doc = "Bit 29 - ADC Clock Gate Control"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 30 - ACMP0 Clock Gate Control"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> ACMP0_W {
        ACMP0_W { w: self }
    }
    #[doc = "Bit 31 - ACMP1 Clock Gate Control"]
    #[inline(always)]
    pub fn acmp1(&mut self) -> ACMP1_W {
        ACMP1_W { w: self }
    }
}

#[doc = "Reader of register SPMSC1"]
pub type R = crate::R<u8, super::SPMSC1>;
#[doc = "Writer for register SPMSC1"]
pub type W = crate::W<u8, super::SPMSC1>;
#[doc = "Register SPMSC1 `reset()`'s with value 0x1c"]
impl crate::ResetValue for super::SPMSC1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1c
    }
}
#[doc = "Bandgap Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGBE_A {
    #[doc = "0: Bandgap buffer is disabled."]
    _0 = 0,
    #[doc = "1: Bandgap buffer is enabled."]
    _1 = 1,
}
impl From<BGBE_A> for bool {
    #[inline(always)]
    fn from(variant: BGBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BGBE`"]
pub type BGBE_R = crate::R<bool, BGBE_A>;
impl BGBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGBE_A {
        match self.bits {
            false => BGBE_A::_0,
            true => BGBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGBE_A::_1
    }
}
#[doc = "Write proxy for field `BGBE`"]
pub struct BGBE_W<'a> {
    w: &'a mut W,
}
impl<'a> BGBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGBE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bandgap buffer is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGBE_A::_0)
    }
    #[doc = "Bandgap buffer is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGBE_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Low-Voltage Detect Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDE_A {
    #[doc = "0: LVD logic is disabled."]
    _0 = 0,
    #[doc = "1: LVD logic is enabled."]
    _1 = 1,
}
impl From<LVDE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LVDE`"]
pub type LVDE_R = crate::R<bool, LVDE_A>;
impl LVDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDE_A {
        match self.bits {
            false => LVDE_A::_0,
            true => LVDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDE_A::_1
    }
}
#[doc = "Write proxy for field `LVDE`"]
pub struct LVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LVD logic is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDE_A::_0)
    }
    #[doc = "LVD logic is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Low-Voltage Detect Stop Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDSE_A {
    #[doc = "0: Low-voltage detect is disabled during Stop mode."]
    _0 = 0,
    #[doc = "1: Low-voltage detect is enabled during Stop mode."]
    _1 = 1,
}
impl From<LVDSE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LVDSE`"]
pub type LVDSE_R = crate::R<bool, LVDSE_A>;
impl LVDSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDSE_A {
        match self.bits {
            false => LVDSE_A::_0,
            true => LVDSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDSE_A::_1
    }
}
#[doc = "Write proxy for field `LVDSE`"]
pub struct LVDSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low-voltage detect is disabled during Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDSE_A::_0)
    }
    #[doc = "Low-voltage detect is enabled during Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDSE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Low-Voltage Detect Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDRE_A {
    #[doc = "0: LVD events do not generate hardware resets."]
    _0 = 0,
    #[doc = "1: Forces an MCU reset when an enabled low-voltage detect event occurs."]
    _1 = 1,
}
impl From<LVDRE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LVDRE`"]
pub type LVDRE_R = crate::R<bool, LVDRE_A>;
impl LVDRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDRE_A {
        match self.bits {
            false => LVDRE_A::_0,
            true => LVDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDRE_A::_1
    }
}
#[doc = "Write proxy for field `LVDRE`"]
pub struct LVDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LVD events do not generate hardware resets."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDRE_A::_0)
    }
    #[doc = "Forces an MCU reset when an enabled low-voltage detect event occurs."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDRE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Low-Voltage Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWIE_A {
    #[doc = "0: Hardware interrupt is disabled (use polling)."]
    _0 = 0,
    #[doc = "1: Requests a hardware interrupt when LVWF = 1."]
    _1 = 1,
}
impl From<LVWIE_A> for bool {
    #[inline(always)]
    fn from(variant: LVWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LVWIE`"]
pub type LVWIE_R = crate::R<bool, LVWIE_A>;
impl LVWIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWIE_A {
        match self.bits {
            false => LVWIE_A::_0,
            true => LVWIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVWIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVWIE_A::_1
    }
}
#[doc = "Write proxy for field `LVWIE`"]
pub struct LVWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVWIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupt is disabled (use polling)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVWIE_A::_0)
    }
    #[doc = "Requests a hardware interrupt when LVWF = 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVWIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `LVWACK`"]
pub struct LVWACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LVWACK_W<'a> {
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
#[doc = "Low-Voltage Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWF_A {
    #[doc = "0: Low-voltage warning is not present."]
    _0 = 0,
    #[doc = "1: Low-voltage warning is present or was present."]
    _1 = 1,
}
impl From<LVWF_A> for bool {
    #[inline(always)]
    fn from(variant: LVWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LVWF`"]
pub type LVWF_R = crate::R<bool, LVWF_A>;
impl LVWF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWF_A {
        match self.bits {
            false => LVWF_A::_0,
            true => LVWF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVWF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVWF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline(always)]
    pub fn bgbe(&self) -> BGBE_R {
        BGBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low-Voltage Detect Enable"]
    #[inline(always)]
    pub fn lvde(&self) -> LVDE_R {
        LVDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Low-Voltage Detect Stop Enable"]
    #[inline(always)]
    pub fn lvdse(&self) -> LVDSE_R {
        LVDSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Low-Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&self) -> LVDRE_R {
        LVDRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    pub fn lvwie(&self) -> LVWIE_R {
        LVWIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low-Voltage Warning Flag"]
    #[inline(always)]
    pub fn lvwf(&self) -> LVWF_R {
        LVWF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline(always)]
    pub fn bgbe(&mut self) -> BGBE_W {
        BGBE_W { w: self }
    }
    #[doc = "Bit 2 - Low-Voltage Detect Enable"]
    #[inline(always)]
    pub fn lvde(&mut self) -> LVDE_W {
        LVDE_W { w: self }
    }
    #[doc = "Bit 3 - Low-Voltage Detect Stop Enable"]
    #[inline(always)]
    pub fn lvdse(&mut self) -> LVDSE_W {
        LVDSE_W { w: self }
    }
    #[doc = "Bit 4 - Low-Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&mut self) -> LVDRE_W {
        LVDRE_W { w: self }
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    pub fn lvwie(&mut self) -> LVWIE_W {
        LVWIE_W { w: self }
    }
    #[doc = "Bit 6 - Low-Voltage Warning Acknowledge"]
    #[inline(always)]
    pub fn lvwack(&mut self) -> LVWACK_W {
        LVWACK_W { w: self }
    }
}

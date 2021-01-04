#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MSCAN Control Register 0"]
    pub canctl0: CANCTL0,
    #[doc = "0x01 - MSCAN Control Register 1"]
    pub canctl1: CANCTL1,
    #[doc = "0x02 - MSCAN Bus Timing Register 0"]
    pub canbtr0: CANBTR0,
    #[doc = "0x03 - MSCAN Bus Timing Register 1"]
    pub canbtr1: CANBTR1,
    #[doc = "0x04 - MSCAN Receiver Flag Register"]
    pub canrflg: CANRFLG,
    #[doc = "0x05 - MSCAN Receiver Interrupt Enable Register"]
    pub canrier: CANRIER,
    #[doc = "0x06 - MSCAN Transmitter Flag Register"]
    pub cantflg: CANTFLG,
    #[doc = "0x07 - MSCAN Transmitter Interrupt Enable Register"]
    pub cantier: CANTIER,
    #[doc = "0x08 - MSCAN Transmitter Message Abort Request Register"]
    pub cantarq: CANTARQ,
    #[doc = "0x09 - MSCAN Transmitter Message Abort Acknowledge Register"]
    pub cantaak: CANTAAK,
    #[doc = "0x0a - MSCAN Transmit Buffer Selection Register"]
    pub cantbsel: CANTBSEL,
    #[doc = "0x0b - MSCAN Identifier Acceptance Control Register"]
    pub canidac: CANIDAC,
    _reserved12: [u8; 1usize],
    #[doc = "0x0d - MSCAN Miscellaneous Register"]
    pub canmisc: CANMISC,
    #[doc = "0x0e - MSCAN Receive Error Counter"]
    pub canrxerr: CANRXERR,
    #[doc = "0x0f - MSCAN Transmit Error Counter"]
    pub cantxerr: CANTXERR,
    #[doc = "0x10 - MSCAN Identifier Acceptance Register n of First Bank"]
    pub canidar0: CANIDAR0,
    #[doc = "0x11 - MSCAN Identifier Acceptance Register n of First Bank"]
    pub canidar1: CANIDAR1,
    #[doc = "0x12 - MSCAN Identifier Acceptance Register n of First Bank"]
    pub canidar2: CANIDAR2,
    #[doc = "0x13 - MSCAN Identifier Acceptance Register n of First Bank"]
    pub canidar3: CANIDAR3,
    #[doc = "0x14 - MSCAN Identifier Mask Register n of First Bank"]
    pub canidmr0: CANIDMR0,
    #[doc = "0x15 - MSCAN Identifier Mask Register n of First Bank"]
    pub canidmr1: CANIDMR1,
    #[doc = "0x16 - MSCAN Identifier Mask Register n of First Bank"]
    pub canidmr2: CANIDMR2,
    #[doc = "0x17 - MSCAN Identifier Mask Register n of First Bank"]
    pub canidmr3: CANIDMR3,
    #[doc = "0x18 - MSCAN Identifier Acceptance Register n of Second Bank"]
    pub canidar4: CANIDAR4,
    #[doc = "0x19 - MSCAN Identifier Acceptance Register n of Second Bank"]
    pub canidar5: CANIDAR5,
    #[doc = "0x1a - MSCAN Identifier Acceptance Register n of Second Bank"]
    pub canidar6: CANIDAR6,
    #[doc = "0x1b - MSCAN Identifier Acceptance Register n of Second Bank"]
    pub canidar7: CANIDAR7,
    #[doc = "0x1c - MSCAN Identifier Mask Register n of Second Bank"]
    pub canidmr4: CANIDMR4,
    #[doc = "0x1d - MSCAN Identifier Mask Register n of Second Bank"]
    pub canidmr5: CANIDMR5,
    #[doc = "0x1e - MSCAN Identifier Mask Register n of Second Bank"]
    pub canidmr6: CANIDMR6,
    #[doc = "0x1f - MSCAN Identifier Mask Register n of Second Bank"]
    pub canidmr7: CANIDMR7,
    _reserved_31_reidr0: [u8; 1usize],
    _reserved_32_reidr1: [u8; 1usize],
    #[doc = "0x22 - Receive Extended Identifier Register 2"]
    pub reidr2: REIDR2,
    #[doc = "0x23 - Receive Extended Identifier Register 3"]
    pub reidr3: REIDR3,
    #[doc = "0x24 - Receive Extended Data Segment Register N"]
    pub redsr: [REDSR; 8],
    #[doc = "0x2c - Receive Data Length Register"]
    pub rdlr: RDLR,
    _reserved37: [u8; 1usize],
    #[doc = "0x2e - Receive Time Stamp Register High"]
    pub rtsrh: RTSRH,
    #[doc = "0x2f - Receive Time Stamp Register Low"]
    pub rtsrl: RTSRL,
    _reserved_39_teidr0: [u8; 1usize],
    _reserved_40_teidr1: [u8; 1usize],
    #[doc = "0x32 - Transmit Extended Identifier Register 2"]
    pub teidr2: TEIDR2,
    #[doc = "0x33 - Transmit Extended Identifier Register 3"]
    pub teidr3: TEIDR3,
    #[doc = "0x34 - Transmit Extended Data Segment Register N"]
    pub tedsr: [TEDSR; 8],
    #[doc = "0x3c - Transmit Data Length Register"]
    pub tdlr: TDLR,
    #[doc = "0x3d - Transmit Buffer Priority Register"]
    pub tbpr: TBPR,
    #[doc = "0x3e - Transmit Time Stamp Register High"]
    pub ttsrh: TTSRH,
    #[doc = "0x3f - Transmit Time Stamp Register Low"]
    pub ttsrl: TTSRL,
}
impl RegisterBlock {
    #[doc = "0x20 - Receive Standard Identifier Register 0"]
    #[inline(always)]
    pub fn rsidr0(&self) -> &RSIDR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const RSIDR0) }
    }
    #[doc = "0x20 - Receive Standard Identifier Register 0"]
    #[inline(always)]
    pub fn rsidr0_mut(&self) -> &mut RSIDR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut RSIDR0) }
    }
    #[doc = "0x20 - Receive Extended Identifier Register 0"]
    #[inline(always)]
    pub fn reidr0(&self) -> &REIDR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const REIDR0) }
    }
    #[doc = "0x20 - Receive Extended Identifier Register 0"]
    #[inline(always)]
    pub fn reidr0_mut(&self) -> &mut REIDR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut REIDR0) }
    }
    #[doc = "0x21 - Receive Standard Identifier Register 1"]
    #[inline(always)]
    pub fn rsidr1(&self) -> &RSIDR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(33usize) as *const RSIDR1) }
    }
    #[doc = "0x21 - Receive Standard Identifier Register 1"]
    #[inline(always)]
    pub fn rsidr1_mut(&self) -> &mut RSIDR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(33usize) as *mut RSIDR1) }
    }
    #[doc = "0x21 - Receive Extended Identifier Register 1"]
    #[inline(always)]
    pub fn reidr1(&self) -> &REIDR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(33usize) as *const REIDR1) }
    }
    #[doc = "0x21 - Receive Extended Identifier Register 1"]
    #[inline(always)]
    pub fn reidr1_mut(&self) -> &mut REIDR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(33usize) as *mut REIDR1) }
    }
    #[doc = "0x30 - Transmit Standard Identifier Register 0"]
    #[inline(always)]
    pub fn tsidr0(&self) -> &TSIDR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const TSIDR0) }
    }
    #[doc = "0x30 - Transmit Standard Identifier Register 0"]
    #[inline(always)]
    pub fn tsidr0_mut(&self) -> &mut TSIDR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut TSIDR0) }
    }
    #[doc = "0x30 - Transmit Extended Identifier Register 0"]
    #[inline(always)]
    pub fn teidr0(&self) -> &TEIDR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const TEIDR0) }
    }
    #[doc = "0x30 - Transmit Extended Identifier Register 0"]
    #[inline(always)]
    pub fn teidr0_mut(&self) -> &mut TEIDR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut TEIDR0) }
    }
    #[doc = "0x31 - Transmit Standard Identifier Register 1"]
    #[inline(always)]
    pub fn tsidr1(&self) -> &TSIDR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(49usize) as *const TSIDR1) }
    }
    #[doc = "0x31 - Transmit Standard Identifier Register 1"]
    #[inline(always)]
    pub fn tsidr1_mut(&self) -> &mut TSIDR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(49usize) as *mut TSIDR1) }
    }
    #[doc = "0x31 - Transmit Extended Identifier Register 1"]
    #[inline(always)]
    pub fn teidr1(&self) -> &TEIDR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(49usize) as *const TEIDR1) }
    }
    #[doc = "0x31 - Transmit Extended Identifier Register 1"]
    #[inline(always)]
    pub fn teidr1_mut(&self) -> &mut TEIDR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(49usize) as *mut TEIDR1) }
    }
}
#[doc = "MSCAN Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canctl0](canctl0) module"]
pub type CANCTL0 = crate::Reg<u8, _CANCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANCTL0;
#[doc = "`read()` method returns [canctl0::R](canctl0::R) reader structure"]
impl crate::Readable for CANCTL0 {}
#[doc = "`write(|w| ..)` method takes [canctl0::W](canctl0::W) writer structure"]
impl crate::Writable for CANCTL0 {}
#[doc = "MSCAN Control Register 0"]
pub mod canctl0;
#[doc = "MSCAN Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canctl1](canctl1) module"]
pub type CANCTL1 = crate::Reg<u8, _CANCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANCTL1;
#[doc = "`read()` method returns [canctl1::R](canctl1::R) reader structure"]
impl crate::Readable for CANCTL1 {}
#[doc = "`write(|w| ..)` method takes [canctl1::W](canctl1::W) writer structure"]
impl crate::Writable for CANCTL1 {}
#[doc = "MSCAN Control Register 1"]
pub mod canctl1;
#[doc = "MSCAN Bus Timing Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canbtr0](canbtr0) module"]
pub type CANBTR0 = crate::Reg<u8, _CANBTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANBTR0;
#[doc = "`read()` method returns [canbtr0::R](canbtr0::R) reader structure"]
impl crate::Readable for CANBTR0 {}
#[doc = "`write(|w| ..)` method takes [canbtr0::W](canbtr0::W) writer structure"]
impl crate::Writable for CANBTR0 {}
#[doc = "MSCAN Bus Timing Register 0"]
pub mod canbtr0;
#[doc = "MSCAN Bus Timing Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canbtr1](canbtr1) module"]
pub type CANBTR1 = crate::Reg<u8, _CANBTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANBTR1;
#[doc = "`read()` method returns [canbtr1::R](canbtr1::R) reader structure"]
impl crate::Readable for CANBTR1 {}
#[doc = "`write(|w| ..)` method takes [canbtr1::W](canbtr1::W) writer structure"]
impl crate::Writable for CANBTR1 {}
#[doc = "MSCAN Bus Timing Register 1"]
pub mod canbtr1;
#[doc = "MSCAN Receiver Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canrflg](canrflg) module"]
pub type CANRFLG = crate::Reg<u8, _CANRFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANRFLG;
#[doc = "`read()` method returns [canrflg::R](canrflg::R) reader structure"]
impl crate::Readable for CANRFLG {}
#[doc = "`write(|w| ..)` method takes [canrflg::W](canrflg::W) writer structure"]
impl crate::Writable for CANRFLG {}
#[doc = "MSCAN Receiver Flag Register"]
pub mod canrflg;
#[doc = "MSCAN Receiver Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canrier](canrier) module"]
pub type CANRIER = crate::Reg<u8, _CANRIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANRIER;
#[doc = "`read()` method returns [canrier::R](canrier::R) reader structure"]
impl crate::Readable for CANRIER {}
#[doc = "`write(|w| ..)` method takes [canrier::W](canrier::W) writer structure"]
impl crate::Writable for CANRIER {}
#[doc = "MSCAN Receiver Interrupt Enable Register"]
pub mod canrier;
#[doc = "MSCAN Transmitter Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cantflg](cantflg) module"]
pub type CANTFLG = crate::Reg<u8, _CANTFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANTFLG;
#[doc = "`read()` method returns [cantflg::R](cantflg::R) reader structure"]
impl crate::Readable for CANTFLG {}
#[doc = "`write(|w| ..)` method takes [cantflg::W](cantflg::W) writer structure"]
impl crate::Writable for CANTFLG {}
#[doc = "MSCAN Transmitter Flag Register"]
pub mod cantflg;
#[doc = "MSCAN Transmitter Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cantier](cantier) module"]
pub type CANTIER = crate::Reg<u8, _CANTIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANTIER;
#[doc = "`read()` method returns [cantier::R](cantier::R) reader structure"]
impl crate::Readable for CANTIER {}
#[doc = "`write(|w| ..)` method takes [cantier::W](cantier::W) writer structure"]
impl crate::Writable for CANTIER {}
#[doc = "MSCAN Transmitter Interrupt Enable Register"]
pub mod cantier;
#[doc = "MSCAN Transmitter Message Abort Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cantarq](cantarq) module"]
pub type CANTARQ = crate::Reg<u8, _CANTARQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANTARQ;
#[doc = "`read()` method returns [cantarq::R](cantarq::R) reader structure"]
impl crate::Readable for CANTARQ {}
#[doc = "`write(|w| ..)` method takes [cantarq::W](cantarq::W) writer structure"]
impl crate::Writable for CANTARQ {}
#[doc = "MSCAN Transmitter Message Abort Request Register"]
pub mod cantarq;
#[doc = "MSCAN Transmitter Message Abort Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cantaak](cantaak) module"]
pub type CANTAAK = crate::Reg<u8, _CANTAAK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANTAAK;
#[doc = "`read()` method returns [cantaak::R](cantaak::R) reader structure"]
impl crate::Readable for CANTAAK {}
#[doc = "MSCAN Transmitter Message Abort Acknowledge Register"]
pub mod cantaak;
#[doc = "MSCAN Transmit Buffer Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cantbsel](cantbsel) module"]
pub type CANTBSEL = crate::Reg<u8, _CANTBSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANTBSEL;
#[doc = "`read()` method returns [cantbsel::R](cantbsel::R) reader structure"]
impl crate::Readable for CANTBSEL {}
#[doc = "`write(|w| ..)` method takes [cantbsel::W](cantbsel::W) writer structure"]
impl crate::Writable for CANTBSEL {}
#[doc = "MSCAN Transmit Buffer Selection Register"]
pub mod cantbsel;
#[doc = "MSCAN Identifier Acceptance Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidac](canidac) module"]
pub type CANIDAC = crate::Reg<u8, _CANIDAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDAC;
#[doc = "`read()` method returns [canidac::R](canidac::R) reader structure"]
impl crate::Readable for CANIDAC {}
#[doc = "`write(|w| ..)` method takes [canidac::W](canidac::W) writer structure"]
impl crate::Writable for CANIDAC {}
#[doc = "MSCAN Identifier Acceptance Control Register"]
pub mod canidac;
#[doc = "MSCAN Miscellaneous Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canmisc](canmisc) module"]
pub type CANMISC = crate::Reg<u8, _CANMISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANMISC;
#[doc = "`read()` method returns [canmisc::R](canmisc::R) reader structure"]
impl crate::Readable for CANMISC {}
#[doc = "`write(|w| ..)` method takes [canmisc::W](canmisc::W) writer structure"]
impl crate::Writable for CANMISC {}
#[doc = "MSCAN Miscellaneous Register"]
pub mod canmisc;
#[doc = "MSCAN Receive Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canrxerr](canrxerr) module"]
pub type CANRXERR = crate::Reg<u8, _CANRXERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANRXERR;
#[doc = "`read()` method returns [canrxerr::R](canrxerr::R) reader structure"]
impl crate::Readable for CANRXERR {}
#[doc = "MSCAN Receive Error Counter"]
pub mod canrxerr;
#[doc = "MSCAN Transmit Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cantxerr](cantxerr) module"]
pub type CANTXERR = crate::Reg<u8, _CANTXERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANTXERR;
#[doc = "`read()` method returns [cantxerr::R](cantxerr::R) reader structure"]
impl crate::Readable for CANTXERR {}
#[doc = "MSCAN Transmit Error Counter"]
pub mod cantxerr;
#[doc = "MSCAN Identifier Acceptance Register n of First Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidar0](canidar0) module"]
pub type CANIDAR0 = crate::Reg<u8, _CANIDAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDAR0;
#[doc = "`read()` method returns [canidar0::R](canidar0::R) reader structure"]
impl crate::Readable for CANIDAR0 {}
#[doc = "`write(|w| ..)` method takes [canidar0::W](canidar0::W) writer structure"]
impl crate::Writable for CANIDAR0 {}
#[doc = "MSCAN Identifier Acceptance Register n of First Bank"]
pub mod canidar0;
#[doc = "MSCAN Identifier Acceptance Register n of First Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidar1](canidar1) module"]
pub type CANIDAR1 = crate::Reg<u8, _CANIDAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDAR1;
#[doc = "`read()` method returns [canidar1::R](canidar1::R) reader structure"]
impl crate::Readable for CANIDAR1 {}
#[doc = "`write(|w| ..)` method takes [canidar1::W](canidar1::W) writer structure"]
impl crate::Writable for CANIDAR1 {}
#[doc = "MSCAN Identifier Acceptance Register n of First Bank"]
pub mod canidar1;
#[doc = "MSCAN Identifier Acceptance Register n of First Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidar2](canidar2) module"]
pub type CANIDAR2 = crate::Reg<u8, _CANIDAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDAR2;
#[doc = "`read()` method returns [canidar2::R](canidar2::R) reader structure"]
impl crate::Readable for CANIDAR2 {}
#[doc = "`write(|w| ..)` method takes [canidar2::W](canidar2::W) writer structure"]
impl crate::Writable for CANIDAR2 {}
#[doc = "MSCAN Identifier Acceptance Register n of First Bank"]
pub mod canidar2;
#[doc = "MSCAN Identifier Acceptance Register n of First Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidar3](canidar3) module"]
pub type CANIDAR3 = crate::Reg<u8, _CANIDAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDAR3;
#[doc = "`read()` method returns [canidar3::R](canidar3::R) reader structure"]
impl crate::Readable for CANIDAR3 {}
#[doc = "`write(|w| ..)` method takes [canidar3::W](canidar3::W) writer structure"]
impl crate::Writable for CANIDAR3 {}
#[doc = "MSCAN Identifier Acceptance Register n of First Bank"]
pub mod canidar3;
#[doc = "MSCAN Identifier Mask Register n of First Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidmr0](canidmr0) module"]
pub type CANIDMR0 = crate::Reg<u8, _CANIDMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDMR0;
#[doc = "`read()` method returns [canidmr0::R](canidmr0::R) reader structure"]
impl crate::Readable for CANIDMR0 {}
#[doc = "`write(|w| ..)` method takes [canidmr0::W](canidmr0::W) writer structure"]
impl crate::Writable for CANIDMR0 {}
#[doc = "MSCAN Identifier Mask Register n of First Bank"]
pub mod canidmr0;
#[doc = "MSCAN Identifier Mask Register n of First Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidmr1](canidmr1) module"]
pub type CANIDMR1 = crate::Reg<u8, _CANIDMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDMR1;
#[doc = "`read()` method returns [canidmr1::R](canidmr1::R) reader structure"]
impl crate::Readable for CANIDMR1 {}
#[doc = "`write(|w| ..)` method takes [canidmr1::W](canidmr1::W) writer structure"]
impl crate::Writable for CANIDMR1 {}
#[doc = "MSCAN Identifier Mask Register n of First Bank"]
pub mod canidmr1;
#[doc = "MSCAN Identifier Mask Register n of First Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidmr2](canidmr2) module"]
pub type CANIDMR2 = crate::Reg<u8, _CANIDMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDMR2;
#[doc = "`read()` method returns [canidmr2::R](canidmr2::R) reader structure"]
impl crate::Readable for CANIDMR2 {}
#[doc = "`write(|w| ..)` method takes [canidmr2::W](canidmr2::W) writer structure"]
impl crate::Writable for CANIDMR2 {}
#[doc = "MSCAN Identifier Mask Register n of First Bank"]
pub mod canidmr2;
#[doc = "MSCAN Identifier Mask Register n of First Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidmr3](canidmr3) module"]
pub type CANIDMR3 = crate::Reg<u8, _CANIDMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDMR3;
#[doc = "`read()` method returns [canidmr3::R](canidmr3::R) reader structure"]
impl crate::Readable for CANIDMR3 {}
#[doc = "`write(|w| ..)` method takes [canidmr3::W](canidmr3::W) writer structure"]
impl crate::Writable for CANIDMR3 {}
#[doc = "MSCAN Identifier Mask Register n of First Bank"]
pub mod canidmr3;
#[doc = "MSCAN Identifier Acceptance Register n of Second Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidar4](canidar4) module"]
pub type CANIDAR4 = crate::Reg<u8, _CANIDAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDAR4;
#[doc = "`read()` method returns [canidar4::R](canidar4::R) reader structure"]
impl crate::Readable for CANIDAR4 {}
#[doc = "`write(|w| ..)` method takes [canidar4::W](canidar4::W) writer structure"]
impl crate::Writable for CANIDAR4 {}
#[doc = "MSCAN Identifier Acceptance Register n of Second Bank"]
pub mod canidar4;
#[doc = "MSCAN Identifier Acceptance Register n of Second Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidar5](canidar5) module"]
pub type CANIDAR5 = crate::Reg<u8, _CANIDAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDAR5;
#[doc = "`read()` method returns [canidar5::R](canidar5::R) reader structure"]
impl crate::Readable for CANIDAR5 {}
#[doc = "`write(|w| ..)` method takes [canidar5::W](canidar5::W) writer structure"]
impl crate::Writable for CANIDAR5 {}
#[doc = "MSCAN Identifier Acceptance Register n of Second Bank"]
pub mod canidar5;
#[doc = "MSCAN Identifier Acceptance Register n of Second Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidar6](canidar6) module"]
pub type CANIDAR6 = crate::Reg<u8, _CANIDAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDAR6;
#[doc = "`read()` method returns [canidar6::R](canidar6::R) reader structure"]
impl crate::Readable for CANIDAR6 {}
#[doc = "`write(|w| ..)` method takes [canidar6::W](canidar6::W) writer structure"]
impl crate::Writable for CANIDAR6 {}
#[doc = "MSCAN Identifier Acceptance Register n of Second Bank"]
pub mod canidar6;
#[doc = "MSCAN Identifier Acceptance Register n of Second Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidar7](canidar7) module"]
pub type CANIDAR7 = crate::Reg<u8, _CANIDAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDAR7;
#[doc = "`read()` method returns [canidar7::R](canidar7::R) reader structure"]
impl crate::Readable for CANIDAR7 {}
#[doc = "`write(|w| ..)` method takes [canidar7::W](canidar7::W) writer structure"]
impl crate::Writable for CANIDAR7 {}
#[doc = "MSCAN Identifier Acceptance Register n of Second Bank"]
pub mod canidar7;
#[doc = "MSCAN Identifier Mask Register n of Second Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidmr4](canidmr4) module"]
pub type CANIDMR4 = crate::Reg<u8, _CANIDMR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDMR4;
#[doc = "`read()` method returns [canidmr4::R](canidmr4::R) reader structure"]
impl crate::Readable for CANIDMR4 {}
#[doc = "`write(|w| ..)` method takes [canidmr4::W](canidmr4::W) writer structure"]
impl crate::Writable for CANIDMR4 {}
#[doc = "MSCAN Identifier Mask Register n of Second Bank"]
pub mod canidmr4;
#[doc = "MSCAN Identifier Mask Register n of Second Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidmr5](canidmr5) module"]
pub type CANIDMR5 = crate::Reg<u8, _CANIDMR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDMR5;
#[doc = "`read()` method returns [canidmr5::R](canidmr5::R) reader structure"]
impl crate::Readable for CANIDMR5 {}
#[doc = "`write(|w| ..)` method takes [canidmr5::W](canidmr5::W) writer structure"]
impl crate::Writable for CANIDMR5 {}
#[doc = "MSCAN Identifier Mask Register n of Second Bank"]
pub mod canidmr5;
#[doc = "MSCAN Identifier Mask Register n of Second Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidmr6](canidmr6) module"]
pub type CANIDMR6 = crate::Reg<u8, _CANIDMR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDMR6;
#[doc = "`read()` method returns [canidmr6::R](canidmr6::R) reader structure"]
impl crate::Readable for CANIDMR6 {}
#[doc = "`write(|w| ..)` method takes [canidmr6::W](canidmr6::W) writer structure"]
impl crate::Writable for CANIDMR6 {}
#[doc = "MSCAN Identifier Mask Register n of Second Bank"]
pub mod canidmr6;
#[doc = "MSCAN Identifier Mask Register n of Second Bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canidmr7](canidmr7) module"]
pub type CANIDMR7 = crate::Reg<u8, _CANIDMR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANIDMR7;
#[doc = "`read()` method returns [canidmr7::R](canidmr7::R) reader structure"]
impl crate::Readable for CANIDMR7 {}
#[doc = "`write(|w| ..)` method takes [canidmr7::W](canidmr7::W) writer structure"]
impl crate::Writable for CANIDMR7 {}
#[doc = "MSCAN Identifier Mask Register n of Second Bank"]
pub mod canidmr7;
#[doc = "Receive Extended Identifier Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reidr0](reidr0) module"]
pub type REIDR0 = crate::Reg<u8, _REIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REIDR0;
#[doc = "`read()` method returns [reidr0::R](reidr0::R) reader structure"]
impl crate::Readable for REIDR0 {}
#[doc = "Receive Extended Identifier Register 0"]
pub mod reidr0;
#[doc = "Receive Standard Identifier Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsidr0](rsidr0) module"]
pub type RSIDR0 = crate::Reg<u8, _RSIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSIDR0;
#[doc = "`read()` method returns [rsidr0::R](rsidr0::R) reader structure"]
impl crate::Readable for RSIDR0 {}
#[doc = "Receive Standard Identifier Register 0"]
pub mod rsidr0;
#[doc = "Receive Extended Identifier Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reidr1](reidr1) module"]
pub type REIDR1 = crate::Reg<u8, _REIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REIDR1;
#[doc = "`read()` method returns [reidr1::R](reidr1::R) reader structure"]
impl crate::Readable for REIDR1 {}
#[doc = "Receive Extended Identifier Register 1"]
pub mod reidr1;
#[doc = "Receive Standard Identifier Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsidr1](rsidr1) module"]
pub type RSIDR1 = crate::Reg<u8, _RSIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSIDR1;
#[doc = "`read()` method returns [rsidr1::R](rsidr1::R) reader structure"]
impl crate::Readable for RSIDR1 {}
#[doc = "Receive Standard Identifier Register 1"]
pub mod rsidr1;
#[doc = "Receive Extended Identifier Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reidr2](reidr2) module"]
pub type REIDR2 = crate::Reg<u8, _REIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REIDR2;
#[doc = "`read()` method returns [reidr2::R](reidr2::R) reader structure"]
impl crate::Readable for REIDR2 {}
#[doc = "Receive Extended Identifier Register 2"]
pub mod reidr2;
#[doc = "Receive Extended Identifier Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reidr3](reidr3) module"]
pub type REIDR3 = crate::Reg<u8, _REIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REIDR3;
#[doc = "`read()` method returns [reidr3::R](reidr3::R) reader structure"]
impl crate::Readable for REIDR3 {}
#[doc = "Receive Extended Identifier Register 3"]
pub mod reidr3;
#[doc = "Receive Extended Data Segment Register N\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [redsr](redsr) module"]
pub type REDSR = crate::Reg<u8, _REDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REDSR;
#[doc = "`read()` method returns [redsr::R](redsr::R) reader structure"]
impl crate::Readable for REDSR {}
#[doc = "Receive Extended Data Segment Register N"]
pub mod redsr;
#[doc = "Receive Data Length Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdlr](rdlr) module"]
pub type RDLR = crate::Reg<u8, _RDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDLR;
#[doc = "`read()` method returns [rdlr::R](rdlr::R) reader structure"]
impl crate::Readable for RDLR {}
#[doc = "Receive Data Length Register"]
pub mod rdlr;
#[doc = "Receive Time Stamp Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsrh](rtsrh) module"]
pub type RTSRH = crate::Reg<u8, _RTSRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSRH;
#[doc = "`read()` method returns [rtsrh::R](rtsrh::R) reader structure"]
impl crate::Readable for RTSRH {}
#[doc = "Receive Time Stamp Register High"]
pub mod rtsrh;
#[doc = "Receive Time Stamp Register Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsrl](rtsrl) module"]
pub type RTSRL = crate::Reg<u8, _RTSRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSRL;
#[doc = "`read()` method returns [rtsrl::R](rtsrl::R) reader structure"]
impl crate::Readable for RTSRL {}
#[doc = "Receive Time Stamp Register Low"]
pub mod rtsrl;
#[doc = "Transmit Extended Identifier Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [teidr0](teidr0) module"]
pub type TEIDR0 = crate::Reg<u8, _TEIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEIDR0;
#[doc = "`read()` method returns [teidr0::R](teidr0::R) reader structure"]
impl crate::Readable for TEIDR0 {}
#[doc = "`write(|w| ..)` method takes [teidr0::W](teidr0::W) writer structure"]
impl crate::Writable for TEIDR0 {}
#[doc = "Transmit Extended Identifier Register 0"]
pub mod teidr0;
#[doc = "Transmit Standard Identifier Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsidr0](tsidr0) module"]
pub type TSIDR0 = crate::Reg<u8, _TSIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSIDR0;
#[doc = "`read()` method returns [tsidr0::R](tsidr0::R) reader structure"]
impl crate::Readable for TSIDR0 {}
#[doc = "`write(|w| ..)` method takes [tsidr0::W](tsidr0::W) writer structure"]
impl crate::Writable for TSIDR0 {}
#[doc = "Transmit Standard Identifier Register 0"]
pub mod tsidr0;
#[doc = "Transmit Extended Identifier Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [teidr1](teidr1) module"]
pub type TEIDR1 = crate::Reg<u8, _TEIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEIDR1;
#[doc = "`read()` method returns [teidr1::R](teidr1::R) reader structure"]
impl crate::Readable for TEIDR1 {}
#[doc = "`write(|w| ..)` method takes [teidr1::W](teidr1::W) writer structure"]
impl crate::Writable for TEIDR1 {}
#[doc = "Transmit Extended Identifier Register 1"]
pub mod teidr1;
#[doc = "Transmit Standard Identifier Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsidr1](tsidr1) module"]
pub type TSIDR1 = crate::Reg<u8, _TSIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSIDR1;
#[doc = "`read()` method returns [tsidr1::R](tsidr1::R) reader structure"]
impl crate::Readable for TSIDR1 {}
#[doc = "`write(|w| ..)` method takes [tsidr1::W](tsidr1::W) writer structure"]
impl crate::Writable for TSIDR1 {}
#[doc = "Transmit Standard Identifier Register 1"]
pub mod tsidr1;
#[doc = "Transmit Extended Identifier Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [teidr2](teidr2) module"]
pub type TEIDR2 = crate::Reg<u8, _TEIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEIDR2;
#[doc = "`read()` method returns [teidr2::R](teidr2::R) reader structure"]
impl crate::Readable for TEIDR2 {}
#[doc = "`write(|w| ..)` method takes [teidr2::W](teidr2::W) writer structure"]
impl crate::Writable for TEIDR2 {}
#[doc = "Transmit Extended Identifier Register 2"]
pub mod teidr2;
#[doc = "Transmit Extended Identifier Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [teidr3](teidr3) module"]
pub type TEIDR3 = crate::Reg<u8, _TEIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEIDR3;
#[doc = "`read()` method returns [teidr3::R](teidr3::R) reader structure"]
impl crate::Readable for TEIDR3 {}
#[doc = "`write(|w| ..)` method takes [teidr3::W](teidr3::W) writer structure"]
impl crate::Writable for TEIDR3 {}
#[doc = "Transmit Extended Identifier Register 3"]
pub mod teidr3;
#[doc = "Transmit Extended Data Segment Register N\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tedsr](tedsr) module"]
pub type TEDSR = crate::Reg<u8, _TEDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEDSR;
#[doc = "`read()` method returns [tedsr::R](tedsr::R) reader structure"]
impl crate::Readable for TEDSR {}
#[doc = "`write(|w| ..)` method takes [tedsr::W](tedsr::W) writer structure"]
impl crate::Writable for TEDSR {}
#[doc = "Transmit Extended Data Segment Register N"]
pub mod tedsr;
#[doc = "Transmit Data Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdlr](tdlr) module"]
pub type TDLR = crate::Reg<u8, _TDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDLR;
#[doc = "`read()` method returns [tdlr::R](tdlr::R) reader structure"]
impl crate::Readable for TDLR {}
#[doc = "`write(|w| ..)` method takes [tdlr::W](tdlr::W) writer structure"]
impl crate::Writable for TDLR {}
#[doc = "Transmit Data Length Register"]
pub mod tdlr;
#[doc = "Transmit Buffer Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpr](tbpr) module"]
pub type TBPR = crate::Reg<u8, _TBPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPR;
#[doc = "`read()` method returns [tbpr::R](tbpr::R) reader structure"]
impl crate::Readable for TBPR {}
#[doc = "`write(|w| ..)` method takes [tbpr::W](tbpr::W) writer structure"]
impl crate::Writable for TBPR {}
#[doc = "Transmit Buffer Priority Register"]
pub mod tbpr;
#[doc = "Transmit Time Stamp Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttsrh](ttsrh) module"]
pub type TTSRH = crate::Reg<u8, _TTSRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTSRH;
#[doc = "`read()` method returns [ttsrh::R](ttsrh::R) reader structure"]
impl crate::Readable for TTSRH {}
#[doc = "Transmit Time Stamp Register High"]
pub mod ttsrh;
#[doc = "Transmit Time Stamp Register Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttsrl](ttsrl) module"]
pub type TTSRL = crate::Reg<u8, _TTSRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTSRL;
#[doc = "`read()` method returns [ttsrl::R](ttsrl::R) reader structure"]
impl crate::Readable for TTSRL {}
#[doc = "Transmit Time Stamp Register Low"]
pub mod ttsrl;

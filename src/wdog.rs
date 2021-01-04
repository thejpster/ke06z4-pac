#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Control and Status Register 1"]
    pub cs1: CS1,
    #[doc = "0x01 - Watchdog Control and Status Register 2"]
    pub cs2: CS2,
    #[doc = "0x02 - Watchdog Counter Register: High"]
    pub cnth: CNTH,
    #[doc = "0x03 - Watchdog Counter Register: Low"]
    pub cntl: CNTL,
    #[doc = "0x04 - Watchdog Timeout Value Register: High"]
    pub tovalh: TOVALH,
    #[doc = "0x05 - Watchdog Timeout Value Register: Low"]
    pub tovall: TOVALL,
    #[doc = "0x06 - Watchdog Window Register: High"]
    pub winh: WINH,
    #[doc = "0x07 - Watchdog Window Register: Low"]
    pub winl: WINL,
}
#[doc = "Watchdog Control and Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs1](cs1) module"]
pub type CS1 = crate::Reg<u8, _CS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS1;
#[doc = "`read()` method returns [cs1::R](cs1::R) reader structure"]
impl crate::Readable for CS1 {}
#[doc = "`write(|w| ..)` method takes [cs1::W](cs1::W) writer structure"]
impl crate::Writable for CS1 {}
#[doc = "Watchdog Control and Status Register 1"]
pub mod cs1;
#[doc = "Watchdog Control and Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs2](cs2) module"]
pub type CS2 = crate::Reg<u8, _CS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS2;
#[doc = "`read()` method returns [cs2::R](cs2::R) reader structure"]
impl crate::Readable for CS2 {}
#[doc = "`write(|w| ..)` method takes [cs2::W](cs2::W) writer structure"]
impl crate::Writable for CS2 {}
#[doc = "Watchdog Control and Status Register 2"]
pub mod cs2;
#[doc = "Watchdog Counter Register: High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnth](cnth) module"]
pub type CNTH = crate::Reg<u8, _CNTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTH;
#[doc = "`read()` method returns [cnth::R](cnth::R) reader structure"]
impl crate::Readable for CNTH {}
#[doc = "`write(|w| ..)` method takes [cnth::W](cnth::W) writer structure"]
impl crate::Writable for CNTH {}
#[doc = "Watchdog Counter Register: High"]
pub mod cnth;
#[doc = "Watchdog Counter Register: Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl](cntl) module"]
pub type CNTL = crate::Reg<u8, _CNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTL;
#[doc = "`read()` method returns [cntl::R](cntl::R) reader structure"]
impl crate::Readable for CNTL {}
#[doc = "`write(|w| ..)` method takes [cntl::W](cntl::W) writer structure"]
impl crate::Writable for CNTL {}
#[doc = "Watchdog Counter Register: Low"]
pub mod cntl;
#[doc = "Watchdog Timeout Value Register: High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tovalh](tovalh) module"]
pub type TOVALH = crate::Reg<u8, _TOVALH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOVALH;
#[doc = "`read()` method returns [tovalh::R](tovalh::R) reader structure"]
impl crate::Readable for TOVALH {}
#[doc = "`write(|w| ..)` method takes [tovalh::W](tovalh::W) writer structure"]
impl crate::Writable for TOVALH {}
#[doc = "Watchdog Timeout Value Register: High"]
pub mod tovalh;
#[doc = "Watchdog Timeout Value Register: Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tovall](tovall) module"]
pub type TOVALL = crate::Reg<u8, _TOVALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOVALL;
#[doc = "`read()` method returns [tovall::R](tovall::R) reader structure"]
impl crate::Readable for TOVALL {}
#[doc = "`write(|w| ..)` method takes [tovall::W](tovall::W) writer structure"]
impl crate::Writable for TOVALL {}
#[doc = "Watchdog Timeout Value Register: Low"]
pub mod tovall;
#[doc = "Watchdog Window Register: High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winh](winh) module"]
pub type WINH = crate::Reg<u8, _WINH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINH;
#[doc = "`read()` method returns [winh::R](winh::R) reader structure"]
impl crate::Readable for WINH {}
#[doc = "`write(|w| ..)` method takes [winh::W](winh::W) writer structure"]
impl crate::Writable for WINH {}
#[doc = "Watchdog Window Register: High"]
pub mod winh;
#[doc = "Watchdog Window Register: Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winl](winl) module"]
pub type WINL = crate::Reg<u8, _WINL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINL;
#[doc = "`read()` method returns [winl::R](winl::R) reader structure"]
impl crate::Readable for WINL {}
#[doc = "`write(|w| ..)` method takes [winl::W](winl::W) writer structure"]
impl crate::Writable for WINL {}
#[doc = "Watchdog Window Register: Low"]
pub mod winl;

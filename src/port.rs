#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Filter Register 0"]
    pub ioflt0: IOFLT0,
    #[doc = "0x04 - Port Filter Register 1"]
    pub ioflt1: IOFLT1,
    #[doc = "0x08 - Port Pullup Enable Register 0"]
    pub pue0: PUE0,
    #[doc = "0x0c - Port Pullup Enable Register 1"]
    pub pue1: PUE1,
    #[doc = "0x10 - Port Pullup Enable Register 2"]
    pub pue2: PUE2,
    #[doc = "0x14 - Port High Drive Enable Register"]
    pub hdrve: HDRVE,
}
#[doc = "Port Filter Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioflt0](ioflt0) module"]
pub type IOFLT0 = crate::Reg<u32, _IOFLT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOFLT0;
#[doc = "`read()` method returns [ioflt0::R](ioflt0::R) reader structure"]
impl crate::Readable for IOFLT0 {}
#[doc = "`write(|w| ..)` method takes [ioflt0::W](ioflt0::W) writer structure"]
impl crate::Writable for IOFLT0 {}
#[doc = "Port Filter Register 0"]
pub mod ioflt0;
#[doc = "Port Filter Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioflt1](ioflt1) module"]
pub type IOFLT1 = crate::Reg<u32, _IOFLT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOFLT1;
#[doc = "`read()` method returns [ioflt1::R](ioflt1::R) reader structure"]
impl crate::Readable for IOFLT1 {}
#[doc = "`write(|w| ..)` method takes [ioflt1::W](ioflt1::W) writer structure"]
impl crate::Writable for IOFLT1 {}
#[doc = "Port Filter Register 1"]
pub mod ioflt1;
#[doc = "Port Pullup Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pue0](pue0) module"]
pub type PUE0 = crate::Reg<u32, _PUE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUE0;
#[doc = "`read()` method returns [pue0::R](pue0::R) reader structure"]
impl crate::Readable for PUE0 {}
#[doc = "`write(|w| ..)` method takes [pue0::W](pue0::W) writer structure"]
impl crate::Writable for PUE0 {}
#[doc = "Port Pullup Enable Register 0"]
pub mod pue0;
#[doc = "Port Pullup Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pue1](pue1) module"]
pub type PUE1 = crate::Reg<u32, _PUE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUE1;
#[doc = "`read()` method returns [pue1::R](pue1::R) reader structure"]
impl crate::Readable for PUE1 {}
#[doc = "`write(|w| ..)` method takes [pue1::W](pue1::W) writer structure"]
impl crate::Writable for PUE1 {}
#[doc = "Port Pullup Enable Register 1"]
pub mod pue1;
#[doc = "Port Pullup Enable Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pue2](pue2) module"]
pub type PUE2 = crate::Reg<u32, _PUE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUE2;
#[doc = "`read()` method returns [pue2::R](pue2::R) reader structure"]
impl crate::Readable for PUE2 {}
#[doc = "`write(|w| ..)` method takes [pue2::W](pue2::W) writer structure"]
impl crate::Writable for PUE2 {}
#[doc = "Port Pullup Enable Register 2"]
pub mod pue2;
#[doc = "Port High Drive Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdrve](hdrve) module"]
pub type HDRVE = crate::Reg<u32, _HDRVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDRVE;
#[doc = "`read()` method returns [hdrve::R](hdrve::R) reader structure"]
impl crate::Readable for HDRVE {}
#[doc = "`write(|w| ..)` method takes [hdrve::W](hdrve::W) writer structure"]
impl crate::Writable for HDRVE {}
#[doc = "Port High Drive Enable Register"]
pub mod hdrve;

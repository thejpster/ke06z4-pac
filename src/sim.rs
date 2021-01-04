#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status and ID Register"]
    pub srsid: SRSID,
    #[doc = "0x04 - System Options Register 0"]
    pub sopt0: SOPT0,
    #[doc = "0x08 - System Options Register"]
    pub sopt1: SOPT1,
    #[doc = "0x0c - Pin Selection Register 0"]
    pub pinsel0: PINSEL0,
    #[doc = "0x10 - Pin Selection Register 1"]
    pub pinsel1: PINSEL1,
    #[doc = "0x14 - System Clock Gating Control Register"]
    pub scgc: SCGC,
    #[doc = "0x18 - Universally Unique Identifier Low Register"]
    pub uuidl: UUIDL,
    #[doc = "0x1c - Universally Unique Identifier Middle Low Register"]
    pub uuidml: UUIDML,
    #[doc = "0x20 - Universally Unique Identifier Middle High Register"]
    pub uuidmh: UUIDMH,
    #[doc = "0x24 - Clock Divider Register"]
    pub clkdiv: CLKDIV,
}
#[doc = "System Reset Status and ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srsid](srsid) module"]
pub type SRSID = crate::Reg<u32, _SRSID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSID;
#[doc = "`read()` method returns [srsid::R](srsid::R) reader structure"]
impl crate::Readable for SRSID {}
#[doc = "System Reset Status and ID Register"]
pub mod srsid;
#[doc = "System Options Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt0](sopt0) module"]
pub type SOPT0 = crate::Reg<u32, _SOPT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT0;
#[doc = "`read()` method returns [sopt0::R](sopt0::R) reader structure"]
impl crate::Readable for SOPT0 {}
#[doc = "`write(|w| ..)` method takes [sopt0::W](sopt0::W) writer structure"]
impl crate::Writable for SOPT0 {}
#[doc = "System Options Register 0"]
pub mod sopt0;
#[doc = "System Options Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt1](sopt1) module"]
pub type SOPT1 = crate::Reg<u32, _SOPT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT1;
#[doc = "`read()` method returns [sopt1::R](sopt1::R) reader structure"]
impl crate::Readable for SOPT1 {}
#[doc = "`write(|w| ..)` method takes [sopt1::W](sopt1::W) writer structure"]
impl crate::Writable for SOPT1 {}
#[doc = "System Options Register"]
pub mod sopt1;
#[doc = "Pin Selection Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel0](pinsel0) module"]
pub type PINSEL0 = crate::Reg<u32, _PINSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL0;
#[doc = "`read()` method returns [pinsel0::R](pinsel0::R) reader structure"]
impl crate::Readable for PINSEL0 {}
#[doc = "`write(|w| ..)` method takes [pinsel0::W](pinsel0::W) writer structure"]
impl crate::Writable for PINSEL0 {}
#[doc = "Pin Selection Register 0"]
pub mod pinsel0;
#[doc = "Pin Selection Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel1](pinsel1) module"]
pub type PINSEL1 = crate::Reg<u32, _PINSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL1;
#[doc = "`read()` method returns [pinsel1::R](pinsel1::R) reader structure"]
impl crate::Readable for PINSEL1 {}
#[doc = "`write(|w| ..)` method takes [pinsel1::W](pinsel1::W) writer structure"]
impl crate::Writable for PINSEL1 {}
#[doc = "Pin Selection Register 1"]
pub mod pinsel1;
#[doc = "System Clock Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc](scgc) module"]
pub type SCGC = crate::Reg<u32, _SCGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC;
#[doc = "`read()` method returns [scgc::R](scgc::R) reader structure"]
impl crate::Readable for SCGC {}
#[doc = "`write(|w| ..)` method takes [scgc::W](scgc::W) writer structure"]
impl crate::Writable for SCGC {}
#[doc = "System Clock Gating Control Register"]
pub mod scgc;
#[doc = "Universally Unique Identifier Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuidl](uuidl) module"]
pub type UUIDL = crate::Reg<u32, _UUIDL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UUIDL;
#[doc = "`read()` method returns [uuidl::R](uuidl::R) reader structure"]
impl crate::Readable for UUIDL {}
#[doc = "Universally Unique Identifier Low Register"]
pub mod uuidl;
#[doc = "Universally Unique Identifier Middle Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuidml](uuidml) module"]
pub type UUIDML = crate::Reg<u32, _UUIDML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UUIDML;
#[doc = "`read()` method returns [uuidml::R](uuidml::R) reader structure"]
impl crate::Readable for UUIDML {}
#[doc = "Universally Unique Identifier Middle Low Register"]
pub mod uuidml;
#[doc = "Universally Unique Identifier Middle High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuidmh](uuidmh) module"]
pub type UUIDMH = crate::Reg<u32, _UUIDMH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UUIDMH;
#[doc = "`read()` method returns [uuidmh::R](uuidmh::R) reader structure"]
impl crate::Readable for UUIDMH {}
#[doc = "Universally Unique Identifier Middle High Register"]
pub mod uuidmh;
#[doc = "Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](clkdiv) module"]
pub type CLKDIV = crate::Reg<u32, _CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV;
#[doc = "`read()` method returns [clkdiv::R](clkdiv::R) reader structure"]
impl crate::Readable for CLKDIV {}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](clkdiv::W) writer structure"]
impl crate::Writable for CLKDIV {}
#[doc = "Clock Divider Register"]
pub mod clkdiv;

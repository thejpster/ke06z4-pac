#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Power Management Status and Control 1 Register"]
    pub spmsc1: SPMSC1,
    #[doc = "0x01 - System Power Management Status and Control 2 Register"]
    pub spmsc2: SPMSC2,
}
#[doc = "System Power Management Status and Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spmsc1](spmsc1) module"]
pub type SPMSC1 = crate::Reg<u8, _SPMSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPMSC1;
#[doc = "`read()` method returns [spmsc1::R](spmsc1::R) reader structure"]
impl crate::Readable for SPMSC1 {}
#[doc = "`write(|w| ..)` method takes [spmsc1::W](spmsc1::W) writer structure"]
impl crate::Writable for SPMSC1 {}
#[doc = "System Power Management Status and Control 1 Register"]
pub mod spmsc1;
#[doc = "System Power Management Status and Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spmsc2](spmsc2) module"]
pub type SPMSC2 = crate::Reg<u8, _SPMSC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPMSC2;
#[doc = "`read()` method returns [spmsc2::R](spmsc2::R) reader structure"]
impl crate::Readable for SPMSC2 {}
#[doc = "`write(|w| ..)` method takes [spmsc2::W](spmsc2::W) writer structure"]
impl crate::Writable for SPMSC2 {}
#[doc = "System Power Management Status and Control 2 Register"]
pub mod spmsc2;

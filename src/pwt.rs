#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pulse Width Timer Register 1"]
    pub r1: R1,
    #[doc = "0x04 - Pulse Width Timer Register 2"]
    pub r2: R2,
}
#[doc = "Pulse Width Timer Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1](r1) module"]
pub type R1 = crate::Reg<u32, _R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1;
#[doc = "`read()` method returns [r1::R](r1::R) reader structure"]
impl crate::Readable for R1 {}
#[doc = "`write(|w| ..)` method takes [r1::W](r1::W) writer structure"]
impl crate::Writable for R1 {}
#[doc = "Pulse Width Timer Register 1"]
pub mod r1;
#[doc = "Pulse Width Timer Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2](r2) module"]
pub type R2 = crate::Reg<u32, _R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2;
#[doc = "`read()` method returns [r2::R](r2::R) reader structure"]
impl crate::Readable for R2 {}
#[doc = "Pulse Width Timer Register 2"]
pub mod r2;

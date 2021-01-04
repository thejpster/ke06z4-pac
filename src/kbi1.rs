#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - KBI Pin Enable Register"]
    pub pe: PE,
    #[doc = "0x04 - KBI Edge Select Register"]
    pub es: ES,
    #[doc = "0x08 - KBI Status and Control Register"]
    pub sc: SC,
    #[doc = "0x0c - KBI Source Pin Register"]
    pub sp: SP,
}
#[doc = "KBI Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe](pe) module"]
pub type PE = crate::Reg<u32, _PE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE;
#[doc = "`read()` method returns [pe::R](pe::R) reader structure"]
impl crate::Readable for PE {}
#[doc = "`write(|w| ..)` method takes [pe::W](pe::W) writer structure"]
impl crate::Writable for PE {}
#[doc = "KBI Pin Enable Register"]
pub mod pe;
#[doc = "KBI Edge Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [es](es) module"]
pub type ES = crate::Reg<u32, _ES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ES;
#[doc = "`read()` method returns [es::R](es::R) reader structure"]
impl crate::Readable for ES {}
#[doc = "`write(|w| ..)` method takes [es::W](es::W) writer structure"]
impl crate::Writable for ES {}
#[doc = "KBI Edge Select Register"]
pub mod es;
#[doc = "KBI Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](sc) module"]
pub type SC = crate::Reg<u32, _SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC;
#[doc = "`read()` method returns [sc::R](sc::R) reader structure"]
impl crate::Readable for SC {}
#[doc = "`write(|w| ..)` method takes [sc::W](sc::W) writer structure"]
impl crate::Writable for SC {}
#[doc = "KBI Status and Control Register"]
pub mod sc;
#[doc = "KBI Source Pin Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sp](sp) module"]
pub type SP = crate::Reg<u32, _SP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SP;
#[doc = "`read()` method returns [sp::R](sp::R) reader structure"]
impl crate::Readable for SP {}
#[doc = "KBI Source Pin Register"]
pub mod sp;

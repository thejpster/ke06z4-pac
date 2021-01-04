#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control Register 1"]
    pub sc1: SC1,
    #[doc = "0x04 - Status and Control Register 2"]
    pub sc2: SC2,
    #[doc = "0x08 - Status and Control Register 3"]
    pub sc3: SC3,
    #[doc = "0x0c - Status and Control Register 4"]
    pub sc4: SC4,
    #[doc = "0x10 - Conversion Result Register"]
    pub r: R,
    #[doc = "0x14 - Compare Value Register"]
    pub cv: CV,
    #[doc = "0x18 - Pin Control 1 Register"]
    pub apctl1: APCTL1,
    #[doc = "0x1c - Status and Control Register 5"]
    pub sc5: SC5,
}
#[doc = "Status and Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc1](sc1) module"]
pub type SC1 = crate::Reg<u32, _SC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC1;
#[doc = "`read()` method returns [sc1::R](sc1::R) reader structure"]
impl crate::Readable for SC1 {}
#[doc = "`write(|w| ..)` method takes [sc1::W](sc1::W) writer structure"]
impl crate::Writable for SC1 {}
#[doc = "Status and Control Register 1"]
pub mod sc1;
#[doc = "Status and Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc2](sc2) module"]
pub type SC2 = crate::Reg<u32, _SC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC2;
#[doc = "`read()` method returns [sc2::R](sc2::R) reader structure"]
impl crate::Readable for SC2 {}
#[doc = "`write(|w| ..)` method takes [sc2::W](sc2::W) writer structure"]
impl crate::Writable for SC2 {}
#[doc = "Status and Control Register 2"]
pub mod sc2;
#[doc = "Status and Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc3](sc3) module"]
pub type SC3 = crate::Reg<u32, _SC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC3;
#[doc = "`read()` method returns [sc3::R](sc3::R) reader structure"]
impl crate::Readable for SC3 {}
#[doc = "`write(|w| ..)` method takes [sc3::W](sc3::W) writer structure"]
impl crate::Writable for SC3 {}
#[doc = "Status and Control Register 3"]
pub mod sc3;
#[doc = "Status and Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc4](sc4) module"]
pub type SC4 = crate::Reg<u32, _SC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC4;
#[doc = "`read()` method returns [sc4::R](sc4::R) reader structure"]
impl crate::Readable for SC4 {}
#[doc = "`write(|w| ..)` method takes [sc4::W](sc4::W) writer structure"]
impl crate::Writable for SC4 {}
#[doc = "Status and Control Register 4"]
pub mod sc4;
#[doc = "Conversion Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r](r) module"]
pub type R = crate::Reg<u32, _R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R;
#[doc = "`read()` method returns [r::R](r::R) reader structure"]
impl crate::Readable for R {}
#[doc = "Conversion Result Register"]
pub mod r;
#[doc = "Compare Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](cv) module"]
pub type CV = crate::Reg<u32, _CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV;
#[doc = "`read()` method returns [cv::R](cv::R) reader structure"]
impl crate::Readable for CV {}
#[doc = "`write(|w| ..)` method takes [cv::W](cv::W) writer structure"]
impl crate::Writable for CV {}
#[doc = "Compare Value Register"]
pub mod cv;
#[doc = "Pin Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apctl1](apctl1) module"]
pub type APCTL1 = crate::Reg<u32, _APCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APCTL1;
#[doc = "`read()` method returns [apctl1::R](apctl1::R) reader structure"]
impl crate::Readable for APCTL1 {}
#[doc = "`write(|w| ..)` method takes [apctl1::W](apctl1::W) writer structure"]
impl crate::Writable for APCTL1 {}
#[doc = "Pin Control 1 Register"]
pub mod apctl1;
#[doc = "Status and Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc5](sc5) module"]
pub type SC5 = crate::Reg<u32, _SC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC5;
#[doc = "`read()` method returns [sc5::R](sc5::R) reader structure"]
impl crate::Readable for SC5 {}
#[doc = "`write(|w| ..)` method takes [sc5::W](sc5::W) writer structure"]
impl crate::Writable for SC5 {}
#[doc = "Status and Control Register 5"]
pub mod sc5;

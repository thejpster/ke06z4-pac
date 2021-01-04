#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ICS Control Register 1"]
    pub c1: C1,
    #[doc = "0x01 - ICS Control Register 2"]
    pub c2: C2,
    #[doc = "0x02 - ICS Control Register 3"]
    pub c3: C3,
    #[doc = "0x03 - ICS Control Register 4"]
    pub c4: C4,
    #[doc = "0x04 - ICS Status Register"]
    pub s: S,
}
#[doc = "ICS Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](c1) module"]
pub type C1 = crate::Reg<u8, _C1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1;
#[doc = "`read()` method returns [c1::R](c1::R) reader structure"]
impl crate::Readable for C1 {}
#[doc = "`write(|w| ..)` method takes [c1::W](c1::W) writer structure"]
impl crate::Writable for C1 {}
#[doc = "ICS Control Register 1"]
pub mod c1;
#[doc = "ICS Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](c2) module"]
pub type C2 = crate::Reg<u8, _C2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2;
#[doc = "`read()` method returns [c2::R](c2::R) reader structure"]
impl crate::Readable for C2 {}
#[doc = "`write(|w| ..)` method takes [c2::W](c2::W) writer structure"]
impl crate::Writable for C2 {}
#[doc = "ICS Control Register 2"]
pub mod c2;
#[doc = "ICS Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3](c3) module"]
pub type C3 = crate::Reg<u8, _C3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3;
#[doc = "`read()` method returns [c3::R](c3::R) reader structure"]
impl crate::Readable for C3 {}
#[doc = "`write(|w| ..)` method takes [c3::W](c3::W) writer structure"]
impl crate::Writable for C3 {}
#[doc = "ICS Control Register 3"]
pub mod c3;
#[doc = "ICS Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4](c4) module"]
pub type C4 = crate::Reg<u8, _C4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4;
#[doc = "`read()` method returns [c4::R](c4::R) reader structure"]
impl crate::Readable for C4 {}
#[doc = "`write(|w| ..)` method takes [c4::W](c4::W) writer structure"]
impl crate::Writable for C4 {}
#[doc = "ICS Control Register 4"]
pub mod c4;
#[doc = "ICS Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s](s) module"]
pub type S = crate::Reg<u8, _S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S;
#[doc = "`read()` method returns [s::R](s::R) reader structure"]
impl crate::Readable for S {}
#[doc = "`write(|w| ..)` method takes [s::W](s::W) writer structure"]
impl crate::Writable for S {}
#[doc = "ICS Status Register"]
pub mod s;

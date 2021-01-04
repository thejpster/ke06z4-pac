#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Flash CCOB Index Register"]
    pub fccobix: FCCOBIX,
    #[doc = "0x02 - Flash Security Register"]
    pub fsec: FSEC,
    #[doc = "0x03 - Flash Clock Divider Register"]
    pub fclkdiv: FCLKDIV,
    _reserved3: [u8; 1usize],
    #[doc = "0x05 - Flash Status Register"]
    pub fstat: FSTAT,
    _reserved4: [u8; 1usize],
    #[doc = "0x07 - Flash Configuration Register"]
    pub fcnfg: FCNFG,
    #[doc = "0x08 - Flash Common Command Object Register: Low"]
    pub fccoblo: FCCOBLO,
    #[doc = "0x09 - Flash Common Command Object Register:High"]
    pub fccobhi: FCCOBHI,
    _reserved7: [u8; 1usize],
    #[doc = "0x0b - Flash Protection Register"]
    pub fprot: FPROT,
    _reserved8: [u8; 3usize],
    #[doc = "0x0f - Flash Option Register"]
    pub fopt: FOPT,
}
#[doc = "Flash CCOB Index Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccobix](fccobix) module"]
pub type FCCOBIX = crate::Reg<u8, _FCCOBIX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOBIX;
#[doc = "`read()` method returns [fccobix::R](fccobix::R) reader structure"]
impl crate::Readable for FCCOBIX {}
#[doc = "`write(|w| ..)` method takes [fccobix::W](fccobix::W) writer structure"]
impl crate::Writable for FCCOBIX {}
#[doc = "Flash CCOB Index Register"]
pub mod fccobix;
#[doc = "Flash Security Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsec](fsec) module"]
pub type FSEC = crate::Reg<u8, _FSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSEC;
#[doc = "`read()` method returns [fsec::R](fsec::R) reader structure"]
impl crate::Readable for FSEC {}
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "Flash Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fclkdiv](fclkdiv) module"]
pub type FCLKDIV = crate::Reg<u8, _FCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCLKDIV;
#[doc = "`read()` method returns [fclkdiv::R](fclkdiv::R) reader structure"]
impl crate::Readable for FCLKDIV {}
#[doc = "`write(|w| ..)` method takes [fclkdiv::W](fclkdiv::W) writer structure"]
impl crate::Writable for FCLKDIV {}
#[doc = "Flash Clock Divider Register"]
pub mod fclkdiv;
#[doc = "Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fstat](fstat) module"]
pub type FSTAT = crate::Reg<u8, _FSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSTAT;
#[doc = "`read()` method returns [fstat::R](fstat::R) reader structure"]
impl crate::Readable for FSTAT {}
#[doc = "`write(|w| ..)` method takes [fstat::W](fstat::W) writer structure"]
impl crate::Writable for FSTAT {}
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcnfg](fcnfg) module"]
pub type FCNFG = crate::Reg<u8, _FCNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCNFG;
#[doc = "`read()` method returns [fcnfg::R](fcnfg::R) reader structure"]
impl crate::Readable for FCNFG {}
#[doc = "`write(|w| ..)` method takes [fcnfg::W](fcnfg::W) writer structure"]
impl crate::Writable for FCNFG {}
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "Flash Common Command Object Register: Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccoblo](fccoblo) module"]
pub type FCCOBLO = crate::Reg<u8, _FCCOBLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOBLO;
#[doc = "`read()` method returns [fccoblo::R](fccoblo::R) reader structure"]
impl crate::Readable for FCCOBLO {}
#[doc = "`write(|w| ..)` method takes [fccoblo::W](fccoblo::W) writer structure"]
impl crate::Writable for FCCOBLO {}
#[doc = "Flash Common Command Object Register: Low"]
pub mod fccoblo;
#[doc = "Flash Common Command Object Register:High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccobhi](fccobhi) module"]
pub type FCCOBHI = crate::Reg<u8, _FCCOBHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOBHI;
#[doc = "`read()` method returns [fccobhi::R](fccobhi::R) reader structure"]
impl crate::Readable for FCCOBHI {}
#[doc = "`write(|w| ..)` method takes [fccobhi::W](fccobhi::W) writer structure"]
impl crate::Writable for FCCOBHI {}
#[doc = "Flash Common Command Object Register:High"]
pub mod fccobhi;
#[doc = "Flash Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot](fprot) module"]
pub type FPROT = crate::Reg<u8, _FPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPROT;
#[doc = "`read()` method returns [fprot::R](fprot::R) reader structure"]
impl crate::Readable for FPROT {}
#[doc = "`write(|w| ..)` method takes [fprot::W](fprot::W) writer structure"]
impl crate::Writable for FPROT {}
#[doc = "Flash Protection Register"]
pub mod fprot;
#[doc = "Flash Option Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fopt](fopt) module"]
pub type FOPT = crate::Reg<u8, _FOPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FOPT;
#[doc = "`read()` method returns [fopt::R](fopt::R) reader structure"]
impl crate::Readable for FOPT {}
#[doc = "Flash Option Register"]
pub mod fopt;

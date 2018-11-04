#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RSTCU_GRSR"]
    pub rstcu_grsr: RSTCU_GRSR,
    #[doc = "0x04 - RSTCU_AHBPRSTR"]
    pub rstcu_ahbprstr: RSTCU_AHBPRSTR,
    #[doc = "0x08 - RSTCU_APBPRSTR0"]
    pub rstcu_apbprstr0: RSTCU_APBPRSTR0,
    #[doc = "0x0c - RSTCU_APBPRSTR1"]
    pub rstcu_apbprstr1: RSTCU_APBPRSTR1,
}
#[doc = "RSTCU_GRSR"]
pub struct RSTCU_GRSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSTCU_GRSR"]
pub mod rstcu_grsr;
#[doc = "RSTCU_AHBPRSTR"]
pub struct RSTCU_AHBPRSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSTCU_AHBPRSTR"]
pub mod rstcu_ahbprstr;
#[doc = "RSTCU_APBPRSTR0"]
pub struct RSTCU_APBPRSTR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSTCU_APBPRSTR0"]
pub mod rstcu_apbprstr0;
#[doc = "RSTCU_APBPRSTR1"]
pub struct RSTCU_APBPRSTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSTCU_APBPRSTR1"]
pub mod rstcu_apbprstr1;

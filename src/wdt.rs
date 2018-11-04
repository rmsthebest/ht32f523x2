#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT_CR"]
    pub wdt_cr: WDT_CR,
    #[doc = "0x04 - WDT_MR0"]
    pub wdt_mr0: WDT_MR0,
    #[doc = "0x08 - WDT_MR1"]
    pub wdt_mr1: WDT_MR1,
    #[doc = "0x0c - WDT_SR"]
    pub wdt_sr: WDT_SR,
    #[doc = "0x10 - WDT_PR"]
    pub wdt_pr: WDT_PR,
}
#[doc = "WDT_CR"]
pub struct WDT_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT_CR"]
pub mod wdt_cr;
#[doc = "WDT_MR0"]
pub struct WDT_MR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT_MR0"]
pub mod wdt_mr0;
#[doc = "WDT_MR1"]
pub struct WDT_MR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT_MR1"]
pub mod wdt_mr1;
#[doc = "WDT_SR"]
pub struct WDT_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT_SR"]
pub mod wdt_sr;
#[doc = "WDT_PR"]
pub struct WDT_PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT_PR"]
pub mod wdt_pr;

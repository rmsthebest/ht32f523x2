#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_CNT"]
    pub rtc_cnt: RTC_CNT,
    #[doc = "0x04 - RTC_CMP"]
    pub rtc_cmp: RTC_CMP,
    #[doc = "0x08 - RTC_CR"]
    pub rtc_cr: RTC_CR,
    #[doc = "0x0c - RTC_SR"]
    pub rtc_sr: RTC_SR,
    #[doc = "0x10 - RTC_IWEN"]
    pub rtc_iwen: RTC_IWEN,
}
#[doc = "RTC_CNT"]
pub struct RTC_CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC_CNT"]
pub mod rtc_cnt;
#[doc = "RTC_CMP"]
pub struct RTC_CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC_CMP"]
pub mod rtc_cmp;
#[doc = "RTC_CR"]
pub struct RTC_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC_CR"]
pub mod rtc_cr;
#[doc = "RTC_SR"]
pub struct RTC_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC_SR"]
pub mod rtc_sr;
#[doc = "RTC_IWEN"]
pub struct RTC_IWEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC_IWEN"]
pub mod rtc_iwen;

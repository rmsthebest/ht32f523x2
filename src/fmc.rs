#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FMC_TADR"]
    pub fmc_tadr: FMC_TADR,
    #[doc = "0x04 - FMC_WRDR"]
    pub fmc_wrdr: FMC_WRDR,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - FMC_OCMR"]
    pub fmc_ocmr: FMC_OCMR,
    #[doc = "0x10 - FMC_OPCR"]
    pub fmc_opcr: FMC_OPCR,
    #[doc = "0x14 - FMC_OIER"]
    pub fmc_oier: FMC_OIER,
    #[doc = "0x18 - FMC_OISR"]
    pub fmc_oisr: FMC_OISR,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - FMC_PPSR0"]
    pub fmc_ppsr0: FMC_PPSR0,
    #[doc = "0x24 - FMC_PPSR1"]
    pub fmc_ppsr1: FMC_PPSR1,
    #[doc = "0x28 - FMC_PPSR2"]
    pub fmc_ppsr2: FMC_PPSR2,
    #[doc = "0x2c - FMC_PPSR3"]
    pub fmc_ppsr3: FMC_PPSR3,
    #[doc = "0x30 - FMC_CPSR"]
    pub fmc_cpsr: FMC_CPSR,
    _reserved2: [u8; 204usize],
    #[doc = "0x100 - FMC_VMCR"]
    pub fmc_vmcr: FMC_VMCR,
    _reserved3: [u8; 124usize],
    #[doc = "0x180 - FMC_MDID"]
    pub fmc_mdid: FMC_MDID,
    #[doc = "0x184 - FMC_PNSR"]
    pub fmc_pnsr: FMC_PNSR,
    #[doc = "0x188 - FMC_PSSR"]
    pub fmc_pssr: FMC_PSSR,
    _reserved4: [u8; 116usize],
    #[doc = "0x200 - FMC_CFCR"]
    pub fmc_cfcr: FMC_CFCR,
    _reserved5: [u8; 268usize],
    #[doc = "0x310 - FMC_CID0"]
    pub fmc_cid0: FMC_CID0,
    #[doc = "0x314 - FMC_CID1"]
    pub fmc_cid1: FMC_CID1,
    #[doc = "0x318 - FMC_CID2"]
    pub fmc_cid2: FMC_CID2,
    #[doc = "0x31c - FMC_CID3"]
    pub fmc_cid3: FMC_CID3,
}
#[doc = "FMC_TADR"]
pub struct FMC_TADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_TADR"]
pub mod fmc_tadr;
#[doc = "FMC_WRDR"]
pub struct FMC_WRDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_WRDR"]
pub mod fmc_wrdr;
#[doc = "FMC_OCMR"]
pub struct FMC_OCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_OCMR"]
pub mod fmc_ocmr;
#[doc = "FMC_OPCR"]
pub struct FMC_OPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_OPCR"]
pub mod fmc_opcr;
#[doc = "FMC_OIER"]
pub struct FMC_OIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_OIER"]
pub mod fmc_oier;
#[doc = "FMC_OISR"]
pub struct FMC_OISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_OISR"]
pub mod fmc_oisr;
#[doc = "FMC_PPSR0"]
pub struct FMC_PPSR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_PPSR0"]
pub mod fmc_ppsr0;
#[doc = "FMC_PPSR1"]
pub struct FMC_PPSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_PPSR1"]
pub mod fmc_ppsr1;
#[doc = "FMC_PPSR2"]
pub struct FMC_PPSR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_PPSR2"]
pub mod fmc_ppsr2;
#[doc = "FMC_PPSR3"]
pub struct FMC_PPSR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_PPSR3"]
pub mod fmc_ppsr3;
#[doc = "FMC_CPSR"]
pub struct FMC_CPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_CPSR"]
pub mod fmc_cpsr;
#[doc = "FMC_VMCR"]
pub struct FMC_VMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_VMCR"]
pub mod fmc_vmcr;
#[doc = "FMC_MDID"]
pub struct FMC_MDID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_MDID"]
pub mod fmc_mdid;
#[doc = "FMC_PNSR"]
pub struct FMC_PNSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_PNSR"]
pub mod fmc_pnsr;
#[doc = "FMC_PSSR"]
pub struct FMC_PSSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_PSSR"]
pub mod fmc_pssr;
#[doc = "FMC_CFCR"]
pub struct FMC_CFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_CFCR"]
pub mod fmc_cfcr;
#[doc = "FMC_CID0"]
pub struct FMC_CID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_CID0"]
pub mod fmc_cid0;
#[doc = "FMC_CID1"]
pub struct FMC_CID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_CID1"]
pub mod fmc_cid1;
#[doc = "FMC_CID2"]
pub struct FMC_CID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_CID2"]
pub mod fmc_cid2;
#[doc = "FMC_CID3"]
pub struct FMC_CID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC_CID3"]
pub mod fmc_cid3;

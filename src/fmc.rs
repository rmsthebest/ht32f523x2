#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TADR"]
    pub tadr: TADR,
    #[doc = "0x04 - WRDR"]
    pub wrdr: WRDR,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - OCMR"]
    pub ocmr: OCMR,
    #[doc = "0x10 - OPCR"]
    pub opcr: OPCR,
    #[doc = "0x14 - OIER"]
    pub oier: OIER,
    #[doc = "0x18 - OISR"]
    pub oisr: OISR,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - PPSR0"]
    pub ppsr0: PPSR0,
    #[doc = "0x24 - PPSR1"]
    pub ppsr1: PPSR1,
    #[doc = "0x28 - PPSR2"]
    pub ppsr2: PPSR2,
    #[doc = "0x2c - PPSR3"]
    pub ppsr3: PPSR3,
    #[doc = "0x30 - CPSR"]
    pub cpsr: CPSR,
    _reserved2: [u8; 204usize],
    #[doc = "0x100 - VMCR"]
    pub vmcr: VMCR,
    _reserved3: [u8; 124usize],
    #[doc = "0x180 - MDID"]
    pub mdid: MDID,
    #[doc = "0x184 - PNSR"]
    pub pnsr: PNSR,
    #[doc = "0x188 - PSSR"]
    pub pssr: PSSR,
    _reserved4: [u8; 116usize],
    #[doc = "0x200 - CFCR"]
    pub cfcr: CFCR,
    _reserved5: [u8; 268usize],
    #[doc = "0x310 - CID0"]
    pub cid0: CID0,
    #[doc = "0x314 - CID1"]
    pub cid1: CID1,
    #[doc = "0x318 - CID2"]
    pub cid2: CID2,
    #[doc = "0x31c - CID3"]
    pub cid3: CID3,
}
#[doc = "TADR"]
pub struct TADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TADR"]
pub mod tadr;
#[doc = "WRDR"]
pub struct WRDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WRDR"]
pub mod wrdr;
#[doc = "OCMR"]
pub struct OCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OCMR"]
pub mod ocmr;
#[doc = "OPCR"]
pub struct OPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OPCR"]
pub mod opcr;
#[doc = "OIER"]
pub struct OIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OIER"]
pub mod oier;
#[doc = "OISR"]
pub struct OISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OISR"]
pub mod oisr;
#[doc = "PPSR0"]
pub struct PPSR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PPSR0"]
pub mod ppsr0;
#[doc = "PPSR1"]
pub struct PPSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PPSR1"]
pub mod ppsr1;
#[doc = "PPSR2"]
pub struct PPSR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PPSR2"]
pub mod ppsr2;
#[doc = "PPSR3"]
pub struct PPSR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PPSR3"]
pub mod ppsr3;
#[doc = "CPSR"]
pub struct CPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPSR"]
pub mod cpsr;
#[doc = "VMCR"]
pub struct VMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VMCR"]
pub mod vmcr;
#[doc = "MDID"]
pub struct MDID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDID"]
pub mod mdid;
#[doc = "PNSR"]
pub struct PNSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PNSR"]
pub mod pnsr;
#[doc = "PSSR"]
pub struct PSSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSSR"]
pub mod pssr;
#[doc = "CFCR"]
pub struct CFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFCR"]
pub mod cfcr;
#[doc = "CID0"]
pub struct CID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CID0"]
pub mod cid0;
#[doc = "CID1"]
pub struct CID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CID1"]
pub mod cid1;
#[doc = "CID2"]
pub struct CID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CID2"]
pub mod cid2;
#[doc = "CID3"]
pub struct CID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CID3"]
pub mod cid3;

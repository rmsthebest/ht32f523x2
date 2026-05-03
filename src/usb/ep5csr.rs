#[doc = "Register `EP5CSR` reader"]
pub type R = crate::R<Ep5csrSpec>;
#[doc = "Register `EP5CSR` writer"]
pub type W = crate::W<Ep5csrSpec>;
#[doc = "Field `DTGTX` reader - DTGTX"]
pub type DtgtxR = crate::BitReader;
#[doc = "Field `DTGTX` writer - DTGTX"]
pub type DtgtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKTX` reader - NAKTX"]
pub type NaktxR = crate::BitReader;
#[doc = "Field `NAKTX` writer - NAKTX"]
pub type NaktxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STLTX` reader - STLTX"]
pub type StltxR = crate::BitReader;
#[doc = "Field `STLTX` writer - STLTX"]
pub type StltxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTGRX` reader - DTGRX"]
pub type DtgrxR = crate::BitReader;
#[doc = "Field `DTGRX` writer - DTGRX"]
pub type DtgrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKRX` reader - NAKRX"]
pub type NakrxR = crate::BitReader;
#[doc = "Field `NAKRX` writer - NAKRX"]
pub type NakrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STLRX` reader - STLRX"]
pub type StlrxR = crate::BitReader;
#[doc = "Field `STLRX` writer - STLRX"]
pub type StlrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDBTG` reader - MDBTG"]
pub type MdbtgR = crate::BitReader;
#[doc = "Field `MDBTG` writer - MDBTG"]
pub type MdbtgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDBTG` reader - UDBTG"]
pub type UdbtgR = crate::BitReader;
#[doc = "Field `UDBTG` writer - UDBTG"]
pub type UdbtgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTGTX"]
    #[inline(always)]
    pub fn dtgtx(&self) -> DtgtxR {
        DtgtxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NAKTX"]
    #[inline(always)]
    pub fn naktx(&self) -> NaktxR {
        NaktxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STLTX"]
    #[inline(always)]
    pub fn stltx(&self) -> StltxR {
        StltxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTGRX"]
    #[inline(always)]
    pub fn dtgrx(&self) -> DtgrxR {
        DtgrxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKRX"]
    #[inline(always)]
    pub fn nakrx(&self) -> NakrxR {
        NakrxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STLRX"]
    #[inline(always)]
    pub fn stlrx(&self) -> StlrxR {
        StlrxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MDBTG"]
    #[inline(always)]
    pub fn mdbtg(&self) -> MdbtgR {
        MdbtgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UDBTG"]
    #[inline(always)]
    pub fn udbtg(&self) -> UdbtgR {
        UdbtgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTGTX"]
    #[inline(always)]
    pub fn dtgtx(&mut self) -> DtgtxW<'_, Ep5csrSpec> {
        DtgtxW::new(self, 0)
    }
    #[doc = "Bit 1 - NAKTX"]
    #[inline(always)]
    pub fn naktx(&mut self) -> NaktxW<'_, Ep5csrSpec> {
        NaktxW::new(self, 1)
    }
    #[doc = "Bit 2 - STLTX"]
    #[inline(always)]
    pub fn stltx(&mut self) -> StltxW<'_, Ep5csrSpec> {
        StltxW::new(self, 2)
    }
    #[doc = "Bit 3 - DTGRX"]
    #[inline(always)]
    pub fn dtgrx(&mut self) -> DtgrxW<'_, Ep5csrSpec> {
        DtgrxW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKRX"]
    #[inline(always)]
    pub fn nakrx(&mut self) -> NakrxW<'_, Ep5csrSpec> {
        NakrxW::new(self, 4)
    }
    #[doc = "Bit 5 - STLRX"]
    #[inline(always)]
    pub fn stlrx(&mut self) -> StlrxW<'_, Ep5csrSpec> {
        StlrxW::new(self, 5)
    }
    #[doc = "Bit 6 - MDBTG"]
    #[inline(always)]
    pub fn mdbtg(&mut self) -> MdbtgW<'_, Ep5csrSpec> {
        MdbtgW::new(self, 6)
    }
    #[doc = "Bit 7 - UDBTG"]
    #[inline(always)]
    pub fn udbtg(&mut self) -> UdbtgW<'_, Ep5csrSpec> {
        UdbtgW::new(self, 7)
    }
}
#[doc = "EP5CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ep5csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep5csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep5csrSpec;
impl crate::RegisterSpec for Ep5csrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep5csr::R`](R) reader structure"]
impl crate::Readable for Ep5csrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep5csr::W`](W) writer structure"]
impl crate::Writable for Ep5csrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP5CSR to value 0"]
impl crate::Resettable for Ep5csrSpec {}

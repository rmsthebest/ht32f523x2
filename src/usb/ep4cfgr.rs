#[doc = "Register `EP4CFGR` reader"]
pub type R = crate::R<Ep4cfgrSpec>;
#[doc = "Register `EP4CFGR` writer"]
pub type W = crate::W<Ep4cfgrSpec>;
#[doc = "Field `EPBUFA` reader - EPBUFA"]
pub type EpbufaR = crate::FieldReader<u16>;
#[doc = "Field `EPBUFA` writer - EPBUFA"]
pub type EpbufaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `EPLEN` reader - EPLEN"]
pub type EplenR = crate::FieldReader<u16>;
#[doc = "Field `EPLEN` writer - EPLEN"]
pub type EplenW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SDBS` reader - SDBS"]
pub type SdbsR = crate::BitReader;
#[doc = "Field `SDBS` writer - SDBS"]
pub type SdbsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPADR` reader - EPADR"]
pub type EpadrR = crate::FieldReader;
#[doc = "Field `EPADR` writer - EPADR"]
pub type EpadrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPDIR` reader - EPDIR"]
pub type EpdirR = crate::BitReader;
#[doc = "Field `EPDIR` writer - EPDIR"]
pub type EpdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYPE` reader - EPTYPE"]
pub type EptypeR = crate::BitReader;
#[doc = "Field `EPTYPE` writer - EPTYPE"]
pub type EptypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEN` reader - EPEN"]
pub type EpenR = crate::BitReader;
#[doc = "Field `EPEN` writer - EPEN"]
pub type EpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - EPBUFA"]
    #[inline(always)]
    pub fn epbufa(&self) -> EpbufaR {
        EpbufaR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - EPLEN"]
    #[inline(always)]
    pub fn eplen(&self) -> EplenR {
        EplenR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bit 23 - SDBS"]
    #[inline(always)]
    pub fn sdbs(&self) -> SdbsR {
        SdbsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - EPADR"]
    #[inline(always)]
    pub fn epadr(&self) -> EpadrR {
        EpadrR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - EPDIR"]
    #[inline(always)]
    pub fn epdir(&self) -> EpdirR {
        EpdirR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EPTYPE"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - EPEN"]
    #[inline(always)]
    pub fn epen(&self) -> EpenR {
        EpenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - EPBUFA"]
    #[inline(always)]
    #[must_use]
    pub fn epbufa(&mut self) -> EpbufaW<Ep4cfgrSpec> {
        EpbufaW::new(self, 0)
    }
    #[doc = "Bits 10:19 - EPLEN"]
    #[inline(always)]
    #[must_use]
    pub fn eplen(&mut self) -> EplenW<Ep4cfgrSpec> {
        EplenW::new(self, 10)
    }
    #[doc = "Bit 23 - SDBS"]
    #[inline(always)]
    #[must_use]
    pub fn sdbs(&mut self) -> SdbsW<Ep4cfgrSpec> {
        SdbsW::new(self, 23)
    }
    #[doc = "Bits 24:27 - EPADR"]
    #[inline(always)]
    #[must_use]
    pub fn epadr(&mut self) -> EpadrW<Ep4cfgrSpec> {
        EpadrW::new(self, 24)
    }
    #[doc = "Bit 28 - EPDIR"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EpdirW<Ep4cfgrSpec> {
        EpdirW::new(self, 28)
    }
    #[doc = "Bit 29 - EPTYPE"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EptypeW<Ep4cfgrSpec> {
        EptypeW::new(self, 29)
    }
    #[doc = "Bit 31 - EPEN"]
    #[inline(always)]
    #[must_use]
    pub fn epen(&mut self) -> EpenW<Ep4cfgrSpec> {
        EpenW::new(self, 31)
    }
}
#[doc = "EP4CFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep4cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep4cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep4cfgrSpec;
impl crate::RegisterSpec for Ep4cfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep4cfgr::R`](R) reader structure"]
impl crate::Readable for Ep4cfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep4cfgr::W`](W) writer structure"]
impl crate::Writable for Ep4cfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP4CFGR to value 0"]
impl crate::Resettable for Ep4cfgrSpec {
    const RESET_VALUE: u32 = 0;
}

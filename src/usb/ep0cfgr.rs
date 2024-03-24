#[doc = "Register `EP0CFGR` reader"]
pub type R = crate::R<Ep0cfgrSpec>;
#[doc = "Register `EP0CFGR` writer"]
pub type W = crate::W<Ep0cfgrSpec>;
#[doc = "Field `EPBUFA` reader - EPBUFA"]
pub type EpbufaR = crate::FieldReader<u16>;
#[doc = "Field `EPBUFA` writer - EPBUFA"]
pub type EpbufaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `EPLEN` reader - EPLEN"]
pub type EplenR = crate::FieldReader;
#[doc = "Field `EPLEN` writer - EPLEN"]
pub type EplenW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EPADR` reader - EPADR"]
pub type EpadrR = crate::FieldReader;
#[doc = "Field `EPADR` writer - EPADR"]
pub type EpadrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bits 10:16 - EPLEN"]
    #[inline(always)]
    pub fn eplen(&self) -> EplenR {
        EplenR::new(((self.bits >> 10) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - EPADR"]
    #[inline(always)]
    pub fn epadr(&self) -> EpadrR {
        EpadrR::new(((self.bits >> 24) & 0x0f) as u8)
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
    pub fn epbufa(&mut self) -> EpbufaW<Ep0cfgrSpec> {
        EpbufaW::new(self, 0)
    }
    #[doc = "Bits 10:16 - EPLEN"]
    #[inline(always)]
    #[must_use]
    pub fn eplen(&mut self) -> EplenW<Ep0cfgrSpec> {
        EplenW::new(self, 10)
    }
    #[doc = "Bits 24:27 - EPADR"]
    #[inline(always)]
    #[must_use]
    pub fn epadr(&mut self) -> EpadrW<Ep0cfgrSpec> {
        EpadrW::new(self, 24)
    }
    #[doc = "Bit 31 - EPEN"]
    #[inline(always)]
    #[must_use]
    pub fn epen(&mut self) -> EpenW<Ep0cfgrSpec> {
        EpenW::new(self, 31)
    }
}
#[doc = "EP0CFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep0cfgrSpec;
impl crate::RegisterSpec for Ep0cfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep0cfgr::R`](R) reader structure"]
impl crate::Readable for Ep0cfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep0cfgr::W`](W) writer structure"]
impl crate::Writable for Ep0cfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP0CFGR to value 0"]
impl crate::Resettable for Ep0cfgrSpec {
    const RESET_VALUE: u32 = 0;
}

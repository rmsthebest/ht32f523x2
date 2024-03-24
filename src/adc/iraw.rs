#[doc = "Register `IRAW` reader"]
pub type R = crate::R<IrawSpec>;
#[doc = "Register `IRAW` writer"]
pub type W = crate::W<IrawSpec>;
#[doc = "Field `ADIRAWS` reader - ADIRAWS"]
pub type AdirawsR = crate::BitReader;
#[doc = "Field `ADIRAWS` writer - ADIRAWS"]
pub type AdirawsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIRAWG` reader - ADIRAWG"]
pub type AdirawgR = crate::BitReader;
#[doc = "Field `ADIRAWG` writer - ADIRAWG"]
pub type AdirawgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIRAWC` reader - ADIRAWC"]
pub type AdirawcR = crate::BitReader;
#[doc = "Field `ADIRAWC` writer - ADIRAWC"]
pub type AdirawcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIRAWL` reader - ADIRAWL"]
pub type AdirawlR = crate::BitReader;
#[doc = "Field `ADIRAWL` writer - ADIRAWL"]
pub type AdirawlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIRAWU` reader - ADIRAWU"]
pub type AdirawuR = crate::BitReader;
#[doc = "Field `ADIRAWU` writer - ADIRAWU"]
pub type AdirawuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIRAWO` reader - ADIRAWO"]
pub type AdirawoR = crate::BitReader;
#[doc = "Field `ADIRAWO` writer - ADIRAWO"]
pub type AdirawoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADIRAWS"]
    #[inline(always)]
    pub fn adiraws(&self) -> AdirawsR {
        AdirawsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADIRAWG"]
    #[inline(always)]
    pub fn adirawg(&self) -> AdirawgR {
        AdirawgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADIRAWC"]
    #[inline(always)]
    pub fn adirawc(&self) -> AdirawcR {
        AdirawcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - ADIRAWL"]
    #[inline(always)]
    pub fn adirawl(&self) -> AdirawlR {
        AdirawlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADIRAWU"]
    #[inline(always)]
    pub fn adirawu(&self) -> AdirawuR {
        AdirawuR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADIRAWO"]
    #[inline(always)]
    pub fn adirawo(&self) -> AdirawoR {
        AdirawoR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADIRAWS"]
    #[inline(always)]
    #[must_use]
    pub fn adiraws(&mut self) -> AdirawsW<IrawSpec> {
        AdirawsW::new(self, 0)
    }
    #[doc = "Bit 1 - ADIRAWG"]
    #[inline(always)]
    #[must_use]
    pub fn adirawg(&mut self) -> AdirawgW<IrawSpec> {
        AdirawgW::new(self, 1)
    }
    #[doc = "Bit 2 - ADIRAWC"]
    #[inline(always)]
    #[must_use]
    pub fn adirawc(&mut self) -> AdirawcW<IrawSpec> {
        AdirawcW::new(self, 2)
    }
    #[doc = "Bit 16 - ADIRAWL"]
    #[inline(always)]
    #[must_use]
    pub fn adirawl(&mut self) -> AdirawlW<IrawSpec> {
        AdirawlW::new(self, 16)
    }
    #[doc = "Bit 17 - ADIRAWU"]
    #[inline(always)]
    #[must_use]
    pub fn adirawu(&mut self) -> AdirawuW<IrawSpec> {
        AdirawuW::new(self, 17)
    }
    #[doc = "Bit 24 - ADIRAWO"]
    #[inline(always)]
    #[must_use]
    pub fn adirawo(&mut self) -> AdirawoW<IrawSpec> {
        AdirawoW::new(self, 24)
    }
}
#[doc = "IRAW\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iraw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iraw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrawSpec;
impl crate::RegisterSpec for IrawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iraw::R`](R) reader structure"]
impl crate::Readable for IrawSpec {}
#[doc = "`write(|w| ..)` method takes [`iraw::W`](W) writer structure"]
impl crate::Writable for IrawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRAW to value 0"]
impl crate::Resettable for IrawSpec {
    const RESET_VALUE: u32 = 0;
}

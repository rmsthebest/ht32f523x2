#[doc = "Register `PR` reader"]
pub type R = crate::R<PrSpec>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PrSpec>;
#[doc = "Field `CSPOL` reader - CSPOL"]
pub type CspolR = crate::BitReader;
#[doc = "Field `CSPOL` writer - CSPOL"]
pub type CspolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPOL` reader - OEPOL"]
pub type OepolR = crate::BitReader;
#[doc = "Field `OEPOL` writer - OEPOL"]
pub type OepolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEPOL` reader - WEPOL"]
pub type WepolR = crate::BitReader;
#[doc = "Field `WEPOL` writer - WEPOL"]
pub type WepolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALEPOL` reader - ALEPOL"]
pub type AlepolR = crate::BitReader;
#[doc = "Field `ALEPOL` writer - ALEPOL"]
pub type AlepolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CSPOL"]
    #[inline(always)]
    pub fn cspol(&self) -> CspolR {
        CspolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OEPOL"]
    #[inline(always)]
    pub fn oepol(&self) -> OepolR {
        OepolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WEPOL"]
    #[inline(always)]
    pub fn wepol(&self) -> WepolR {
        WepolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ALEPOL"]
    #[inline(always)]
    pub fn alepol(&self) -> AlepolR {
        AlepolR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSPOL"]
    #[inline(always)]
    #[must_use]
    pub fn cspol(&mut self) -> CspolW<PrSpec> {
        CspolW::new(self, 0)
    }
    #[doc = "Bit 1 - OEPOL"]
    #[inline(always)]
    #[must_use]
    pub fn oepol(&mut self) -> OepolW<PrSpec> {
        OepolW::new(self, 1)
    }
    #[doc = "Bit 2 - WEPOL"]
    #[inline(always)]
    #[must_use]
    pub fn wepol(&mut self) -> WepolW<PrSpec> {
        WepolW::new(self, 2)
    }
    #[doc = "Bit 3 - ALEPOL"]
    #[inline(always)]
    #[must_use]
    pub fn alepol(&mut self) -> AlepolW<PrSpec> {
        AlepolW::new(self, 3)
    }
}
#[doc = "PR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrSpec;
impl crate::RegisterSpec for PrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PrSpec {
    const RESET_VALUE: u32 = 0;
}

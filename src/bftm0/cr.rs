#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `MIEN` reader - MIEN"]
pub type MienR = crate::BitReader;
#[doc = "Field `MIEN` writer - MIEN"]
pub type MienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSM` reader - OSM"]
pub type OsmR = crate::BitReader;
#[doc = "Field `OSM` writer - OSM"]
pub type OsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN` reader - CEN"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - CEN"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MIEN"]
    #[inline(always)]
    pub fn mien(&self) -> MienR {
        MienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OSM"]
    #[inline(always)]
    pub fn osm(&self) -> OsmR {
        OsmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CEN"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MIEN"]
    #[inline(always)]
    pub fn mien(&mut self) -> MienW<'_, CrSpec> {
        MienW::new(self, 0)
    }
    #[doc = "Bit 1 - OSM"]
    #[inline(always)]
    pub fn osm(&mut self) -> OsmW<'_, CrSpec> {
        OsmW::new(self, 1)
    }
    #[doc = "Bit 2 - CEN"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<'_, CrSpec> {
        CenW::new(self, 2)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}

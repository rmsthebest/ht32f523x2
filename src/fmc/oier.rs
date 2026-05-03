#[doc = "Register `OIER` reader"]
pub type R = crate::R<OierSpec>;
#[doc = "Register `OIER` writer"]
pub type W = crate::W<OierSpec>;
#[doc = "Field `ORFIEN` reader - ORFIEN"]
pub type OrfienR = crate::BitReader;
#[doc = "Field `ORFIEN` writer - ORFIEN"]
pub type OrfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITADIEN` reader - ITADIEN"]
pub type ItadienR = crate::BitReader;
#[doc = "Field `ITADIEN` writer - ITADIEN"]
pub type ItadienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBEIEN` reader - OBEIEN"]
pub type ObeienR = crate::BitReader;
#[doc = "Field `OBEIEN` writer - OBEIEN"]
pub type ObeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCMIEN` reader - IOCMIEN"]
pub type IocmienR = crate::BitReader;
#[doc = "Field `IOCMIEN` writer - IOCMIEN"]
pub type IocmienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OREIEN` reader - OREIEN"]
pub type OreienR = crate::BitReader;
#[doc = "Field `OREIEN` writer - OREIEN"]
pub type OreienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ORFIEN"]
    #[inline(always)]
    pub fn orfien(&self) -> OrfienR {
        OrfienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ITADIEN"]
    #[inline(always)]
    pub fn itadien(&self) -> ItadienR {
        ItadienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OBEIEN"]
    #[inline(always)]
    pub fn obeien(&self) -> ObeienR {
        ObeienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IOCMIEN"]
    #[inline(always)]
    pub fn iocmien(&self) -> IocmienR {
        IocmienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OREIEN"]
    #[inline(always)]
    pub fn oreien(&self) -> OreienR {
        OreienR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ORFIEN"]
    #[inline(always)]
    pub fn orfien(&mut self) -> OrfienW<'_, OierSpec> {
        OrfienW::new(self, 0)
    }
    #[doc = "Bit 1 - ITADIEN"]
    #[inline(always)]
    pub fn itadien(&mut self) -> ItadienW<'_, OierSpec> {
        ItadienW::new(self, 1)
    }
    #[doc = "Bit 2 - OBEIEN"]
    #[inline(always)]
    pub fn obeien(&mut self) -> ObeienW<'_, OierSpec> {
        ObeienW::new(self, 2)
    }
    #[doc = "Bit 3 - IOCMIEN"]
    #[inline(always)]
    pub fn iocmien(&mut self) -> IocmienW<'_, OierSpec> {
        IocmienW::new(self, 3)
    }
    #[doc = "Bit 4 - OREIEN"]
    #[inline(always)]
    pub fn oreien(&mut self) -> OreienW<'_, OierSpec> {
        OreienW::new(self, 4)
    }
}
#[doc = "OIER\n\nYou can [`read`](crate::Reg::read) this register and get [`oier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OierSpec;
impl crate::RegisterSpec for OierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oier::R`](R) reader structure"]
impl crate::Readable for OierSpec {}
#[doc = "`write(|w| ..)` method takes [`oier::W`](W) writer structure"]
impl crate::Writable for OierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OIER to value 0"]
impl crate::Resettable for OierSpec {}

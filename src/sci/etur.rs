#[doc = "Register `ETUR` reader"]
pub type R = crate::R<EturSpec>;
#[doc = "Register `ETUR` writer"]
pub type W = crate::W<EturSpec>;
#[doc = "Field `ETU` reader - ETU"]
pub type EtuR = crate::FieldReader<u16>;
#[doc = "Field `ETU` writer - ETU"]
pub type EtuW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `COMP` reader - COMP"]
pub type CompR = crate::BitReader;
#[doc = "Field `COMP` writer - COMP"]
pub type CompW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - ETU"]
    #[inline(always)]
    pub fn etu(&self) -> EtuR {
        EtuR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - COMP"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - ETU"]
    #[inline(always)]
    #[must_use]
    pub fn etu(&mut self) -> EtuW<EturSpec> {
        EtuW::new(self, 0)
    }
    #[doc = "Bit 15 - COMP"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> CompW<EturSpec> {
        CompW::new(self, 15)
    }
}
#[doc = "ETUR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EturSpec;
impl crate::RegisterSpec for EturSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etur::R`](R) reader structure"]
impl crate::Readable for EturSpec {}
#[doc = "`write(|w| ..)` method takes [`etur::W`](W) writer structure"]
impl crate::Writable for EturSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETUR to value 0"]
impl crate::Resettable for EturSpec {
    const RESET_VALUE: u32 = 0;
}

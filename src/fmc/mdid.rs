#[doc = "Register `MDID` reader"]
pub type R = crate::R<MdidSpec>;
#[doc = "Register `MDID` writer"]
pub type W = crate::W<MdidSpec>;
#[doc = "Field `CHIPID` reader - CHIPID"]
pub type ChipidR = crate::FieldReader<u16>;
#[doc = "Field `CHIPID` writer - CHIPID"]
pub type ChipidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MFID` reader - MFID"]
pub type MfidR = crate::FieldReader<u16>;
#[doc = "Field `MFID` writer - MFID"]
pub type MfidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CHIPID"]
    #[inline(always)]
    pub fn chipid(&self) -> ChipidR {
        ChipidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MFID"]
    #[inline(always)]
    pub fn mfid(&self) -> MfidR {
        MfidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CHIPID"]
    #[inline(always)]
    pub fn chipid(&mut self) -> ChipidW<'_, MdidSpec> {
        ChipidW::new(self, 0)
    }
    #[doc = "Bits 16:31 - MFID"]
    #[inline(always)]
    pub fn mfid(&mut self) -> MfidW<'_, MdidSpec> {
        MfidW::new(self, 16)
    }
}
#[doc = "MDID\n\nYou can [`read`](crate::Reg::read) this register and get [`mdid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdidSpec;
impl crate::RegisterSpec for MdidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdid::R`](R) reader structure"]
impl crate::Readable for MdidSpec {}
#[doc = "`write(|w| ..)` method takes [`mdid::W`](W) writer structure"]
impl crate::Writable for MdidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDID to value 0"]
impl crate::Resettable for MdidSpec {}

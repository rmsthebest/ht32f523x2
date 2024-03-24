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
    #[must_use]
    pub fn chipid(&mut self) -> ChipidW<MdidSpec> {
        ChipidW::new(self, 0)
    }
    #[doc = "Bits 16:31 - MFID"]
    #[inline(always)]
    #[must_use]
    pub fn mfid(&mut self) -> MfidW<MdidSpec> {
        MfidW::new(self, 16)
    }
}
#[doc = "MDID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdidSpec;
impl crate::RegisterSpec for MdidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdid::R`](R) reader structure"]
impl crate::Readable for MdidSpec {}
#[doc = "`write(|w| ..)` method takes [`mdid::W`](W) writer structure"]
impl crate::Writable for MdidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDID to value 0"]
impl crate::Resettable for MdidSpec {
    const RESET_VALUE: u32 = 0;
}

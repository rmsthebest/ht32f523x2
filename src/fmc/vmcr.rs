#[doc = "Register `VMCR` reader"]
pub type R = crate::R<VmcrSpec>;
#[doc = "Register `VMCR` writer"]
pub type W = crate::W<VmcrSpec>;
#[doc = "Field `VMCB` reader - VMCB"]
pub type VmcbR = crate::BitReader;
#[doc = "Field `VMCB` writer - VMCB"]
pub type VmcbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - VMCB"]
    #[inline(always)]
    pub fn vmcb(&self) -> VmcbR {
        VmcbR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - VMCB"]
    #[inline(always)]
    pub fn vmcb(&mut self) -> VmcbW<'_, VmcrSpec> {
        VmcbW::new(self, 1)
    }
}
#[doc = "VMCR\n\nYou can [`read`](crate::Reg::read) this register and get [`vmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VmcrSpec;
impl crate::RegisterSpec for VmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmcr::R`](R) reader structure"]
impl crate::Readable for VmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`vmcr::W`](W) writer structure"]
impl crate::Writable for VmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VMCR to value 0"]
impl crate::Resettable for VmcrSpec {}

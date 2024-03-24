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
    #[must_use]
    pub fn vmcb(&mut self) -> VmcbW<VmcrSpec> {
        VmcbW::new(self, 1)
    }
}
#[doc = "VMCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VmcrSpec;
impl crate::RegisterSpec for VmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmcr::R`](R) reader structure"]
impl crate::Readable for VmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`vmcr::W`](W) writer structure"]
impl crate::Writable for VmcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VMCR to value 0"]
impl crate::Resettable for VmcrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CFCR` reader"]
pub type R = crate::R<CfcrSpec>;
#[doc = "Register `CFCR` writer"]
pub type W = crate::W<CfcrSpec>;
#[doc = "Field `WAIT` reader - WAIT"]
pub type WaitR = crate::FieldReader;
#[doc = "Field `WAIT` writer - WAIT"]
pub type WaitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PFBE` reader - PFBE"]
pub type PfbeR = crate::BitReader;
#[doc = "Field `PFBE` writer - PFBE"]
pub type PfbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDB` reader - DCDB"]
pub type DcdbR = crate::BitReader;
#[doc = "Field `DCDB` writer - DCDB"]
pub type DcdbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CE` reader - CE"]
pub type CeR = crate::BitReader;
#[doc = "Field `CE` writer - CE"]
pub type CeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - WAIT"]
    #[inline(always)]
    pub fn wait(&self) -> WaitR {
        WaitR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - PFBE"]
    #[inline(always)]
    pub fn pfbe(&self) -> PfbeR {
        PfbeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - DCDB"]
    #[inline(always)]
    pub fn dcdb(&self) -> DcdbR {
        DcdbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CE"]
    #[inline(always)]
    pub fn ce(&self) -> CeR {
        CeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - WAIT"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WaitW<CfcrSpec> {
        WaitW::new(self, 0)
    }
    #[doc = "Bit 4 - PFBE"]
    #[inline(always)]
    #[must_use]
    pub fn pfbe(&mut self) -> PfbeW<CfcrSpec> {
        PfbeW::new(self, 4)
    }
    #[doc = "Bit 7 - DCDB"]
    #[inline(always)]
    #[must_use]
    pub fn dcdb(&mut self) -> DcdbW<CfcrSpec> {
        DcdbW::new(self, 7)
    }
    #[doc = "Bit 12 - CE"]
    #[inline(always)]
    #[must_use]
    pub fn ce(&mut self) -> CeW<CfcrSpec> {
        CeW::new(self, 12)
    }
}
#[doc = "CFCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfcrSpec;
impl crate::RegisterSpec for CfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfcr::R`](R) reader structure"]
impl crate::Readable for CfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfcr::W`](W) writer structure"]
impl crate::Writable for CfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFCR to value 0"]
impl crate::Resettable for CfcrSpec {
    const RESET_VALUE: u32 = 0;
}

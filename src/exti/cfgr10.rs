#[doc = "Register `CFGR10` reader"]
pub type R = crate::R<Cfgr10Spec>;
#[doc = "Register `CFGR10` writer"]
pub type W = crate::W<Cfgr10Spec>;
#[doc = "Field `DBCNT` reader - DBCNT"]
pub type DbcntR = crate::FieldReader<u16>;
#[doc = "Field `DBCNT` writer - DBCNT"]
pub type DbcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRCTYPE` reader - SRCTYPE"]
pub type SrctypeR = crate::FieldReader;
#[doc = "Field `SRCTYPE` writer - SRCTYPE"]
pub type SrctypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBEN` reader - DBEN"]
pub type DbenR = crate::BitReader;
#[doc = "Field `DBEN` writer - DBEN"]
pub type DbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - DBCNT"]
    #[inline(always)]
    pub fn dbcnt(&self) -> DbcntR {
        DbcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:30 - SRCTYPE"]
    #[inline(always)]
    pub fn srctype(&self) -> SrctypeR {
        SrctypeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - DBEN"]
    #[inline(always)]
    pub fn dben(&self) -> DbenR {
        DbenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - DBCNT"]
    #[inline(always)]
    #[must_use]
    pub fn dbcnt(&mut self) -> DbcntW<Cfgr10Spec> {
        DbcntW::new(self, 0)
    }
    #[doc = "Bits 28:30 - SRCTYPE"]
    #[inline(always)]
    #[must_use]
    pub fn srctype(&mut self) -> SrctypeW<Cfgr10Spec> {
        SrctypeW::new(self, 28)
    }
    #[doc = "Bit 31 - DBEN"]
    #[inline(always)]
    #[must_use]
    pub fn dben(&mut self) -> DbenW<Cfgr10Spec> {
        DbenW::new(self, 31)
    }
}
#[doc = "CFGR10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr10Spec;
impl crate::RegisterSpec for Cfgr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr10::R`](R) reader structure"]
impl crate::Readable for Cfgr10Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr10::W`](W) writer structure"]
impl crate::Writable for Cfgr10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR10 to value 0"]
impl crate::Resettable for Cfgr10Spec {
    const RESET_VALUE: u32 = 0;
}

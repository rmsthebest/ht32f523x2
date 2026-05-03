#[doc = "Register `CFGR6` reader"]
pub type R = crate::R<Cfgr6Spec>;
#[doc = "Register `CFGR6` writer"]
pub type W = crate::W<Cfgr6Spec>;
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
    pub fn dbcnt(&mut self) -> DbcntW<'_, Cfgr6Spec> {
        DbcntW::new(self, 0)
    }
    #[doc = "Bits 28:30 - SRCTYPE"]
    #[inline(always)]
    pub fn srctype(&mut self) -> SrctypeW<'_, Cfgr6Spec> {
        SrctypeW::new(self, 28)
    }
    #[doc = "Bit 31 - DBEN"]
    #[inline(always)]
    pub fn dben(&mut self) -> DbenW<'_, Cfgr6Spec> {
        DbenW::new(self, 31)
    }
}
#[doc = "CFGR6\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr6Spec;
impl crate::RegisterSpec for Cfgr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr6::R`](R) reader structure"]
impl crate::Readable for Cfgr6Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr6::W`](W) writer structure"]
impl crate::Writable for Cfgr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR6 to value 0"]
impl crate::Resettable for Cfgr6Spec {}

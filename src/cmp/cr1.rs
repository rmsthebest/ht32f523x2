#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `CMPEN` reader - CMPEN"]
pub type CmpenR = crate::BitReader;
#[doc = "Field `CMPEN` writer - CMPEN"]
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPSM` reader - CMPSM"]
pub type CmpsmR = crate::BitReader;
#[doc = "Field `CMPSM` writer - CMPSM"]
pub type CmpsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPHM` reader - CMPHM"]
pub type CmphmR = crate::FieldReader;
#[doc = "Field `CMPHM` writer - CMPHM"]
pub type CmphmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPINSEL` reader - CMPINSEL"]
pub type CmpinselR = crate::FieldReader;
#[doc = "Field `CMPINSEL` writer - CMPINSEL"]
pub type CmpinselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPPOL` reader - CMPPOL"]
pub type CmppolR = crate::BitReader;
#[doc = "Field `CMPPOL` writer - CMPPOL"]
pub type CmppolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCSEL` reader - SYNCSEL"]
pub type SyncselR = crate::BitReader;
#[doc = "Field `SYNCSEL` writer - SYNCSEL"]
pub type SyncselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVREN` reader - CVREN"]
pub type CvrenR = crate::BitReader;
#[doc = "Field `CVREN` writer - CVREN"]
pub type CvrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVROE` reader - CVROE"]
pub type CvroeR = crate::BitReader;
#[doc = "Field `CVROE` writer - CVROE"]
pub type CvroeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVRSS` reader - CVRSS"]
pub type CvrssR = crate::BitReader;
#[doc = "Field `CVRSS` writer - CVRSS"]
pub type CvrssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOSEL` reader - CMPOSEL"]
pub type CmposelR = crate::FieldReader;
#[doc = "Field `CMPOSEL` writer - CMPOSEL"]
pub type CmposelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMPWPEN` reader - CMPWPEN"]
pub type CmpwpenR = crate::BitReader;
#[doc = "Field `CMPWPEN` writer - CMPWPEN"]
pub type CmpwpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPSTS` reader - CMPSTS"]
pub type CmpstsR = crate::BitReader;
#[doc = "Field `CMPSTS` writer - CMPSTS"]
pub type CmpstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROTECT` reader - PROTECT"]
pub type ProtectR = crate::FieldReader<u16>;
#[doc = "Field `PROTECT` writer - PROTECT"]
pub type ProtectW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - CMPEN"]
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMPSM"]
    #[inline(always)]
    pub fn cmpsm(&self) -> CmpsmR {
        CmpsmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - CMPHM"]
    #[inline(always)]
    pub fn cmphm(&self) -> CmphmR {
        CmphmR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CMPINSEL"]
    #[inline(always)]
    pub fn cmpinsel(&self) -> CmpinselR {
        CmpinselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - CMPPOL"]
    #[inline(always)]
    pub fn cmppol(&self) -> CmppolR {
        CmppolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SYNCSEL"]
    #[inline(always)]
    pub fn syncsel(&self) -> SyncselR {
        SyncselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CVREN"]
    #[inline(always)]
    pub fn cvren(&self) -> CvrenR {
        CvrenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CVROE"]
    #[inline(always)]
    pub fn cvroe(&self) -> CvroeR {
        CvroeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CVRSS"]
    #[inline(always)]
    pub fn cvrss(&self) -> CvrssR {
        CvrssR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - CMPOSEL"]
    #[inline(always)]
    pub fn cmposel(&self) -> CmposelR {
        CmposelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - CMPWPEN"]
    #[inline(always)]
    pub fn cmpwpen(&self) -> CmpwpenR {
        CmpwpenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CMPSTS"]
    #[inline(always)]
    pub fn cmpsts(&self) -> CmpstsR {
        CmpstsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PROTECT"]
    #[inline(always)]
    pub fn protect(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - CMPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CmpenW<Cr1Spec> {
        CmpenW::new(self, 0)
    }
    #[doc = "Bit 1 - CMPSM"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsm(&mut self) -> CmpsmW<Cr1Spec> {
        CmpsmW::new(self, 1)
    }
    #[doc = "Bits 2:3 - CMPHM"]
    #[inline(always)]
    #[must_use]
    pub fn cmphm(&mut self) -> CmphmW<Cr1Spec> {
        CmphmW::new(self, 2)
    }
    #[doc = "Bits 4:5 - CMPINSEL"]
    #[inline(always)]
    #[must_use]
    pub fn cmpinsel(&mut self) -> CmpinselW<Cr1Spec> {
        CmpinselW::new(self, 4)
    }
    #[doc = "Bit 6 - CMPPOL"]
    #[inline(always)]
    #[must_use]
    pub fn cmppol(&mut self) -> CmppolW<Cr1Spec> {
        CmppolW::new(self, 6)
    }
    #[doc = "Bit 7 - SYNCSEL"]
    #[inline(always)]
    #[must_use]
    pub fn syncsel(&mut self) -> SyncselW<Cr1Spec> {
        SyncselW::new(self, 7)
    }
    #[doc = "Bit 8 - CVREN"]
    #[inline(always)]
    #[must_use]
    pub fn cvren(&mut self) -> CvrenW<Cr1Spec> {
        CvrenW::new(self, 8)
    }
    #[doc = "Bit 9 - CVROE"]
    #[inline(always)]
    #[must_use]
    pub fn cvroe(&mut self) -> CvroeW<Cr1Spec> {
        CvroeW::new(self, 9)
    }
    #[doc = "Bit 10 - CVRSS"]
    #[inline(always)]
    #[must_use]
    pub fn cvrss(&mut self) -> CvrssW<Cr1Spec> {
        CvrssW::new(self, 10)
    }
    #[doc = "Bits 11:13 - CMPOSEL"]
    #[inline(always)]
    #[must_use]
    pub fn cmposel(&mut self) -> CmposelW<Cr1Spec> {
        CmposelW::new(self, 11)
    }
    #[doc = "Bit 14 - CMPWPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmpwpen(&mut self) -> CmpwpenW<Cr1Spec> {
        CmpwpenW::new(self, 14)
    }
    #[doc = "Bit 15 - CMPSTS"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsts(&mut self) -> CmpstsW<Cr1Spec> {
        CmpstsW::new(self, 15)
    }
    #[doc = "Bits 16:31 - PROTECT"]
    #[inline(always)]
    #[must_use]
    pub fn protect(&mut self) -> ProtectW<Cr1Spec> {
        ProtectW::new(self, 16)
    }
}
#[doc = "CR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}

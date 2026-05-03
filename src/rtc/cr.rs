#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `RTCEN` reader - RTCEN"]
pub type RtcenR = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTCEN"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSRC` reader - RTCSRC"]
pub type RtcsrcR = crate::BitReader;
#[doc = "Field `RTCSRC` writer - RTCSRC"]
pub type RtcsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIEN` reader - LSIEN"]
pub type LsienR = crate::BitReader;
#[doc = "Field `LSIEN` writer - LSIEN"]
pub type LsienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEEN` reader - LSEEN"]
pub type LseenR = crate::BitReader;
#[doc = "Field `LSEEN` writer - LSEEN"]
pub type LseenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCLR` reader - CMPCLR"]
pub type CmpclrR = crate::BitReader;
#[doc = "Field `CMPCLR` writer - CMPCLR"]
pub type CmpclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSESM` reader - LSESM"]
pub type LsesmR = crate::BitReader;
#[doc = "Field `LSESM` writer - LSESM"]
pub type LsesmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPRE` reader - RPRE"]
pub type RpreR = crate::FieldReader;
#[doc = "Field `RPRE` writer - RPRE"]
pub type RpreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ROEN` reader - ROEN"]
pub type RoenR = crate::BitReader;
#[doc = "Field `ROEN` writer - ROEN"]
pub type RoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROES` reader - ROES"]
pub type RoesR = crate::BitReader;
#[doc = "Field `ROES` writer - ROES"]
pub type RoesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROWM` reader - ROWM"]
pub type RowmR = crate::BitReader;
#[doc = "Field `ROWM` writer - ROWM"]
pub type RowmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROAP` reader - ROAP"]
pub type RoapR = crate::BitReader;
#[doc = "Field `ROAP` writer - ROAP"]
pub type RoapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROLF` reader - ROLF"]
pub type RolfR = crate::BitReader;
#[doc = "Field `ROLF` writer - ROLF"]
pub type RolfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTCSRC"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RtcsrcR {
        RtcsrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSIEN"]
    #[inline(always)]
    pub fn lsien(&self) -> LsienR {
        LsienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSEEN"]
    #[inline(always)]
    pub fn lseen(&self) -> LseenR {
        LseenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMPCLR"]
    #[inline(always)]
    pub fn cmpclr(&self) -> CmpclrR {
        CmpclrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LSESM"]
    #[inline(always)]
    pub fn lsesm(&self) -> LsesmR {
        LsesmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - RPRE"]
    #[inline(always)]
    pub fn rpre(&self) -> RpreR {
        RpreR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - ROEN"]
    #[inline(always)]
    pub fn roen(&self) -> RoenR {
        RoenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ROES"]
    #[inline(always)]
    pub fn roes(&self) -> RoesR {
        RoesR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ROWM"]
    #[inline(always)]
    pub fn rowm(&self) -> RowmR {
        RowmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ROAP"]
    #[inline(always)]
    pub fn roap(&self) -> RoapR {
        RoapR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ROLF"]
    #[inline(always)]
    pub fn rolf(&self) -> RolfR {
        RolfR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RtcenW<'_, CrSpec> {
        RtcenW::new(self, 0)
    }
    #[doc = "Bit 1 - RTCSRC"]
    #[inline(always)]
    pub fn rtcsrc(&mut self) -> RtcsrcW<'_, CrSpec> {
        RtcsrcW::new(self, 1)
    }
    #[doc = "Bit 2 - LSIEN"]
    #[inline(always)]
    pub fn lsien(&mut self) -> LsienW<'_, CrSpec> {
        LsienW::new(self, 2)
    }
    #[doc = "Bit 3 - LSEEN"]
    #[inline(always)]
    pub fn lseen(&mut self) -> LseenW<'_, CrSpec> {
        LseenW::new(self, 3)
    }
    #[doc = "Bit 4 - CMPCLR"]
    #[inline(always)]
    pub fn cmpclr(&mut self) -> CmpclrW<'_, CrSpec> {
        CmpclrW::new(self, 4)
    }
    #[doc = "Bit 5 - LSESM"]
    #[inline(always)]
    pub fn lsesm(&mut self) -> LsesmW<'_, CrSpec> {
        LsesmW::new(self, 5)
    }
    #[doc = "Bits 8:11 - RPRE"]
    #[inline(always)]
    pub fn rpre(&mut self) -> RpreW<'_, CrSpec> {
        RpreW::new(self, 8)
    }
    #[doc = "Bit 16 - ROEN"]
    #[inline(always)]
    pub fn roen(&mut self) -> RoenW<'_, CrSpec> {
        RoenW::new(self, 16)
    }
    #[doc = "Bit 17 - ROES"]
    #[inline(always)]
    pub fn roes(&mut self) -> RoesW<'_, CrSpec> {
        RoesW::new(self, 17)
    }
    #[doc = "Bit 18 - ROWM"]
    #[inline(always)]
    pub fn rowm(&mut self) -> RowmW<'_, CrSpec> {
        RowmW::new(self, 18)
    }
    #[doc = "Bit 19 - ROAP"]
    #[inline(always)]
    pub fn roap(&mut self) -> RoapW<'_, CrSpec> {
        RoapW::new(self, 19)
    }
    #[doc = "Bit 20 - ROLF"]
    #[inline(always)]
    pub fn rolf(&mut self) -> RolfW<'_, CrSpec> {
        RolfW::new(self, 20)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}

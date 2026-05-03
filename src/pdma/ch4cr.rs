#[doc = "Register `CH4CR` reader"]
pub type R = crate::R<Ch4crSpec>;
#[doc = "Register `CH4CR` writer"]
pub type W = crate::W<Ch4crSpec>;
#[doc = "Field `CHEN` reader - CHEN"]
pub type ChenR = crate::BitReader;
#[doc = "Field `CHEN` writer - CHEN"]
pub type ChenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG` reader - SWTRIG"]
pub type SwtrigR = crate::BitReader;
#[doc = "Field `SWTRIG` writer - SWTRIG"]
pub type SwtrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DWIDTH` reader - DWIDTH"]
pub type DwidthR = crate::FieldReader;
#[doc = "Field `DWIDTH` writer - DWIDTH"]
pub type DwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DSTAINC` reader - DSTAINC"]
pub type DstaincR = crate::BitReader;
#[doc = "Field `DSTAINC` writer - DSTAINC"]
pub type DstaincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSTAMOD` reader - DSTAMOD"]
pub type DstamodR = crate::BitReader;
#[doc = "Field `DSTAMOD` writer - DSTAMOD"]
pub type DstamodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRCAINC` reader - SRCAINC"]
pub type SrcaincR = crate::BitReader;
#[doc = "Field `SRCAINC` writer - SRCAINC"]
pub type SrcaincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRCAMOD` reader - SRCAMOD"]
pub type SrcamodR = crate::BitReader;
#[doc = "Field `SRCAMOD` writer - SRCAMOD"]
pub type SrcamodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHPRI` reader - CHPRI"]
pub type ChpriR = crate::FieldReader;
#[doc = "Field `CHPRI` writer - CHPRI"]
pub type ChpriW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIXAEN` reader - FIXAEN"]
pub type FixaenR = crate::BitReader;
#[doc = "Field `FIXAEN` writer - FIXAEN"]
pub type FixaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTORL` reader - AUTORL"]
pub type AutorlR = crate::BitReader;
#[doc = "Field `AUTORL` writer - AUTORL"]
pub type AutorlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CHEN"]
    #[inline(always)]
    pub fn chen(&self) -> ChenR {
        ChenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWTRIG"]
    #[inline(always)]
    pub fn swtrig(&self) -> SwtrigR {
        SwtrigR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DWIDTH"]
    #[inline(always)]
    pub fn dwidth(&self) -> DwidthR {
        DwidthR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - DSTAINC"]
    #[inline(always)]
    pub fn dstainc(&self) -> DstaincR {
        DstaincR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DSTAMOD"]
    #[inline(always)]
    pub fn dstamod(&self) -> DstamodR {
        DstamodR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SRCAINC"]
    #[inline(always)]
    pub fn srcainc(&self) -> SrcaincR {
        SrcaincR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SRCAMOD"]
    #[inline(always)]
    pub fn srcamod(&self) -> SrcamodR {
        SrcamodR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CHPRI"]
    #[inline(always)]
    pub fn chpri(&self) -> ChpriR {
        ChpriR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - FIXAEN"]
    #[inline(always)]
    pub fn fixaen(&self) -> FixaenR {
        FixaenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AUTORL"]
    #[inline(always)]
    pub fn autorl(&self) -> AutorlR {
        AutorlR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CHEN"]
    #[inline(always)]
    pub fn chen(&mut self) -> ChenW<'_, Ch4crSpec> {
        ChenW::new(self, 0)
    }
    #[doc = "Bit 1 - SWTRIG"]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SwtrigW<'_, Ch4crSpec> {
        SwtrigW::new(self, 1)
    }
    #[doc = "Bits 2:3 - DWIDTH"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DwidthW<'_, Ch4crSpec> {
        DwidthW::new(self, 2)
    }
    #[doc = "Bit 4 - DSTAINC"]
    #[inline(always)]
    pub fn dstainc(&mut self) -> DstaincW<'_, Ch4crSpec> {
        DstaincW::new(self, 4)
    }
    #[doc = "Bit 5 - DSTAMOD"]
    #[inline(always)]
    pub fn dstamod(&mut self) -> DstamodW<'_, Ch4crSpec> {
        DstamodW::new(self, 5)
    }
    #[doc = "Bit 6 - SRCAINC"]
    #[inline(always)]
    pub fn srcainc(&mut self) -> SrcaincW<'_, Ch4crSpec> {
        SrcaincW::new(self, 6)
    }
    #[doc = "Bit 7 - SRCAMOD"]
    #[inline(always)]
    pub fn srcamod(&mut self) -> SrcamodW<'_, Ch4crSpec> {
        SrcamodW::new(self, 7)
    }
    #[doc = "Bits 8:9 - CHPRI"]
    #[inline(always)]
    pub fn chpri(&mut self) -> ChpriW<'_, Ch4crSpec> {
        ChpriW::new(self, 8)
    }
    #[doc = "Bit 10 - FIXAEN"]
    #[inline(always)]
    pub fn fixaen(&mut self) -> FixaenW<'_, Ch4crSpec> {
        FixaenW::new(self, 10)
    }
    #[doc = "Bit 11 - AUTORL"]
    #[inline(always)]
    pub fn autorl(&mut self) -> AutorlW<'_, Ch4crSpec> {
        AutorlW::new(self, 11)
    }
}
#[doc = "CH4CR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4crSpec;
impl crate::RegisterSpec for Ch4crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4cr::R`](R) reader structure"]
impl crate::Readable for Ch4crSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4cr::W`](W) writer structure"]
impl crate::Writable for Ch4crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4CR to value 0"]
impl crate::Resettable for Ch4crSpec {}

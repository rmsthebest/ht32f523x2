#[doc = "Register `CH3CR` reader"]
pub type R = crate::R<Ch3crSpec>;
#[doc = "Register `CH3CR` writer"]
pub type W = crate::W<Ch3crSpec>;
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
    #[must_use]
    pub fn chen(&mut self) -> ChenW<Ch3crSpec> {
        ChenW::new(self, 0)
    }
    #[doc = "Bit 1 - SWTRIG"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig(&mut self) -> SwtrigW<Ch3crSpec> {
        SwtrigW::new(self, 1)
    }
    #[doc = "Bits 2:3 - DWIDTH"]
    #[inline(always)]
    #[must_use]
    pub fn dwidth(&mut self) -> DwidthW<Ch3crSpec> {
        DwidthW::new(self, 2)
    }
    #[doc = "Bit 4 - DSTAINC"]
    #[inline(always)]
    #[must_use]
    pub fn dstainc(&mut self) -> DstaincW<Ch3crSpec> {
        DstaincW::new(self, 4)
    }
    #[doc = "Bit 5 - DSTAMOD"]
    #[inline(always)]
    #[must_use]
    pub fn dstamod(&mut self) -> DstamodW<Ch3crSpec> {
        DstamodW::new(self, 5)
    }
    #[doc = "Bit 6 - SRCAINC"]
    #[inline(always)]
    #[must_use]
    pub fn srcainc(&mut self) -> SrcaincW<Ch3crSpec> {
        SrcaincW::new(self, 6)
    }
    #[doc = "Bit 7 - SRCAMOD"]
    #[inline(always)]
    #[must_use]
    pub fn srcamod(&mut self) -> SrcamodW<Ch3crSpec> {
        SrcamodW::new(self, 7)
    }
    #[doc = "Bits 8:9 - CHPRI"]
    #[inline(always)]
    #[must_use]
    pub fn chpri(&mut self) -> ChpriW<Ch3crSpec> {
        ChpriW::new(self, 8)
    }
    #[doc = "Bit 10 - FIXAEN"]
    #[inline(always)]
    #[must_use]
    pub fn fixaen(&mut self) -> FixaenW<Ch3crSpec> {
        FixaenW::new(self, 10)
    }
    #[doc = "Bit 11 - AUTORL"]
    #[inline(always)]
    #[must_use]
    pub fn autorl(&mut self) -> AutorlW<Ch3crSpec> {
        AutorlW::new(self, 11)
    }
}
#[doc = "CH3CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3crSpec;
impl crate::RegisterSpec for Ch3crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3cr::R`](R) reader structure"]
impl crate::Readable for Ch3crSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3cr::W`](W) writer structure"]
impl crate::Writable for Ch3crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3CR to value 0"]
impl crate::Resettable for Ch3crSpec {
    const RESET_VALUE: u32 = 0;
}

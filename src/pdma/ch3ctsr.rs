#[doc = "Register `CH3CTSR` reader"]
pub type R = crate::R<Ch3ctsrSpec>;
#[doc = "Register `CH3CTSR` writer"]
pub type W = crate::W<Ch3ctsrSpec>;
#[doc = "Field `CBLKLEN` reader - CBLKLEN"]
pub type CblklenR = crate::FieldReader<u16>;
#[doc = "Field `CBLKLEN` writer - CBLKLEN"]
pub type CblklenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CBLKCNT` reader - CBLKCNT"]
pub type CblkcntR = crate::FieldReader<u16>;
#[doc = "Field `CBLKCNT` writer - CBLKCNT"]
pub type CblkcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CBLKLEN"]
    #[inline(always)]
    pub fn cblklen(&self) -> CblklenR {
        CblklenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CBLKCNT"]
    #[inline(always)]
    pub fn cblkcnt(&self) -> CblkcntR {
        CblkcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CBLKLEN"]
    #[inline(always)]
    pub fn cblklen(&mut self) -> CblklenW<'_, Ch3ctsrSpec> {
        CblklenW::new(self, 0)
    }
    #[doc = "Bits 16:31 - CBLKCNT"]
    #[inline(always)]
    pub fn cblkcnt(&mut self) -> CblkcntW<'_, Ch3ctsrSpec> {
        CblkcntW::new(self, 16)
    }
}
#[doc = "CH3CTSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3ctsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3ctsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3ctsrSpec;
impl crate::RegisterSpec for Ch3ctsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3ctsr::R`](R) reader structure"]
impl crate::Readable for Ch3ctsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3ctsr::W`](W) writer structure"]
impl crate::Writable for Ch3ctsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3CTSR to value 0"]
impl crate::Resettable for Ch3ctsrSpec {}

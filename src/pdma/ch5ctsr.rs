#[doc = "Register `CH5CTSR` reader"]
pub type R = crate::R<Ch5ctsrSpec>;
#[doc = "Register `CH5CTSR` writer"]
pub type W = crate::W<Ch5ctsrSpec>;
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
    pub fn cblklen(&mut self) -> CblklenW<'_, Ch5ctsrSpec> {
        CblklenW::new(self, 0)
    }
    #[doc = "Bits 16:31 - CBLKCNT"]
    #[inline(always)]
    pub fn cblkcnt(&mut self) -> CblkcntW<'_, Ch5ctsrSpec> {
        CblkcntW::new(self, 16)
    }
}
#[doc = "CH5CTSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5ctsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5ctsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch5ctsrSpec;
impl crate::RegisterSpec for Ch5ctsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5ctsr::R`](R) reader structure"]
impl crate::Readable for Ch5ctsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch5ctsr::W`](W) writer structure"]
impl crate::Writable for Ch5ctsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH5CTSR to value 0"]
impl crate::Resettable for Ch5ctsrSpec {}

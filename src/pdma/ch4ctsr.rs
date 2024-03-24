#[doc = "Register `CH4CTSR` reader"]
pub type R = crate::R<Ch4ctsrSpec>;
#[doc = "Register `CH4CTSR` writer"]
pub type W = crate::W<Ch4ctsrSpec>;
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
    #[must_use]
    pub fn cblklen(&mut self) -> CblklenW<Ch4ctsrSpec> {
        CblklenW::new(self, 0)
    }
    #[doc = "Bits 16:31 - CBLKCNT"]
    #[inline(always)]
    #[must_use]
    pub fn cblkcnt(&mut self) -> CblkcntW<Ch4ctsrSpec> {
        CblkcntW::new(self, 16)
    }
}
#[doc = "CH4CTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4ctsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4ctsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4ctsrSpec;
impl crate::RegisterSpec for Ch4ctsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4ctsr::R`](R) reader structure"]
impl crate::Readable for Ch4ctsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4ctsr::W`](W) writer structure"]
impl crate::Writable for Ch4ctsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4CTSR to value 0"]
impl crate::Resettable for Ch4ctsrSpec {
    const RESET_VALUE: u32 = 0;
}

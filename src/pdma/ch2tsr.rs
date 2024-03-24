#[doc = "Register `CH2TSR` reader"]
pub type R = crate::R<Ch2tsrSpec>;
#[doc = "Register `CH2TSR` writer"]
pub type W = crate::W<Ch2tsrSpec>;
#[doc = "Field `BLKLEN` reader - BLKLEN"]
pub type BlklenR = crate::FieldReader<u16>;
#[doc = "Field `BLKLEN` writer - BLKLEN"]
pub type BlklenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BLKCNT` reader - BLKCNT"]
pub type BlkcntR = crate::FieldReader<u16>;
#[doc = "Field `BLKCNT` writer - BLKCNT"]
pub type BlkcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BLKLEN"]
    #[inline(always)]
    pub fn blklen(&self) -> BlklenR {
        BlklenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - BLKCNT"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BlkcntR {
        BlkcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BLKLEN"]
    #[inline(always)]
    #[must_use]
    pub fn blklen(&mut self) -> BlklenW<Ch2tsrSpec> {
        BlklenW::new(self, 0)
    }
    #[doc = "Bits 16:31 - BLKCNT"]
    #[inline(always)]
    #[must_use]
    pub fn blkcnt(&mut self) -> BlkcntW<Ch2tsrSpec> {
        BlkcntW::new(self, 16)
    }
}
#[doc = "CH2TSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2tsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2tsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2tsrSpec;
impl crate::RegisterSpec for Ch2tsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2tsr::R`](R) reader structure"]
impl crate::Readable for Ch2tsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2tsr::W`](W) writer structure"]
impl crate::Writable for Ch2tsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2TSR to value 0"]
impl crate::Resettable for Ch2tsrSpec {
    const RESET_VALUE: u32 = 0;
}

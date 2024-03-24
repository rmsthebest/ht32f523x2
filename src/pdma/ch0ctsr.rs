#[doc = "Register `CH0CTSR` reader"]
pub type R = crate::R<Ch0ctsrSpec>;
#[doc = "Register `CH0CTSR` writer"]
pub type W = crate::W<Ch0ctsrSpec>;
#[doc = "Field `CBLKCNT` reader - CBLKCNT"]
pub type CblkcntR = crate::FieldReader<u16>;
#[doc = "Field `CBLKCNT` writer - CBLKCNT"]
pub type CblkcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - CBLKCNT"]
    #[inline(always)]
    pub fn cblkcnt(&self) -> CblkcntR {
        CblkcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - CBLKCNT"]
    #[inline(always)]
    #[must_use]
    pub fn cblkcnt(&mut self) -> CblkcntW<Ch0ctsrSpec> {
        CblkcntW::new(self, 16)
    }
}
#[doc = "CH0CTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0ctsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0ctsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0ctsrSpec;
impl crate::RegisterSpec for Ch0ctsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0ctsr::R`](R) reader structure"]
impl crate::Readable for Ch0ctsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0ctsr::W`](W) writer structure"]
impl crate::Writable for Ch0ctsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0CTSR to value 0"]
impl crate::Resettable for Ch0ctsrSpec {
    const RESET_VALUE: u32 = 0;
}

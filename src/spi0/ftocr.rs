#[doc = "Register `FTOCR` reader"]
pub type R = crate::R<FtocrSpec>;
#[doc = "Register `FTOCR` writer"]
pub type W = crate::W<FtocrSpec>;
#[doc = "Field `TOC` reader - TOC"]
pub type TocR = crate::FieldReader<u16>;
#[doc = "Field `TOC` writer - TOC"]
pub type TocW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TOC"]
    #[inline(always)]
    pub fn toc(&mut self) -> TocW<'_, FtocrSpec> {
        TocW::new(self, 0)
    }
}
#[doc = "FTOCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ftocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtocrSpec;
impl crate::RegisterSpec for FtocrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftocr::R`](R) reader structure"]
impl crate::Readable for FtocrSpec {}
#[doc = "`write(|w| ..)` method takes [`ftocr::W`](W) writer structure"]
impl crate::Writable for FtocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTOCR to value 0"]
impl crate::Resettable for FtocrSpec {}

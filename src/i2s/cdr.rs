#[doc = "Register `CDR` reader"]
pub type R = crate::R<CdrSpec>;
#[doc = "Register `CDR` writer"]
pub type W = crate::W<CdrSpec>;
#[doc = "Field `Y_DIV` reader - Y_DIV"]
pub type YDivR = crate::FieldReader;
#[doc = "Field `Y_DIV` writer - Y_DIV"]
pub type YDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `X_DIV` reader - X_DIV"]
pub type XDivR = crate::FieldReader;
#[doc = "Field `X_DIV` writer - X_DIV"]
pub type XDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `N_DIV` reader - N_DIV"]
pub type NDivR = crate::FieldReader;
#[doc = "Field `N_DIV` writer - N_DIV"]
pub type NDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Y_DIV"]
    #[inline(always)]
    pub fn y_div(&self) -> YDivR {
        YDivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - X_DIV"]
    #[inline(always)]
    pub fn x_div(&self) -> XDivR {
        XDivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - N_DIV"]
    #[inline(always)]
    pub fn n_div(&self) -> NDivR {
        NDivR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Y_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn y_div(&mut self) -> YDivW<CdrSpec> {
        YDivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - X_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn x_div(&mut self) -> XDivW<CdrSpec> {
        XDivW::new(self, 8)
    }
    #[doc = "Bits 16:23 - N_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn n_div(&mut self) -> NDivW<CdrSpec> {
        NDivW::new(self, 16)
    }
}
#[doc = "CDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdrSpec;
impl crate::RegisterSpec for CdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr::R`](R) reader structure"]
impl crate::Readable for CdrSpec {}
#[doc = "`write(|w| ..)` method takes [`cdr::W`](W) writer structure"]
impl crate::Writable for CdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CdrSpec {
    const RESET_VALUE: u32 = 0;
}

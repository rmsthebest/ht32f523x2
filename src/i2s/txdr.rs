#[doc = "Register `TXDR` reader"]
pub type R = crate::R<TxdrSpec>;
#[doc = "Register `TXDR` writer"]
pub type W = crate::W<TxdrSpec>;
#[doc = "Field `TXDR` reader - TXDR"]
pub type TxdrR = crate::FieldReader<u32>;
#[doc = "Field `TXDR` writer - TXDR"]
pub type TxdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TXDR"]
    #[inline(always)]
    pub fn txdr(&self) -> TxdrR {
        TxdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TXDR"]
    #[inline(always)]
    #[must_use]
    pub fn txdr(&mut self) -> TxdrW<TxdrSpec> {
        TxdrW::new(self, 0)
    }
}
#[doc = "TXDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdrSpec;
impl crate::RegisterSpec for TxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr::R`](R) reader structure"]
impl crate::Readable for TxdrSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr::W`](W) writer structure"]
impl crate::Writable for TxdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDR to value 0"]
impl crate::Resettable for TxdrSpec {
    const RESET_VALUE: u32 = 0;
}

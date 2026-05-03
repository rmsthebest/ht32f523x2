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
    pub fn txdr(&mut self) -> TxdrW<'_, TxdrSpec> {
        TxdrW::new(self, 0)
    }
}
#[doc = "TXDR\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdrSpec;
impl crate::RegisterSpec for TxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr::R`](R) reader structure"]
impl crate::Readable for TxdrSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr::W`](W) writer structure"]
impl crate::Writable for TxdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDR to value 0"]
impl crate::Resettable for TxdrSpec {}

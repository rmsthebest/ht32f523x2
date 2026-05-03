#[doc = "Register `TXB` reader"]
pub type R = crate::R<TxbSpec>;
#[doc = "Register `TXB` writer"]
pub type W = crate::W<TxbSpec>;
#[doc = "Field `TB` reader - TB"]
pub type TbR = crate::FieldReader;
#[doc = "Field `TB` writer - TB"]
pub type TbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TB"]
    #[inline(always)]
    pub fn tb(&self) -> TbR {
        TbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TB"]
    #[inline(always)]
    pub fn tb(&mut self) -> TbW<'_, TxbSpec> {
        TbW::new(self, 0)
    }
}
#[doc = "TXB\n\nYou can [`read`](crate::Reg::read) this register and get [`txb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbSpec;
impl crate::RegisterSpec for TxbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txb::R`](R) reader structure"]
impl crate::Readable for TxbSpec {}
#[doc = "`write(|w| ..)` method takes [`txb::W`](W) writer structure"]
impl crate::Writable for TxbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXB to value 0"]
impl crate::Resettable for TxbSpec {}

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
    #[must_use]
    pub fn tb(&mut self) -> TbW<TxbSpec> {
        TbW::new(self, 0)
    }
}
#[doc = "TXB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbSpec;
impl crate::RegisterSpec for TxbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txb::R`](R) reader structure"]
impl crate::Readable for TxbSpec {}
#[doc = "`write(|w| ..)` method takes [`txb::W`](W) writer structure"]
impl crate::Writable for TxbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXB to value 0"]
impl crate::Resettable for TxbSpec {
    const RESET_VALUE: u32 = 0;
}

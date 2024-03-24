#[doc = "Register `ADDMR` reader"]
pub type R = crate::R<AddmrSpec>;
#[doc = "Register `ADDMR` writer"]
pub type W = crate::W<AddmrSpec>;
#[doc = "Field `ADDMR` reader - ADDMR"]
pub type AddmrR = crate::FieldReader<u16>;
#[doc = "Field `ADDMR` writer - ADDMR"]
pub type AddmrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - ADDMR"]
    #[inline(always)]
    pub fn addmr(&self) -> AddmrR {
        AddmrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - ADDMR"]
    #[inline(always)]
    #[must_use]
    pub fn addmr(&mut self) -> AddmrW<AddmrSpec> {
        AddmrW::new(self, 0)
    }
}
#[doc = "ADDMR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddmrSpec;
impl crate::RegisterSpec for AddmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addmr::R`](R) reader structure"]
impl crate::Readable for AddmrSpec {}
#[doc = "`write(|w| ..)` method takes [`addmr::W`](W) writer structure"]
impl crate::Writable for AddmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDMR to value 0"]
impl crate::Resettable for AddmrSpec {
    const RESET_VALUE: u32 = 0;
}

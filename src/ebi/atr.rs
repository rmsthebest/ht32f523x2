#[doc = "Register `ATR` reader"]
pub type R = crate::R<AtrSpec>;
#[doc = "Register `ATR` writer"]
pub type W = crate::W<AtrSpec>;
#[doc = "Field `ADDRSETUP` reader - ADDRSETUP"]
pub type AddrsetupR = crate::FieldReader;
#[doc = "Field `ADDRSETUP` writer - ADDRSETUP"]
pub type AddrsetupW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRHOLD` reader - ADDRHOLD"]
pub type AddrholdR = crate::FieldReader;
#[doc = "Field `ADDRHOLD` writer - ADDRHOLD"]
pub type AddrholdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDRSETUP"]
    #[inline(always)]
    pub fn addrsetup(&self) -> AddrsetupR {
        AddrsetupR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRHOLD"]
    #[inline(always)]
    pub fn addrhold(&self) -> AddrholdR {
        AddrholdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRSETUP"]
    #[inline(always)]
    #[must_use]
    pub fn addrsetup(&mut self) -> AddrsetupW<AtrSpec> {
        AddrsetupW::new(self, 0)
    }
    #[doc = "Bits 8:11 - ADDRHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn addrhold(&mut self) -> AddrholdW<AtrSpec> {
        AddrholdW::new(self, 8)
    }
}
#[doc = "ATR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtrSpec;
impl crate::RegisterSpec for AtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atr::R`](R) reader structure"]
impl crate::Readable for AtrSpec {}
#[doc = "`write(|w| ..)` method takes [`atr::W`](W) writer structure"]
impl crate::Writable for AtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATR to value 0"]
impl crate::Resettable for AtrSpec {
    const RESET_VALUE: u32 = 0;
}

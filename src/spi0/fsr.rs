#[doc = "Register `FSR` reader"]
pub type R = crate::R<FsrSpec>;
#[doc = "Register `FSR` writer"]
pub type W = crate::W<FsrSpec>;
#[doc = "Field `TXFS` reader - TXFS"]
pub type TxfsR = crate::FieldReader;
#[doc = "Field `TXFS` writer - TXFS"]
pub type TxfsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RXFS` reader - RXFS"]
pub type RxfsR = crate::FieldReader;
#[doc = "Field `RXFS` writer - RXFS"]
pub type RxfsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TXFS"]
    #[inline(always)]
    pub fn txfs(&self) -> TxfsR {
        TxfsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&self) -> RxfsR {
        RxfsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TXFS"]
    #[inline(always)]
    pub fn txfs(&mut self) -> TxfsW<'_, FsrSpec> {
        TxfsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&mut self) -> RxfsW<'_, FsrSpec> {
        RxfsW::new(self, 4)
    }
}
#[doc = "FSR\n\nYou can [`read`](crate::Reg::read) this register and get [`fsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsrSpec;
impl crate::RegisterSpec for FsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsr::R`](R) reader structure"]
impl crate::Readable for FsrSpec {}
#[doc = "`write(|w| ..)` method takes [`fsr::W`](W) writer structure"]
impl crate::Writable for FsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSR to value 0"]
impl crate::Resettable for FsrSpec {}

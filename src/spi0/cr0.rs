#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `SPIEN` reader - SPIEN"]
pub type SpienR = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPIEN"]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAE` reader - TXDMAE"]
pub type TxdmaeR = crate::BitReader;
#[doc = "Field `TXDMAE` writer - TXDMAE"]
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAE` reader - RXDMAE"]
pub type RxdmaeR = crate::BitReader;
#[doc = "Field `RXDMAE` writer - RXDMAE"]
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELOEN` reader - SELOEN"]
pub type SeloenR = crate::BitReader;
#[doc = "Field `SELOEN` writer - SELOEN"]
pub type SeloenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSELC` reader - SSELC"]
pub type SselcR = crate::BitReader;
#[doc = "Field `SSELC` writer - SSELC"]
pub type SselcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALEN` reader - DUALEN"]
pub type DualenR = crate::BitReader;
#[doc = "Field `DUALEN` writer - DUALEN"]
pub type DualenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GUADTEN` reader - GUADTEN"]
pub type GuadtenR = crate::BitReader;
#[doc = "Field `GUADTEN` writer - GUADTEN"]
pub type GuadtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GUADT` reader - GUADT"]
pub type GuadtR = crate::FieldReader;
#[doc = "Field `GUADT` writer - GUADT"]
pub type GuadtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SELHT` reader - SELHT"]
pub type SelhtR = crate::FieldReader;
#[doc = "Field `SELHT` writer - SELHT"]
pub type SelhtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - SPIEN"]
    #[inline(always)]
    pub fn spien(&self) -> SpienR {
        SpienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXDMAE"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SELOEN"]
    #[inline(always)]
    pub fn seloen(&self) -> SeloenR {
        SeloenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSELC"]
    #[inline(always)]
    pub fn sselc(&self) -> SselcR {
        SselcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - DUALEN"]
    #[inline(always)]
    pub fn dualen(&self) -> DualenR {
        DualenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GUADTEN"]
    #[inline(always)]
    pub fn guadten(&self) -> GuadtenR {
        GuadtenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - GUADT"]
    #[inline(always)]
    pub fn guadt(&self) -> GuadtR {
        GuadtR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SELHT"]
    #[inline(always)]
    pub fn selht(&self) -> SelhtR {
        SelhtR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPIEN"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SpienW<Cr0Spec> {
        SpienW::new(self, 0)
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TxdmaeW<Cr0Spec> {
        TxdmaeW::new(self, 1)
    }
    #[doc = "Bit 2 - RXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RxdmaeW<Cr0Spec> {
        RxdmaeW::new(self, 2)
    }
    #[doc = "Bit 3 - SELOEN"]
    #[inline(always)]
    #[must_use]
    pub fn seloen(&mut self) -> SeloenW<Cr0Spec> {
        SeloenW::new(self, 3)
    }
    #[doc = "Bit 4 - SSELC"]
    #[inline(always)]
    #[must_use]
    pub fn sselc(&mut self) -> SselcW<Cr0Spec> {
        SselcW::new(self, 4)
    }
    #[doc = "Bit 6 - DUALEN"]
    #[inline(always)]
    #[must_use]
    pub fn dualen(&mut self) -> DualenW<Cr0Spec> {
        DualenW::new(self, 6)
    }
    #[doc = "Bit 7 - GUADTEN"]
    #[inline(always)]
    #[must_use]
    pub fn guadten(&mut self) -> GuadtenW<Cr0Spec> {
        GuadtenW::new(self, 7)
    }
    #[doc = "Bits 8:11 - GUADT"]
    #[inline(always)]
    #[must_use]
    pub fn guadt(&mut self) -> GuadtW<Cr0Spec> {
        GuadtW::new(self, 8)
    }
    #[doc = "Bits 12:15 - SELHT"]
    #[inline(always)]
    #[must_use]
    pub fn selht(&mut self) -> SelhtW<Cr0Spec> {
        SelhtW::new(self, 12)
    }
}
#[doc = "CR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARF` reader - PARF"]
pub type PARF_R = crate::BitReader<bool>;
#[doc = "Field `PARF` writer - PARF"]
pub type PARF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `RXCF` reader - RXCF"]
pub type RXCF_R = crate::BitReader<bool>;
#[doc = "Field `RXCF` writer - RXCF"]
pub type RXCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TXCF` reader - TXCF"]
pub type TXCF_R = crate::BitReader<bool>;
#[doc = "Field `TXCF` writer - TXCF"]
pub type TXCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `WTF` reader - WTF"]
pub type WTF_R = crate::BitReader<bool>;
#[doc = "Field `WTF` writer - WTF"]
pub type WTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CPREF` reader - CPREF"]
pub type CPREF_R = crate::BitReader<bool>;
#[doc = "Field `CPREF` writer - CPREF"]
pub type CPREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TXBEF` reader - TXBEF"]
pub type TXBEF_R = crate::BitReader<bool>;
#[doc = "Field `TXBEF` writer - TXBEF"]
pub type TXBEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PARF"]
    #[inline(always)]
    pub fn parf(&self) -> PARF_R {
        PARF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXCF"]
    #[inline(always)]
    pub fn rxcf(&self) -> RXCF_R {
        RXCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXCF"]
    #[inline(always)]
    pub fn txcf(&self) -> TXCF_R {
        TXCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WTF"]
    #[inline(always)]
    pub fn wtf(&self) -> WTF_R {
        WTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - CPREF"]
    #[inline(always)]
    pub fn cpref(&self) -> CPREF_R {
        CPREF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXBEF"]
    #[inline(always)]
    pub fn txbef(&self) -> TXBEF_R {
        TXBEF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PARF"]
    #[inline(always)]
    #[must_use]
    pub fn parf(&mut self) -> PARF_W<0> {
        PARF_W::new(self)
    }
    #[doc = "Bit 1 - RXCF"]
    #[inline(always)]
    #[must_use]
    pub fn rxcf(&mut self) -> RXCF_W<1> {
        RXCF_W::new(self)
    }
    #[doc = "Bit 2 - TXCF"]
    #[inline(always)]
    #[must_use]
    pub fn txcf(&mut self) -> TXCF_W<2> {
        TXCF_W::new(self)
    }
    #[doc = "Bit 3 - WTF"]
    #[inline(always)]
    #[must_use]
    pub fn wtf(&mut self) -> WTF_W<3> {
        WTF_W::new(self)
    }
    #[doc = "Bit 6 - CPREF"]
    #[inline(always)]
    #[must_use]
    pub fn cpref(&mut self) -> CPREF_W<6> {
        CPREF_W::new(self)
    }
    #[doc = "Bit 7 - TXBEF"]
    #[inline(always)]
    #[must_use]
    pub fn txbef(&mut self) -> TXBEF_W<7> {
        TXBEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

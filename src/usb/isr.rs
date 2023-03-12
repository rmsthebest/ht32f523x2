#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFIF` reader - SOFIF"]
pub type SOFIF_R = crate::BitReader<bool>;
#[doc = "Field `SOFIF` writer - SOFIF"]
pub type SOFIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `URSTIF` reader - URSTIF"]
pub type URSTIF_R = crate::BitReader<bool>;
#[doc = "Field `URSTIF` writer - URSTIF"]
pub type URSTIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `RSMIF` reader - RSMIF"]
pub type RSMIF_R = crate::BitReader<bool>;
#[doc = "Field `RSMIF` writer - RSMIF"]
pub type RSMIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `SUSPIF` reader - SUSPIF"]
pub type SUSPIF_R = crate::BitReader<bool>;
#[doc = "Field `SUSPIF` writer - SUSPIF"]
pub type SUSPIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `ESOFIF` reader - ESOFIF"]
pub type ESOFIF_R = crate::BitReader<bool>;
#[doc = "Field `ESOFIF` writer - ESOFIF"]
pub type ESOFIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `EP0IF` reader - EP0IF"]
pub type EP0IF_R = crate::BitReader<bool>;
#[doc = "Field `EP0IF` writer - EP0IF"]
pub type EP0IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `EP1IF` reader - EP1IF"]
pub type EP1IF_R = crate::BitReader<bool>;
#[doc = "Field `EP1IF` writer - EP1IF"]
pub type EP1IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `EP2IF` reader - EP2IF"]
pub type EP2IF_R = crate::BitReader<bool>;
#[doc = "Field `EP2IF` writer - EP2IF"]
pub type EP2IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `EP3IF` reader - EP3IF"]
pub type EP3IF_R = crate::BitReader<bool>;
#[doc = "Field `EP3IF` writer - EP3IF"]
pub type EP3IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `EP4IF` reader - EP4IF"]
pub type EP4IF_R = crate::BitReader<bool>;
#[doc = "Field `EP4IF` writer - EP4IF"]
pub type EP4IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `EP5IF` reader - EP5IF"]
pub type EP5IF_R = crate::BitReader<bool>;
#[doc = "Field `EP5IF` writer - EP5IF"]
pub type EP5IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `EP6IF` reader - EP6IF"]
pub type EP6IF_R = crate::BitReader<bool>;
#[doc = "Field `EP6IF` writer - EP6IF"]
pub type EP6IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `EP7IF` reader - EP7IF"]
pub type EP7IF_R = crate::BitReader<bool>;
#[doc = "Field `EP7IF` writer - EP7IF"]
pub type EP7IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SOFIF"]
    #[inline(always)]
    pub fn sofif(&self) -> SOFIF_R {
        SOFIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - URSTIF"]
    #[inline(always)]
    pub fn urstif(&self) -> URSTIF_R {
        URSTIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSMIF"]
    #[inline(always)]
    pub fn rsmif(&self) -> RSMIF_R {
        RSMIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUSPIF"]
    #[inline(always)]
    pub fn suspif(&self) -> SUSPIF_R {
        SUSPIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ESOFIF"]
    #[inline(always)]
    pub fn esofif(&self) -> ESOFIF_R {
        ESOFIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - EP0IF"]
    #[inline(always)]
    pub fn ep0if(&self) -> EP0IF_R {
        EP0IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EP1IF"]
    #[inline(always)]
    pub fn ep1if(&self) -> EP1IF_R {
        EP1IF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EP2IF"]
    #[inline(always)]
    pub fn ep2if(&self) -> EP2IF_R {
        EP2IF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EP3IF"]
    #[inline(always)]
    pub fn ep3if(&self) -> EP3IF_R {
        EP3IF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EP4IF"]
    #[inline(always)]
    pub fn ep4if(&self) -> EP4IF_R {
        EP4IF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EP5IF"]
    #[inline(always)]
    pub fn ep5if(&self) -> EP5IF_R {
        EP5IF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EP6IF"]
    #[inline(always)]
    pub fn ep6if(&self) -> EP6IF_R {
        EP6IF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EP7IF"]
    #[inline(always)]
    pub fn ep7if(&self) -> EP7IF_R {
        EP7IF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SOFIF"]
    #[inline(always)]
    #[must_use]
    pub fn sofif(&mut self) -> SOFIF_W<1> {
        SOFIF_W::new(self)
    }
    #[doc = "Bit 2 - URSTIF"]
    #[inline(always)]
    #[must_use]
    pub fn urstif(&mut self) -> URSTIF_W<2> {
        URSTIF_W::new(self)
    }
    #[doc = "Bit 3 - RSMIF"]
    #[inline(always)]
    #[must_use]
    pub fn rsmif(&mut self) -> RSMIF_W<3> {
        RSMIF_W::new(self)
    }
    #[doc = "Bit 4 - SUSPIF"]
    #[inline(always)]
    #[must_use]
    pub fn suspif(&mut self) -> SUSPIF_W<4> {
        SUSPIF_W::new(self)
    }
    #[doc = "Bit 5 - ESOFIF"]
    #[inline(always)]
    #[must_use]
    pub fn esofif(&mut self) -> ESOFIF_W<5> {
        ESOFIF_W::new(self)
    }
    #[doc = "Bit 8 - EP0IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep0if(&mut self) -> EP0IF_W<8> {
        EP0IF_W::new(self)
    }
    #[doc = "Bit 9 - EP1IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep1if(&mut self) -> EP1IF_W<9> {
        EP1IF_W::new(self)
    }
    #[doc = "Bit 10 - EP2IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep2if(&mut self) -> EP2IF_W<10> {
        EP2IF_W::new(self)
    }
    #[doc = "Bit 11 - EP3IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep3if(&mut self) -> EP3IF_W<11> {
        EP3IF_W::new(self)
    }
    #[doc = "Bit 12 - EP4IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep4if(&mut self) -> EP4IF_W<12> {
        EP4IF_W::new(self)
    }
    #[doc = "Bit 13 - EP5IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep5if(&mut self) -> EP5IF_W<13> {
        EP5IF_W::new(self)
    }
    #[doc = "Bit 14 - EP6IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep6if(&mut self) -> EP6IF_W<14> {
        EP6IF_W::new(self)
    }
    #[doc = "Bit 15 - EP7IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep7if(&mut self) -> EP7IF_W<15> {
        EP7IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UGIE` reader - UGIE"]
pub type UGIE_R = crate::BitReader<bool>;
#[doc = "Field `UGIE` writer - UGIE"]
pub type UGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `SOFIE` reader - SOFIE"]
pub type SOFIE_R = crate::BitReader<bool>;
#[doc = "Field `SOFIE` writer - SOFIE"]
pub type SOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `URSTIE` reader - URSTIE"]
pub type URSTIE_R = crate::BitReader<bool>;
#[doc = "Field `URSTIE` writer - URSTIE"]
pub type URSTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RSMIE` reader - RSMIE"]
pub type RSMIE_R = crate::BitReader<bool>;
#[doc = "Field `RSMIE` writer - RSMIE"]
pub type RSMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `SUSPIE` reader - SUSPIE"]
pub type SUSPIE_R = crate::BitReader<bool>;
#[doc = "Field `SUSPIE` writer - SUSPIE"]
pub type SUSPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `ESOFIE` reader - ESOFIE"]
pub type ESOFIE_R = crate::BitReader<bool>;
#[doc = "Field `ESOFIE` writer - ESOFIE"]
pub type ESOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP0IE` reader - EP0IE"]
pub type EP0IE_R = crate::BitReader<bool>;
#[doc = "Field `EP0IE` writer - EP0IE"]
pub type EP0IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP1IE` reader - EP1IE"]
pub type EP1IE_R = crate::BitReader<bool>;
#[doc = "Field `EP1IE` writer - EP1IE"]
pub type EP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP2IE` reader - EP2IE"]
pub type EP2IE_R = crate::BitReader<bool>;
#[doc = "Field `EP2IE` writer - EP2IE"]
pub type EP2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP3IE` reader - EP3IE"]
pub type EP3IE_R = crate::BitReader<bool>;
#[doc = "Field `EP3IE` writer - EP3IE"]
pub type EP3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP4IE` reader - EP4IE"]
pub type EP4IE_R = crate::BitReader<bool>;
#[doc = "Field `EP4IE` writer - EP4IE"]
pub type EP4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP5IE` reader - EP5IE"]
pub type EP5IE_R = crate::BitReader<bool>;
#[doc = "Field `EP5IE` writer - EP5IE"]
pub type EP5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP6IE` reader - EP6IE"]
pub type EP6IE_R = crate::BitReader<bool>;
#[doc = "Field `EP6IE` writer - EP6IE"]
pub type EP6IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP7IE` reader - EP7IE"]
pub type EP7IE_R = crate::BitReader<bool>;
#[doc = "Field `EP7IE` writer - EP7IE"]
pub type EP7IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UGIE"]
    #[inline(always)]
    pub fn ugie(&self) -> UGIE_R {
        UGIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOFIE"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - URSTIE"]
    #[inline(always)]
    pub fn urstie(&self) -> URSTIE_R {
        URSTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSMIE"]
    #[inline(always)]
    pub fn rsmie(&self) -> RSMIE_R {
        RSMIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUSPIE"]
    #[inline(always)]
    pub fn suspie(&self) -> SUSPIE_R {
        SUSPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ESOFIE"]
    #[inline(always)]
    pub fn esofie(&self) -> ESOFIE_R {
        ESOFIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - EP0IE"]
    #[inline(always)]
    pub fn ep0ie(&self) -> EP0IE_R {
        EP0IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EP1IE"]
    #[inline(always)]
    pub fn ep1ie(&self) -> EP1IE_R {
        EP1IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EP2IE"]
    #[inline(always)]
    pub fn ep2ie(&self) -> EP2IE_R {
        EP2IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EP3IE"]
    #[inline(always)]
    pub fn ep3ie(&self) -> EP3IE_R {
        EP3IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EP4IE"]
    #[inline(always)]
    pub fn ep4ie(&self) -> EP4IE_R {
        EP4IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EP5IE"]
    #[inline(always)]
    pub fn ep5ie(&self) -> EP5IE_R {
        EP5IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EP6IE"]
    #[inline(always)]
    pub fn ep6ie(&self) -> EP6IE_R {
        EP6IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EP7IE"]
    #[inline(always)]
    pub fn ep7ie(&self) -> EP7IE_R {
        EP7IE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UGIE"]
    #[inline(always)]
    #[must_use]
    pub fn ugie(&mut self) -> UGIE_W<0> {
        UGIE_W::new(self)
    }
    #[doc = "Bit 1 - SOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SOFIE_W<1> {
        SOFIE_W::new(self)
    }
    #[doc = "Bit 2 - URSTIE"]
    #[inline(always)]
    #[must_use]
    pub fn urstie(&mut self) -> URSTIE_W<2> {
        URSTIE_W::new(self)
    }
    #[doc = "Bit 3 - RSMIE"]
    #[inline(always)]
    #[must_use]
    pub fn rsmie(&mut self) -> RSMIE_W<3> {
        RSMIE_W::new(self)
    }
    #[doc = "Bit 4 - SUSPIE"]
    #[inline(always)]
    #[must_use]
    pub fn suspie(&mut self) -> SUSPIE_W<4> {
        SUSPIE_W::new(self)
    }
    #[doc = "Bit 5 - ESOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn esofie(&mut self) -> ESOFIE_W<5> {
        ESOFIE_W::new(self)
    }
    #[doc = "Bit 8 - EP0IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep0ie(&mut self) -> EP0IE_W<8> {
        EP0IE_W::new(self)
    }
    #[doc = "Bit 9 - EP1IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep1ie(&mut self) -> EP1IE_W<9> {
        EP1IE_W::new(self)
    }
    #[doc = "Bit 10 - EP2IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep2ie(&mut self) -> EP2IE_W<10> {
        EP2IE_W::new(self)
    }
    #[doc = "Bit 11 - EP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep3ie(&mut self) -> EP3IE_W<11> {
        EP3IE_W::new(self)
    }
    #[doc = "Bit 12 - EP4IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep4ie(&mut self) -> EP4IE_W<12> {
        EP4IE_W::new(self)
    }
    #[doc = "Bit 13 - EP5IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep5ie(&mut self) -> EP5IE_W<13> {
        EP5IE_W::new(self)
    }
    #[doc = "Bit 14 - EP6IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep6ie(&mut self) -> EP6IE_W<14> {
        EP6IE_W::new(self)
    }
    #[doc = "Bit 15 - EP7IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep7ie(&mut self) -> EP7IE_W<15> {
        EP7IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

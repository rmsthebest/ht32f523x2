#[doc = "Register `EP5IER` reader"]
pub struct R(crate::R<EP5IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP5IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP5IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP5IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP5IER` writer"]
pub struct W(crate::W<EP5IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP5IER_SPEC>;
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
impl From<crate::W<EP5IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP5IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTRXIE` reader - OTRXIE"]
pub type OTRXIE_R = crate::BitReader<bool>;
#[doc = "Field `OTRXIE` writer - OTRXIE"]
pub type OTRXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5IER_SPEC, bool, O>;
#[doc = "Field `ODRXIE` reader - ODRXIE"]
pub type ODRXIE_R = crate::BitReader<bool>;
#[doc = "Field `ODRXIE` writer - ODRXIE"]
pub type ODRXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5IER_SPEC, bool, O>;
#[doc = "Field `ODOVIE` reader - ODOVIE"]
pub type ODOVIE_R = crate::BitReader<bool>;
#[doc = "Field `ODOVIE` writer - ODOVIE"]
pub type ODOVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5IER_SPEC, bool, O>;
#[doc = "Field `ITRXIE` reader - ITRXIE"]
pub type ITRXIE_R = crate::BitReader<bool>;
#[doc = "Field `ITRXIE` writer - ITRXIE"]
pub type ITRXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5IER_SPEC, bool, O>;
#[doc = "Field `IDTXIE` reader - IDTXIE"]
pub type IDTXIE_R = crate::BitReader<bool>;
#[doc = "Field `IDTXIE` writer - IDTXIE"]
pub type IDTXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5IER_SPEC, bool, O>;
#[doc = "Field `NAKIE` reader - NAKIE"]
pub type NAKIE_R = crate::BitReader<bool>;
#[doc = "Field `NAKIE` writer - NAKIE"]
pub type NAKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5IER_SPEC, bool, O>;
#[doc = "Field `STLIE` reader - STLIE"]
pub type STLIE_R = crate::BitReader<bool>;
#[doc = "Field `STLIE` writer - STLIE"]
pub type STLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5IER_SPEC, bool, O>;
#[doc = "Field `UERIE` reader - UERIE"]
pub type UERIE_R = crate::BitReader<bool>;
#[doc = "Field `UERIE` writer - UERIE"]
pub type UERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OTRXIE"]
    #[inline(always)]
    pub fn otrxie(&self) -> OTRXIE_R {
        OTRXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ODRXIE"]
    #[inline(always)]
    pub fn odrxie(&self) -> ODRXIE_R {
        ODRXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ODOVIE"]
    #[inline(always)]
    pub fn odovie(&self) -> ODOVIE_R {
        ODOVIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ITRXIE"]
    #[inline(always)]
    pub fn itrxie(&self) -> ITRXIE_R {
        ITRXIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDTXIE"]
    #[inline(always)]
    pub fn idtxie(&self) -> IDTXIE_R {
        IDTXIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NAKIE"]
    #[inline(always)]
    pub fn nakie(&self) -> NAKIE_R {
        NAKIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STLIE"]
    #[inline(always)]
    pub fn stlie(&self) -> STLIE_R {
        STLIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UERIE"]
    #[inline(always)]
    pub fn uerie(&self) -> UERIE_R {
        UERIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn otrxie(&mut self) -> OTRXIE_W<0> {
        OTRXIE_W::new(self)
    }
    #[doc = "Bit 1 - ODRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn odrxie(&mut self) -> ODRXIE_W<1> {
        ODRXIE_W::new(self)
    }
    #[doc = "Bit 2 - ODOVIE"]
    #[inline(always)]
    #[must_use]
    pub fn odovie(&mut self) -> ODOVIE_W<2> {
        ODOVIE_W::new(self)
    }
    #[doc = "Bit 3 - ITRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn itrxie(&mut self) -> ITRXIE_W<3> {
        ITRXIE_W::new(self)
    }
    #[doc = "Bit 4 - IDTXIE"]
    #[inline(always)]
    #[must_use]
    pub fn idtxie(&mut self) -> IDTXIE_W<4> {
        IDTXIE_W::new(self)
    }
    #[doc = "Bit 5 - NAKIE"]
    #[inline(always)]
    #[must_use]
    pub fn nakie(&mut self) -> NAKIE_W<5> {
        NAKIE_W::new(self)
    }
    #[doc = "Bit 6 - STLIE"]
    #[inline(always)]
    #[must_use]
    pub fn stlie(&mut self) -> STLIE_W<6> {
        STLIE_W::new(self)
    }
    #[doc = "Bit 7 - UERIE"]
    #[inline(always)]
    #[must_use]
    pub fn uerie(&mut self) -> UERIE_W<7> {
        UERIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EP5IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5ier](index.html) module"]
pub struct EP5IER_SPEC;
impl crate::RegisterSpec for EP5IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep5ier::R](R) reader structure"]
impl crate::Readable for EP5IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep5ier::W](W) writer structure"]
impl crate::Writable for EP5IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP5IER to value 0"]
impl crate::Resettable for EP5IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

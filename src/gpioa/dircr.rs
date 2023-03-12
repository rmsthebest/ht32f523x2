#[doc = "Register `DIRCR` reader"]
pub struct R(crate::R<DIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIRCR` writer"]
pub struct W(crate::W<DIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRCR_SPEC>;
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
impl From<crate::W<DIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR0` reader - DIR0"]
pub type DIR0_R = crate::BitReader<bool>;
#[doc = "Field `DIR0` writer - DIR0"]
pub type DIR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR1` reader - DIR1"]
pub type DIR1_R = crate::BitReader<bool>;
#[doc = "Field `DIR1` writer - DIR1"]
pub type DIR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR2` reader - DIR2"]
pub type DIR2_R = crate::BitReader<bool>;
#[doc = "Field `DIR2` writer - DIR2"]
pub type DIR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR3` reader - DIR3"]
pub type DIR3_R = crate::BitReader<bool>;
#[doc = "Field `DIR3` writer - DIR3"]
pub type DIR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR4` reader - DIR4"]
pub type DIR4_R = crate::BitReader<bool>;
#[doc = "Field `DIR4` writer - DIR4"]
pub type DIR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR5` reader - DIR5"]
pub type DIR5_R = crate::BitReader<bool>;
#[doc = "Field `DIR5` writer - DIR5"]
pub type DIR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR6` reader - DIR6"]
pub type DIR6_R = crate::BitReader<bool>;
#[doc = "Field `DIR6` writer - DIR6"]
pub type DIR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR7` reader - DIR7"]
pub type DIR7_R = crate::BitReader<bool>;
#[doc = "Field `DIR7` writer - DIR7"]
pub type DIR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR8` reader - DIR8"]
pub type DIR8_R = crate::BitReader<bool>;
#[doc = "Field `DIR8` writer - DIR8"]
pub type DIR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR9` reader - DIR9"]
pub type DIR9_R = crate::BitReader<bool>;
#[doc = "Field `DIR9` writer - DIR9"]
pub type DIR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR10` reader - DIR10"]
pub type DIR10_R = crate::BitReader<bool>;
#[doc = "Field `DIR10` writer - DIR10"]
pub type DIR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR11` reader - DIR11"]
pub type DIR11_R = crate::BitReader<bool>;
#[doc = "Field `DIR11` writer - DIR11"]
pub type DIR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR12` reader - DIR12"]
pub type DIR12_R = crate::BitReader<bool>;
#[doc = "Field `DIR12` writer - DIR12"]
pub type DIR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR13` reader - DIR13"]
pub type DIR13_R = crate::BitReader<bool>;
#[doc = "Field `DIR13` writer - DIR13"]
pub type DIR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR14` reader - DIR14"]
pub type DIR14_R = crate::BitReader<bool>;
#[doc = "Field `DIR14` writer - DIR14"]
pub type DIR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
#[doc = "Field `DIR15` reader - DIR15"]
pub type DIR15_R = crate::BitReader<bool>;
#[doc = "Field `DIR15` writer - DIR15"]
pub type DIR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DIR0"]
    #[inline(always)]
    pub fn dir0(&self) -> DIR0_R {
        DIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIR1"]
    #[inline(always)]
    pub fn dir1(&self) -> DIR1_R {
        DIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIR2"]
    #[inline(always)]
    pub fn dir2(&self) -> DIR2_R {
        DIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIR3"]
    #[inline(always)]
    pub fn dir3(&self) -> DIR3_R {
        DIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIR4"]
    #[inline(always)]
    pub fn dir4(&self) -> DIR4_R {
        DIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIR5"]
    #[inline(always)]
    pub fn dir5(&self) -> DIR5_R {
        DIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIR6"]
    #[inline(always)]
    pub fn dir6(&self) -> DIR6_R {
        DIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIR7"]
    #[inline(always)]
    pub fn dir7(&self) -> DIR7_R {
        DIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIR8"]
    #[inline(always)]
    pub fn dir8(&self) -> DIR8_R {
        DIR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIR9"]
    #[inline(always)]
    pub fn dir9(&self) -> DIR9_R {
        DIR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIR10"]
    #[inline(always)]
    pub fn dir10(&self) -> DIR10_R {
        DIR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIR11"]
    #[inline(always)]
    pub fn dir11(&self) -> DIR11_R {
        DIR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIR12"]
    #[inline(always)]
    pub fn dir12(&self) -> DIR12_R {
        DIR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIR13"]
    #[inline(always)]
    pub fn dir13(&self) -> DIR13_R {
        DIR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIR14"]
    #[inline(always)]
    pub fn dir14(&self) -> DIR14_R {
        DIR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIR15"]
    #[inline(always)]
    pub fn dir15(&self) -> DIR15_R {
        DIR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIR0"]
    #[inline(always)]
    #[must_use]
    pub fn dir0(&mut self) -> DIR0_W<0> {
        DIR0_W::new(self)
    }
    #[doc = "Bit 1 - DIR1"]
    #[inline(always)]
    #[must_use]
    pub fn dir1(&mut self) -> DIR1_W<1> {
        DIR1_W::new(self)
    }
    #[doc = "Bit 2 - DIR2"]
    #[inline(always)]
    #[must_use]
    pub fn dir2(&mut self) -> DIR2_W<2> {
        DIR2_W::new(self)
    }
    #[doc = "Bit 3 - DIR3"]
    #[inline(always)]
    #[must_use]
    pub fn dir3(&mut self) -> DIR3_W<3> {
        DIR3_W::new(self)
    }
    #[doc = "Bit 4 - DIR4"]
    #[inline(always)]
    #[must_use]
    pub fn dir4(&mut self) -> DIR4_W<4> {
        DIR4_W::new(self)
    }
    #[doc = "Bit 5 - DIR5"]
    #[inline(always)]
    #[must_use]
    pub fn dir5(&mut self) -> DIR5_W<5> {
        DIR5_W::new(self)
    }
    #[doc = "Bit 6 - DIR6"]
    #[inline(always)]
    #[must_use]
    pub fn dir6(&mut self) -> DIR6_W<6> {
        DIR6_W::new(self)
    }
    #[doc = "Bit 7 - DIR7"]
    #[inline(always)]
    #[must_use]
    pub fn dir7(&mut self) -> DIR7_W<7> {
        DIR7_W::new(self)
    }
    #[doc = "Bit 8 - DIR8"]
    #[inline(always)]
    #[must_use]
    pub fn dir8(&mut self) -> DIR8_W<8> {
        DIR8_W::new(self)
    }
    #[doc = "Bit 9 - DIR9"]
    #[inline(always)]
    #[must_use]
    pub fn dir9(&mut self) -> DIR9_W<9> {
        DIR9_W::new(self)
    }
    #[doc = "Bit 10 - DIR10"]
    #[inline(always)]
    #[must_use]
    pub fn dir10(&mut self) -> DIR10_W<10> {
        DIR10_W::new(self)
    }
    #[doc = "Bit 11 - DIR11"]
    #[inline(always)]
    #[must_use]
    pub fn dir11(&mut self) -> DIR11_W<11> {
        DIR11_W::new(self)
    }
    #[doc = "Bit 12 - DIR12"]
    #[inline(always)]
    #[must_use]
    pub fn dir12(&mut self) -> DIR12_W<12> {
        DIR12_W::new(self)
    }
    #[doc = "Bit 13 - DIR13"]
    #[inline(always)]
    #[must_use]
    pub fn dir13(&mut self) -> DIR13_W<13> {
        DIR13_W::new(self)
    }
    #[doc = "Bit 14 - DIR14"]
    #[inline(always)]
    #[must_use]
    pub fn dir14(&mut self) -> DIR14_W<14> {
        DIR14_W::new(self)
    }
    #[doc = "Bit 15 - DIR15"]
    #[inline(always)]
    #[must_use]
    pub fn dir15(&mut self) -> DIR15_W<15> {
        DIR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dircr](index.html) module"]
pub struct DIRCR_SPEC;
impl crate::RegisterSpec for DIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dircr::R](R) reader structure"]
impl crate::Readable for DIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dircr::W](W) writer structure"]
impl crate::Writable for DIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRCR to value 0"]
impl crate::Resettable for DIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

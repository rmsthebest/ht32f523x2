#[doc = "Register `DINR` reader"]
pub struct R(crate::R<DINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DINR` writer"]
pub struct W(crate::W<DINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DINR_SPEC>;
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
impl From<crate::W<DINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIN0` reader - DIN0"]
pub type DIN0_R = crate::BitReader<bool>;
#[doc = "Field `DIN0` writer - DIN0"]
pub type DIN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN1` reader - DIN1"]
pub type DIN1_R = crate::BitReader<bool>;
#[doc = "Field `DIN1` writer - DIN1"]
pub type DIN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN2` reader - DIN2"]
pub type DIN2_R = crate::BitReader<bool>;
#[doc = "Field `DIN2` writer - DIN2"]
pub type DIN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN3` reader - DIN3"]
pub type DIN3_R = crate::BitReader<bool>;
#[doc = "Field `DIN3` writer - DIN3"]
pub type DIN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN4` reader - DIN4"]
pub type DIN4_R = crate::BitReader<bool>;
#[doc = "Field `DIN4` writer - DIN4"]
pub type DIN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN5` reader - DIN5"]
pub type DIN5_R = crate::BitReader<bool>;
#[doc = "Field `DIN5` writer - DIN5"]
pub type DIN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN6` reader - DIN6"]
pub type DIN6_R = crate::BitReader<bool>;
#[doc = "Field `DIN6` writer - DIN6"]
pub type DIN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN7` reader - DIN7"]
pub type DIN7_R = crate::BitReader<bool>;
#[doc = "Field `DIN7` writer - DIN7"]
pub type DIN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN8` reader - DIN8"]
pub type DIN8_R = crate::BitReader<bool>;
#[doc = "Field `DIN8` writer - DIN8"]
pub type DIN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN9` reader - DIN9"]
pub type DIN9_R = crate::BitReader<bool>;
#[doc = "Field `DIN9` writer - DIN9"]
pub type DIN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN10` reader - DIN10"]
pub type DIN10_R = crate::BitReader<bool>;
#[doc = "Field `DIN10` writer - DIN10"]
pub type DIN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN11` reader - DIN11"]
pub type DIN11_R = crate::BitReader<bool>;
#[doc = "Field `DIN11` writer - DIN11"]
pub type DIN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN12` reader - DIN12"]
pub type DIN12_R = crate::BitReader<bool>;
#[doc = "Field `DIN12` writer - DIN12"]
pub type DIN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN13` reader - DIN13"]
pub type DIN13_R = crate::BitReader<bool>;
#[doc = "Field `DIN13` writer - DIN13"]
pub type DIN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN14` reader - DIN14"]
pub type DIN14_R = crate::BitReader<bool>;
#[doc = "Field `DIN14` writer - DIN14"]
pub type DIN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
#[doc = "Field `DIN15` reader - DIN15"]
pub type DIN15_R = crate::BitReader<bool>;
#[doc = "Field `DIN15` writer - DIN15"]
pub type DIN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DINR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DIN0"]
    #[inline(always)]
    pub fn din0(&self) -> DIN0_R {
        DIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIN1"]
    #[inline(always)]
    pub fn din1(&self) -> DIN1_R {
        DIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIN2"]
    #[inline(always)]
    pub fn din2(&self) -> DIN2_R {
        DIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIN3"]
    #[inline(always)]
    pub fn din3(&self) -> DIN3_R {
        DIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIN4"]
    #[inline(always)]
    pub fn din4(&self) -> DIN4_R {
        DIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIN5"]
    #[inline(always)]
    pub fn din5(&self) -> DIN5_R {
        DIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIN6"]
    #[inline(always)]
    pub fn din6(&self) -> DIN6_R {
        DIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIN7"]
    #[inline(always)]
    pub fn din7(&self) -> DIN7_R {
        DIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIN8"]
    #[inline(always)]
    pub fn din8(&self) -> DIN8_R {
        DIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIN9"]
    #[inline(always)]
    pub fn din9(&self) -> DIN9_R {
        DIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIN10"]
    #[inline(always)]
    pub fn din10(&self) -> DIN10_R {
        DIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIN11"]
    #[inline(always)]
    pub fn din11(&self) -> DIN11_R {
        DIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIN12"]
    #[inline(always)]
    pub fn din12(&self) -> DIN12_R {
        DIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIN13"]
    #[inline(always)]
    pub fn din13(&self) -> DIN13_R {
        DIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIN14"]
    #[inline(always)]
    pub fn din14(&self) -> DIN14_R {
        DIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIN15"]
    #[inline(always)]
    pub fn din15(&self) -> DIN15_R {
        DIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIN0"]
    #[inline(always)]
    #[must_use]
    pub fn din0(&mut self) -> DIN0_W<0> {
        DIN0_W::new(self)
    }
    #[doc = "Bit 1 - DIN1"]
    #[inline(always)]
    #[must_use]
    pub fn din1(&mut self) -> DIN1_W<1> {
        DIN1_W::new(self)
    }
    #[doc = "Bit 2 - DIN2"]
    #[inline(always)]
    #[must_use]
    pub fn din2(&mut self) -> DIN2_W<2> {
        DIN2_W::new(self)
    }
    #[doc = "Bit 3 - DIN3"]
    #[inline(always)]
    #[must_use]
    pub fn din3(&mut self) -> DIN3_W<3> {
        DIN3_W::new(self)
    }
    #[doc = "Bit 4 - DIN4"]
    #[inline(always)]
    #[must_use]
    pub fn din4(&mut self) -> DIN4_W<4> {
        DIN4_W::new(self)
    }
    #[doc = "Bit 5 - DIN5"]
    #[inline(always)]
    #[must_use]
    pub fn din5(&mut self) -> DIN5_W<5> {
        DIN5_W::new(self)
    }
    #[doc = "Bit 6 - DIN6"]
    #[inline(always)]
    #[must_use]
    pub fn din6(&mut self) -> DIN6_W<6> {
        DIN6_W::new(self)
    }
    #[doc = "Bit 7 - DIN7"]
    #[inline(always)]
    #[must_use]
    pub fn din7(&mut self) -> DIN7_W<7> {
        DIN7_W::new(self)
    }
    #[doc = "Bit 8 - DIN8"]
    #[inline(always)]
    #[must_use]
    pub fn din8(&mut self) -> DIN8_W<8> {
        DIN8_W::new(self)
    }
    #[doc = "Bit 9 - DIN9"]
    #[inline(always)]
    #[must_use]
    pub fn din9(&mut self) -> DIN9_W<9> {
        DIN9_W::new(self)
    }
    #[doc = "Bit 10 - DIN10"]
    #[inline(always)]
    #[must_use]
    pub fn din10(&mut self) -> DIN10_W<10> {
        DIN10_W::new(self)
    }
    #[doc = "Bit 11 - DIN11"]
    #[inline(always)]
    #[must_use]
    pub fn din11(&mut self) -> DIN11_W<11> {
        DIN11_W::new(self)
    }
    #[doc = "Bit 12 - DIN12"]
    #[inline(always)]
    #[must_use]
    pub fn din12(&mut self) -> DIN12_W<12> {
        DIN12_W::new(self)
    }
    #[doc = "Bit 13 - DIN13"]
    #[inline(always)]
    #[must_use]
    pub fn din13(&mut self) -> DIN13_W<13> {
        DIN13_W::new(self)
    }
    #[doc = "Bit 14 - DIN14"]
    #[inline(always)]
    #[must_use]
    pub fn din14(&mut self) -> DIN14_W<14> {
        DIN14_W::new(self)
    }
    #[doc = "Bit 15 - DIN15"]
    #[inline(always)]
    #[must_use]
    pub fn din15(&mut self) -> DIN15_W<15> {
        DIN15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr](index.html) module"]
pub struct DINR_SPEC;
impl crate::RegisterSpec for DINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dinr::R](R) reader structure"]
impl crate::Readable for DINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dinr::W](W) writer structure"]
impl crate::Writable for DINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DINR to value 0"]
impl crate::Resettable for DINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

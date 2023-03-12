#[doc = "Register `ISCR` reader"]
pub struct R(crate::R<ISCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISCR` writer"]
pub struct W(crate::W<ISCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISCR_SPEC>;
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
impl From<crate::W<ISCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEICLR0` reader - GEICLR0"]
pub type GEICLR0_R = crate::BitReader<bool>;
#[doc = "Field `GEICLR0` writer - GEICLR0"]
pub type GEICLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `BEICLR0` reader - BEICLR0"]
pub type BEICLR0_R = crate::BitReader<bool>;
#[doc = "Field `BEICLR0` writer - BEICLR0"]
pub type BEICLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `HTICLR0` reader - HTICLR0"]
pub type HTICLR0_R = crate::BitReader<bool>;
#[doc = "Field `HTICLR0` writer - HTICLR0"]
pub type HTICLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TCICLR0` reader - TCICLR0"]
pub type TCICLR0_R = crate::BitReader<bool>;
#[doc = "Field `TCICLR0` writer - TCICLR0"]
pub type TCICLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TEICLR0` reader - TEICLR0"]
pub type TEICLR0_R = crate::BitReader<bool>;
#[doc = "Field `TEICLR0` writer - TEICLR0"]
pub type TEICLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `GEICLR1` reader - GEICLR1"]
pub type GEICLR1_R = crate::BitReader<bool>;
#[doc = "Field `GEICLR1` writer - GEICLR1"]
pub type GEICLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `BEICLR1` reader - BEICLR1"]
pub type BEICLR1_R = crate::BitReader<bool>;
#[doc = "Field `BEICLR1` writer - BEICLR1"]
pub type BEICLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `HTICLR1` reader - HTICLR1"]
pub type HTICLR1_R = crate::BitReader<bool>;
#[doc = "Field `HTICLR1` writer - HTICLR1"]
pub type HTICLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TCICLR1` reader - TCICLR1"]
pub type TCICLR1_R = crate::BitReader<bool>;
#[doc = "Field `TCICLR1` writer - TCICLR1"]
pub type TCICLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TEICLR1` reader - TEICLR1"]
pub type TEICLR1_R = crate::BitReader<bool>;
#[doc = "Field `TEICLR1` writer - TEICLR1"]
pub type TEICLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `GEICLR2` reader - GEICLR2"]
pub type GEICLR2_R = crate::BitReader<bool>;
#[doc = "Field `GEICLR2` writer - GEICLR2"]
pub type GEICLR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `BEICLR2` reader - BEICLR2"]
pub type BEICLR2_R = crate::BitReader<bool>;
#[doc = "Field `BEICLR2` writer - BEICLR2"]
pub type BEICLR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `HTICLR2` reader - HTICLR2"]
pub type HTICLR2_R = crate::BitReader<bool>;
#[doc = "Field `HTICLR2` writer - HTICLR2"]
pub type HTICLR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TCICLR2` reader - TCICLR2"]
pub type TCICLR2_R = crate::BitReader<bool>;
#[doc = "Field `TCICLR2` writer - TCICLR2"]
pub type TCICLR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TEICLR2` reader - TEICLR2"]
pub type TEICLR2_R = crate::BitReader<bool>;
#[doc = "Field `TEICLR2` writer - TEICLR2"]
pub type TEICLR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `GEICLR3` reader - GEICLR3"]
pub type GEICLR3_R = crate::BitReader<bool>;
#[doc = "Field `GEICLR3` writer - GEICLR3"]
pub type GEICLR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `BEICLR3` reader - BEICLR3"]
pub type BEICLR3_R = crate::BitReader<bool>;
#[doc = "Field `BEICLR3` writer - BEICLR3"]
pub type BEICLR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `HTICLR3` reader - HTICLR3"]
pub type HTICLR3_R = crate::BitReader<bool>;
#[doc = "Field `HTICLR3` writer - HTICLR3"]
pub type HTICLR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TCICLR3` reader - TCICLR3"]
pub type TCICLR3_R = crate::BitReader<bool>;
#[doc = "Field `TCICLR3` writer - TCICLR3"]
pub type TCICLR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TEICLR3` reader - TEICLR3"]
pub type TEICLR3_R = crate::BitReader<bool>;
#[doc = "Field `TEICLR3` writer - TEICLR3"]
pub type TEICLR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `GEICLR4` reader - GEICLR4"]
pub type GEICLR4_R = crate::BitReader<bool>;
#[doc = "Field `GEICLR4` writer - GEICLR4"]
pub type GEICLR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `BEICLR4` reader - BEICLR4"]
pub type BEICLR4_R = crate::BitReader<bool>;
#[doc = "Field `BEICLR4` writer - BEICLR4"]
pub type BEICLR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `HTICLR4` reader - HTICLR4"]
pub type HTICLR4_R = crate::BitReader<bool>;
#[doc = "Field `HTICLR4` writer - HTICLR4"]
pub type HTICLR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TCICLR4` reader - TCICLR4"]
pub type TCICLR4_R = crate::BitReader<bool>;
#[doc = "Field `TCICLR4` writer - TCICLR4"]
pub type TCICLR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TEICLR4` reader - TEICLR4"]
pub type TEICLR4_R = crate::BitReader<bool>;
#[doc = "Field `TEICLR4` writer - TEICLR4"]
pub type TEICLR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `GEICLR5` reader - GEICLR5"]
pub type GEICLR5_R = crate::BitReader<bool>;
#[doc = "Field `GEICLR5` writer - GEICLR5"]
pub type GEICLR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `BEICLR5` reader - BEICLR5"]
pub type BEICLR5_R = crate::BitReader<bool>;
#[doc = "Field `BEICLR5` writer - BEICLR5"]
pub type BEICLR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `HTICLR5` reader - HTICLR5"]
pub type HTICLR5_R = crate::BitReader<bool>;
#[doc = "Field `HTICLR5` writer - HTICLR5"]
pub type HTICLR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TCICLR5` reader - TCICLR5"]
pub type TCICLR5_R = crate::BitReader<bool>;
#[doc = "Field `TCICLR5` writer - TCICLR5"]
pub type TCICLR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
#[doc = "Field `TEICLR5` reader - TEICLR5"]
pub type TEICLR5_R = crate::BitReader<bool>;
#[doc = "Field `TEICLR5` writer - TEICLR5"]
pub type TEICLR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GEICLR0"]
    #[inline(always)]
    pub fn geiclr0(&self) -> GEICLR0_R {
        GEICLR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BEICLR0"]
    #[inline(always)]
    pub fn beiclr0(&self) -> BEICLR0_R {
        BEICLR0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTICLR0"]
    #[inline(always)]
    pub fn hticlr0(&self) -> HTICLR0_R {
        HTICLR0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TCICLR0"]
    #[inline(always)]
    pub fn tciclr0(&self) -> TCICLR0_R {
        TCICLR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TEICLR0"]
    #[inline(always)]
    pub fn teiclr0(&self) -> TEICLR0_R {
        TEICLR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GEICLR1"]
    #[inline(always)]
    pub fn geiclr1(&self) -> GEICLR1_R {
        GEICLR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BEICLR1"]
    #[inline(always)]
    pub fn beiclr1(&self) -> BEICLR1_R {
        BEICLR1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HTICLR1"]
    #[inline(always)]
    pub fn hticlr1(&self) -> HTICLR1_R {
        HTICLR1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TCICLR1"]
    #[inline(always)]
    pub fn tciclr1(&self) -> TCICLR1_R {
        TCICLR1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TEICLR1"]
    #[inline(always)]
    pub fn teiclr1(&self) -> TEICLR1_R {
        TEICLR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GEICLR2"]
    #[inline(always)]
    pub fn geiclr2(&self) -> GEICLR2_R {
        GEICLR2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BEICLR2"]
    #[inline(always)]
    pub fn beiclr2(&self) -> BEICLR2_R {
        BEICLR2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HTICLR2"]
    #[inline(always)]
    pub fn hticlr2(&self) -> HTICLR2_R {
        HTICLR2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TCICLR2"]
    #[inline(always)]
    pub fn tciclr2(&self) -> TCICLR2_R {
        TCICLR2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TEICLR2"]
    #[inline(always)]
    pub fn teiclr2(&self) -> TEICLR2_R {
        TEICLR2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GEICLR3"]
    #[inline(always)]
    pub fn geiclr3(&self) -> GEICLR3_R {
        GEICLR3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BEICLR3"]
    #[inline(always)]
    pub fn beiclr3(&self) -> BEICLR3_R {
        BEICLR3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HTICLR3"]
    #[inline(always)]
    pub fn hticlr3(&self) -> HTICLR3_R {
        HTICLR3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TCICLR3"]
    #[inline(always)]
    pub fn tciclr3(&self) -> TCICLR3_R {
        TCICLR3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TEICLR3"]
    #[inline(always)]
    pub fn teiclr3(&self) -> TEICLR3_R {
        TEICLR3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GEICLR4"]
    #[inline(always)]
    pub fn geiclr4(&self) -> GEICLR4_R {
        GEICLR4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BEICLR4"]
    #[inline(always)]
    pub fn beiclr4(&self) -> BEICLR4_R {
        BEICLR4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HTICLR4"]
    #[inline(always)]
    pub fn hticlr4(&self) -> HTICLR4_R {
        HTICLR4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TCICLR4"]
    #[inline(always)]
    pub fn tciclr4(&self) -> TCICLR4_R {
        TCICLR4_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TEICLR4"]
    #[inline(always)]
    pub fn teiclr4(&self) -> TEICLR4_R {
        TEICLR4_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GEICLR5"]
    #[inline(always)]
    pub fn geiclr5(&self) -> GEICLR5_R {
        GEICLR5_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BEICLR5"]
    #[inline(always)]
    pub fn beiclr5(&self) -> BEICLR5_R {
        BEICLR5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - HTICLR5"]
    #[inline(always)]
    pub fn hticlr5(&self) -> HTICLR5_R {
        HTICLR5_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TCICLR5"]
    #[inline(always)]
    pub fn tciclr5(&self) -> TCICLR5_R {
        TCICLR5_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TEICLR5"]
    #[inline(always)]
    pub fn teiclr5(&self) -> TEICLR5_R {
        TEICLR5_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEICLR0"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr0(&mut self) -> GEICLR0_W<0> {
        GEICLR0_W::new(self)
    }
    #[doc = "Bit 1 - BEICLR0"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr0(&mut self) -> BEICLR0_W<1> {
        BEICLR0_W::new(self)
    }
    #[doc = "Bit 2 - HTICLR0"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr0(&mut self) -> HTICLR0_W<2> {
        HTICLR0_W::new(self)
    }
    #[doc = "Bit 3 - TCICLR0"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr0(&mut self) -> TCICLR0_W<3> {
        TCICLR0_W::new(self)
    }
    #[doc = "Bit 4 - TEICLR0"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr0(&mut self) -> TEICLR0_W<4> {
        TEICLR0_W::new(self)
    }
    #[doc = "Bit 5 - GEICLR1"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr1(&mut self) -> GEICLR1_W<5> {
        GEICLR1_W::new(self)
    }
    #[doc = "Bit 6 - BEICLR1"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr1(&mut self) -> BEICLR1_W<6> {
        BEICLR1_W::new(self)
    }
    #[doc = "Bit 7 - HTICLR1"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr1(&mut self) -> HTICLR1_W<7> {
        HTICLR1_W::new(self)
    }
    #[doc = "Bit 8 - TCICLR1"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr1(&mut self) -> TCICLR1_W<8> {
        TCICLR1_W::new(self)
    }
    #[doc = "Bit 9 - TEICLR1"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr1(&mut self) -> TEICLR1_W<9> {
        TEICLR1_W::new(self)
    }
    #[doc = "Bit 10 - GEICLR2"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr2(&mut self) -> GEICLR2_W<10> {
        GEICLR2_W::new(self)
    }
    #[doc = "Bit 11 - BEICLR2"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr2(&mut self) -> BEICLR2_W<11> {
        BEICLR2_W::new(self)
    }
    #[doc = "Bit 12 - HTICLR2"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr2(&mut self) -> HTICLR2_W<12> {
        HTICLR2_W::new(self)
    }
    #[doc = "Bit 13 - TCICLR2"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr2(&mut self) -> TCICLR2_W<13> {
        TCICLR2_W::new(self)
    }
    #[doc = "Bit 14 - TEICLR2"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr2(&mut self) -> TEICLR2_W<14> {
        TEICLR2_W::new(self)
    }
    #[doc = "Bit 15 - GEICLR3"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr3(&mut self) -> GEICLR3_W<15> {
        GEICLR3_W::new(self)
    }
    #[doc = "Bit 16 - BEICLR3"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr3(&mut self) -> BEICLR3_W<16> {
        BEICLR3_W::new(self)
    }
    #[doc = "Bit 17 - HTICLR3"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr3(&mut self) -> HTICLR3_W<17> {
        HTICLR3_W::new(self)
    }
    #[doc = "Bit 18 - TCICLR3"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr3(&mut self) -> TCICLR3_W<18> {
        TCICLR3_W::new(self)
    }
    #[doc = "Bit 19 - TEICLR3"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr3(&mut self) -> TEICLR3_W<19> {
        TEICLR3_W::new(self)
    }
    #[doc = "Bit 20 - GEICLR4"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr4(&mut self) -> GEICLR4_W<20> {
        GEICLR4_W::new(self)
    }
    #[doc = "Bit 21 - BEICLR4"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr4(&mut self) -> BEICLR4_W<21> {
        BEICLR4_W::new(self)
    }
    #[doc = "Bit 22 - HTICLR4"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr4(&mut self) -> HTICLR4_W<22> {
        HTICLR4_W::new(self)
    }
    #[doc = "Bit 23 - TCICLR4"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr4(&mut self) -> TCICLR4_W<23> {
        TCICLR4_W::new(self)
    }
    #[doc = "Bit 24 - TEICLR4"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr4(&mut self) -> TEICLR4_W<24> {
        TEICLR4_W::new(self)
    }
    #[doc = "Bit 25 - GEICLR5"]
    #[inline(always)]
    #[must_use]
    pub fn geiclr5(&mut self) -> GEICLR5_W<25> {
        GEICLR5_W::new(self)
    }
    #[doc = "Bit 26 - BEICLR5"]
    #[inline(always)]
    #[must_use]
    pub fn beiclr5(&mut self) -> BEICLR5_W<26> {
        BEICLR5_W::new(self)
    }
    #[doc = "Bit 27 - HTICLR5"]
    #[inline(always)]
    #[must_use]
    pub fn hticlr5(&mut self) -> HTICLR5_W<27> {
        HTICLR5_W::new(self)
    }
    #[doc = "Bit 28 - TCICLR5"]
    #[inline(always)]
    #[must_use]
    pub fn tciclr5(&mut self) -> TCICLR5_W<28> {
        TCICLR5_W::new(self)
    }
    #[doc = "Bit 29 - TEICLR5"]
    #[inline(always)]
    #[must_use]
    pub fn teiclr5(&mut self) -> TEICLR5_W<29> {
        TEICLR5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iscr](index.html) module"]
pub struct ISCR_SPEC;
impl crate::RegisterSpec for ISCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iscr::R](R) reader structure"]
impl crate::Readable for ISCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iscr::W](W) writer structure"]
impl crate::Writable for ISCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISCR to value 0"]
impl crate::Resettable for ISCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

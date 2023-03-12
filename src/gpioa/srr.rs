#[doc = "Register `SRR` reader"]
pub struct R(crate::R<SRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRR` writer"]
pub struct W(crate::W<SRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRR_SPEC>;
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
impl From<crate::W<SRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET0` reader - SET0"]
pub type SET0_R = crate::BitReader<bool>;
#[doc = "Field `SET0` writer - SET0"]
pub type SET0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET1` reader - SET1"]
pub type SET1_R = crate::BitReader<bool>;
#[doc = "Field `SET1` writer - SET1"]
pub type SET1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET2` reader - SET2"]
pub type SET2_R = crate::BitReader<bool>;
#[doc = "Field `SET2` writer - SET2"]
pub type SET2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET3` reader - SET3"]
pub type SET3_R = crate::BitReader<bool>;
#[doc = "Field `SET3` writer - SET3"]
pub type SET3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET4` reader - SET4"]
pub type SET4_R = crate::BitReader<bool>;
#[doc = "Field `SET4` writer - SET4"]
pub type SET4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET5` reader - SET5"]
pub type SET5_R = crate::BitReader<bool>;
#[doc = "Field `SET5` writer - SET5"]
pub type SET5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET6` reader - SET6"]
pub type SET6_R = crate::BitReader<bool>;
#[doc = "Field `SET6` writer - SET6"]
pub type SET6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET7` reader - SET7"]
pub type SET7_R = crate::BitReader<bool>;
#[doc = "Field `SET7` writer - SET7"]
pub type SET7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET8` reader - SET8"]
pub type SET8_R = crate::BitReader<bool>;
#[doc = "Field `SET8` writer - SET8"]
pub type SET8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET9` reader - SET9"]
pub type SET9_R = crate::BitReader<bool>;
#[doc = "Field `SET9` writer - SET9"]
pub type SET9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET10` reader - SET10"]
pub type SET10_R = crate::BitReader<bool>;
#[doc = "Field `SET10` writer - SET10"]
pub type SET10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET11` reader - SET11"]
pub type SET11_R = crate::BitReader<bool>;
#[doc = "Field `SET11` writer - SET11"]
pub type SET11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET12` reader - SET12"]
pub type SET12_R = crate::BitReader<bool>;
#[doc = "Field `SET12` writer - SET12"]
pub type SET12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET13` reader - SET13"]
pub type SET13_R = crate::BitReader<bool>;
#[doc = "Field `SET13` writer - SET13"]
pub type SET13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET14` reader - SET14"]
pub type SET14_R = crate::BitReader<bool>;
#[doc = "Field `SET14` writer - SET14"]
pub type SET14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `SET15` reader - SET15"]
pub type SET15_R = crate::BitReader<bool>;
#[doc = "Field `SET15` writer - SET15"]
pub type SET15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST0` reader - RST0"]
pub type RST0_R = crate::BitReader<bool>;
#[doc = "Field `RST0` writer - RST0"]
pub type RST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST1` reader - RST1"]
pub type RST1_R = crate::BitReader<bool>;
#[doc = "Field `RST1` writer - RST1"]
pub type RST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST2` reader - RST2"]
pub type RST2_R = crate::BitReader<bool>;
#[doc = "Field `RST2` writer - RST2"]
pub type RST2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST3` reader - RST3"]
pub type RST3_R = crate::BitReader<bool>;
#[doc = "Field `RST3` writer - RST3"]
pub type RST3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST4` reader - RST4"]
pub type RST4_R = crate::BitReader<bool>;
#[doc = "Field `RST4` writer - RST4"]
pub type RST4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST5` reader - RST5"]
pub type RST5_R = crate::BitReader<bool>;
#[doc = "Field `RST5` writer - RST5"]
pub type RST5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST6` reader - RST6"]
pub type RST6_R = crate::BitReader<bool>;
#[doc = "Field `RST6` writer - RST6"]
pub type RST6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST7` reader - RST7"]
pub type RST7_R = crate::BitReader<bool>;
#[doc = "Field `RST7` writer - RST7"]
pub type RST7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST8` reader - RST8"]
pub type RST8_R = crate::BitReader<bool>;
#[doc = "Field `RST8` writer - RST8"]
pub type RST8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST9` reader - RST9"]
pub type RST9_R = crate::BitReader<bool>;
#[doc = "Field `RST9` writer - RST9"]
pub type RST9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST10` reader - RST10"]
pub type RST10_R = crate::BitReader<bool>;
#[doc = "Field `RST10` writer - RST10"]
pub type RST10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST11` reader - RST11"]
pub type RST11_R = crate::BitReader<bool>;
#[doc = "Field `RST11` writer - RST11"]
pub type RST11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST12` reader - RST12"]
pub type RST12_R = crate::BitReader<bool>;
#[doc = "Field `RST12` writer - RST12"]
pub type RST12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST13` reader - RST13"]
pub type RST13_R = crate::BitReader<bool>;
#[doc = "Field `RST13` writer - RST13"]
pub type RST13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST14` reader - RST14"]
pub type RST14_R = crate::BitReader<bool>;
#[doc = "Field `RST14` writer - RST14"]
pub type RST14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
#[doc = "Field `RST15` reader - RST15"]
pub type RST15_R = crate::BitReader<bool>;
#[doc = "Field `RST15` writer - RST15"]
pub type RST15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SET0"]
    #[inline(always)]
    pub fn set0(&self) -> SET0_R {
        SET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SET1"]
    #[inline(always)]
    pub fn set1(&self) -> SET1_R {
        SET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SET2"]
    #[inline(always)]
    pub fn set2(&self) -> SET2_R {
        SET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SET3"]
    #[inline(always)]
    pub fn set3(&self) -> SET3_R {
        SET3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SET4"]
    #[inline(always)]
    pub fn set4(&self) -> SET4_R {
        SET4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SET5"]
    #[inline(always)]
    pub fn set5(&self) -> SET5_R {
        SET5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SET6"]
    #[inline(always)]
    pub fn set6(&self) -> SET6_R {
        SET6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SET7"]
    #[inline(always)]
    pub fn set7(&self) -> SET7_R {
        SET7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SET8"]
    #[inline(always)]
    pub fn set8(&self) -> SET8_R {
        SET8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SET9"]
    #[inline(always)]
    pub fn set9(&self) -> SET9_R {
        SET9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SET10"]
    #[inline(always)]
    pub fn set10(&self) -> SET10_R {
        SET10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SET11"]
    #[inline(always)]
    pub fn set11(&self) -> SET11_R {
        SET11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SET12"]
    #[inline(always)]
    pub fn set12(&self) -> SET12_R {
        SET12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SET13"]
    #[inline(always)]
    pub fn set13(&self) -> SET13_R {
        SET13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SET14"]
    #[inline(always)]
    pub fn set14(&self) -> SET14_R {
        SET14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SET15"]
    #[inline(always)]
    pub fn set15(&self) -> SET15_R {
        SET15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RST0"]
    #[inline(always)]
    pub fn rst0(&self) -> RST0_R {
        RST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RST1"]
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RST2"]
    #[inline(always)]
    pub fn rst2(&self) -> RST2_R {
        RST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RST3"]
    #[inline(always)]
    pub fn rst3(&self) -> RST3_R {
        RST3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RST4"]
    #[inline(always)]
    pub fn rst4(&self) -> RST4_R {
        RST4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RST5"]
    #[inline(always)]
    pub fn rst5(&self) -> RST5_R {
        RST5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - RST6"]
    #[inline(always)]
    pub fn rst6(&self) -> RST6_R {
        RST6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RST7"]
    #[inline(always)]
    pub fn rst7(&self) -> RST7_R {
        RST7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RST8"]
    #[inline(always)]
    pub fn rst8(&self) -> RST8_R {
        RST8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RST9"]
    #[inline(always)]
    pub fn rst9(&self) -> RST9_R {
        RST9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RST10"]
    #[inline(always)]
    pub fn rst10(&self) -> RST10_R {
        RST10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RST11"]
    #[inline(always)]
    pub fn rst11(&self) -> RST11_R {
        RST11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RST12"]
    #[inline(always)]
    pub fn rst12(&self) -> RST12_R {
        RST12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RST13"]
    #[inline(always)]
    pub fn rst13(&self) -> RST13_R {
        RST13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RST14"]
    #[inline(always)]
    pub fn rst14(&self) -> RST14_R {
        RST14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RST15"]
    #[inline(always)]
    pub fn rst15(&self) -> RST15_R {
        RST15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SET0"]
    #[inline(always)]
    #[must_use]
    pub fn set0(&mut self) -> SET0_W<0> {
        SET0_W::new(self)
    }
    #[doc = "Bit 1 - SET1"]
    #[inline(always)]
    #[must_use]
    pub fn set1(&mut self) -> SET1_W<1> {
        SET1_W::new(self)
    }
    #[doc = "Bit 2 - SET2"]
    #[inline(always)]
    #[must_use]
    pub fn set2(&mut self) -> SET2_W<2> {
        SET2_W::new(self)
    }
    #[doc = "Bit 3 - SET3"]
    #[inline(always)]
    #[must_use]
    pub fn set3(&mut self) -> SET3_W<3> {
        SET3_W::new(self)
    }
    #[doc = "Bit 4 - SET4"]
    #[inline(always)]
    #[must_use]
    pub fn set4(&mut self) -> SET4_W<4> {
        SET4_W::new(self)
    }
    #[doc = "Bit 5 - SET5"]
    #[inline(always)]
    #[must_use]
    pub fn set5(&mut self) -> SET5_W<5> {
        SET5_W::new(self)
    }
    #[doc = "Bit 6 - SET6"]
    #[inline(always)]
    #[must_use]
    pub fn set6(&mut self) -> SET6_W<6> {
        SET6_W::new(self)
    }
    #[doc = "Bit 7 - SET7"]
    #[inline(always)]
    #[must_use]
    pub fn set7(&mut self) -> SET7_W<7> {
        SET7_W::new(self)
    }
    #[doc = "Bit 8 - SET8"]
    #[inline(always)]
    #[must_use]
    pub fn set8(&mut self) -> SET8_W<8> {
        SET8_W::new(self)
    }
    #[doc = "Bit 9 - SET9"]
    #[inline(always)]
    #[must_use]
    pub fn set9(&mut self) -> SET9_W<9> {
        SET9_W::new(self)
    }
    #[doc = "Bit 10 - SET10"]
    #[inline(always)]
    #[must_use]
    pub fn set10(&mut self) -> SET10_W<10> {
        SET10_W::new(self)
    }
    #[doc = "Bit 11 - SET11"]
    #[inline(always)]
    #[must_use]
    pub fn set11(&mut self) -> SET11_W<11> {
        SET11_W::new(self)
    }
    #[doc = "Bit 12 - SET12"]
    #[inline(always)]
    #[must_use]
    pub fn set12(&mut self) -> SET12_W<12> {
        SET12_W::new(self)
    }
    #[doc = "Bit 13 - SET13"]
    #[inline(always)]
    #[must_use]
    pub fn set13(&mut self) -> SET13_W<13> {
        SET13_W::new(self)
    }
    #[doc = "Bit 14 - SET14"]
    #[inline(always)]
    #[must_use]
    pub fn set14(&mut self) -> SET14_W<14> {
        SET14_W::new(self)
    }
    #[doc = "Bit 15 - SET15"]
    #[inline(always)]
    #[must_use]
    pub fn set15(&mut self) -> SET15_W<15> {
        SET15_W::new(self)
    }
    #[doc = "Bit 16 - RST0"]
    #[inline(always)]
    #[must_use]
    pub fn rst0(&mut self) -> RST0_W<16> {
        RST0_W::new(self)
    }
    #[doc = "Bit 17 - RST1"]
    #[inline(always)]
    #[must_use]
    pub fn rst1(&mut self) -> RST1_W<17> {
        RST1_W::new(self)
    }
    #[doc = "Bit 18 - RST2"]
    #[inline(always)]
    #[must_use]
    pub fn rst2(&mut self) -> RST2_W<18> {
        RST2_W::new(self)
    }
    #[doc = "Bit 19 - RST3"]
    #[inline(always)]
    #[must_use]
    pub fn rst3(&mut self) -> RST3_W<19> {
        RST3_W::new(self)
    }
    #[doc = "Bit 20 - RST4"]
    #[inline(always)]
    #[must_use]
    pub fn rst4(&mut self) -> RST4_W<20> {
        RST4_W::new(self)
    }
    #[doc = "Bit 21 - RST5"]
    #[inline(always)]
    #[must_use]
    pub fn rst5(&mut self) -> RST5_W<21> {
        RST5_W::new(self)
    }
    #[doc = "Bit 22 - RST6"]
    #[inline(always)]
    #[must_use]
    pub fn rst6(&mut self) -> RST6_W<22> {
        RST6_W::new(self)
    }
    #[doc = "Bit 23 - RST7"]
    #[inline(always)]
    #[must_use]
    pub fn rst7(&mut self) -> RST7_W<23> {
        RST7_W::new(self)
    }
    #[doc = "Bit 24 - RST8"]
    #[inline(always)]
    #[must_use]
    pub fn rst8(&mut self) -> RST8_W<24> {
        RST8_W::new(self)
    }
    #[doc = "Bit 25 - RST9"]
    #[inline(always)]
    #[must_use]
    pub fn rst9(&mut self) -> RST9_W<25> {
        RST9_W::new(self)
    }
    #[doc = "Bit 26 - RST10"]
    #[inline(always)]
    #[must_use]
    pub fn rst10(&mut self) -> RST10_W<26> {
        RST10_W::new(self)
    }
    #[doc = "Bit 27 - RST11"]
    #[inline(always)]
    #[must_use]
    pub fn rst11(&mut self) -> RST11_W<27> {
        RST11_W::new(self)
    }
    #[doc = "Bit 28 - RST12"]
    #[inline(always)]
    #[must_use]
    pub fn rst12(&mut self) -> RST12_W<28> {
        RST12_W::new(self)
    }
    #[doc = "Bit 29 - RST13"]
    #[inline(always)]
    #[must_use]
    pub fn rst13(&mut self) -> RST13_W<29> {
        RST13_W::new(self)
    }
    #[doc = "Bit 30 - RST14"]
    #[inline(always)]
    #[must_use]
    pub fn rst14(&mut self) -> RST14_W<30> {
        RST14_W::new(self)
    }
    #[doc = "Bit 31 - RST15"]
    #[inline(always)]
    #[must_use]
    pub fn rst15(&mut self) -> RST15_W<31> {
        RST15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srr](index.html) module"]
pub struct SRR_SPEC;
impl crate::RegisterSpec for SRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srr::R](R) reader structure"]
impl crate::Readable for SRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srr::W](W) writer structure"]
impl crate::Writable for SRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRR to value 0"]
impl crate::Resettable for SRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `APBPRSTR0` reader"]
pub struct R(crate::R<APBPRSTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBPRSTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBPRSTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBPRSTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBPRSTR0` writer"]
pub struct W(crate::W<APBPRSTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBPRSTR0_SPEC>;
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
impl From<crate::W<APBPRSTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBPRSTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C0RST` reader - I2C0RST"]
pub type I2C0RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C0RST` writer - I2C0RST"]
pub type I2C0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `I2C1RST` reader - I2C1RST"]
pub type I2C1RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C1RST` writer - I2C1RST"]
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `SPI0RST` reader - SPI0RST"]
pub type SPI0RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI0RST` writer - SPI0RST"]
pub type SPI0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `SPI1RST` reader - SPI1RST"]
pub type SPI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI1RST` writer - SPI1RST"]
pub type SPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `USR0RST` reader - USR0RST"]
pub type USR0RST_R = crate::BitReader<bool>;
#[doc = "Field `USR0RST` writer - USR0RST"]
pub type USR0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `USR1RST` reader - USR1RST"]
pub type USR1RST_R = crate::BitReader<bool>;
#[doc = "Field `USR1RST` writer - USR1RST"]
pub type USR1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `UR0RST` reader - UR0RST"]
pub type UR0RST_R = crate::BitReader<bool>;
#[doc = "Field `UR0RST` writer - UR0RST"]
pub type UR0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `UR1RST` reader - UR1RST"]
pub type UR1RST_R = crate::BitReader<bool>;
#[doc = "Field `UR1RST` writer - UR1RST"]
pub type UR1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `AFIORST` reader - AFIORST"]
pub type AFIORST_R = crate::BitReader<bool>;
#[doc = "Field `AFIORST` writer - AFIORST"]
pub type AFIORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `EXTIRST` reader - EXTIRST"]
pub type EXTIRST_R = crate::BitReader<bool>;
#[doc = "Field `EXTIRST` writer - EXTIRST"]
pub type EXTIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `SCI0RST` reader - SCI0RST"]
pub type SCI0RST_R = crate::BitReader<bool>;
#[doc = "Field `SCI0RST` writer - SCI0RST"]
pub type SCI0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `I2SRST` reader - I2SRST"]
pub type I2SRST_R = crate::BitReader<bool>;
#[doc = "Field `I2SRST` writer - I2SRST"]
pub type I2SRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
#[doc = "Field `SCI1RST` reader - SCI1RST"]
pub type SCI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SCI1RST` writer - SCI1RST"]
pub type SCI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I2C0RST"]
    #[inline(always)]
    pub fn i2c0rst(&self) -> I2C0RST_R {
        I2C0RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI0RST"]
    #[inline(always)]
    pub fn spi0rst(&self) -> SPI0RST_R {
        SPI0RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USR0RST"]
    #[inline(always)]
    pub fn usr0rst(&self) -> USR0RST_R {
        USR0RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USR1RST"]
    #[inline(always)]
    pub fn usr1rst(&self) -> USR1RST_R {
        USR1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UR0RST"]
    #[inline(always)]
    pub fn ur0rst(&self) -> UR0RST_R {
        UR0RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UR1RST"]
    #[inline(always)]
    pub fn ur1rst(&self) -> UR1RST_R {
        UR1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - AFIORST"]
    #[inline(always)]
    pub fn afiorst(&self) -> AFIORST_R {
        AFIORST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTIRST"]
    #[inline(always)]
    pub fn extirst(&self) -> EXTIRST_R {
        EXTIRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - SCI0RST"]
    #[inline(always)]
    pub fn sci0rst(&self) -> SCI0RST_R {
        SCI0RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2SRST"]
    #[inline(always)]
    pub fn i2srst(&self) -> I2SRST_R {
        I2SRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - SCI1RST"]
    #[inline(always)]
    pub fn sci1rst(&self) -> SCI1RST_R {
        SCI1RST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0rst(&mut self) -> I2C0RST_W<0> {
        I2C0RST_W::new(self)
    }
    #[doc = "Bit 1 - I2C1RST"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<1> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 4 - SPI0RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi0rst(&mut self) -> SPI0RST_W<4> {
        SPI0RST_W::new(self)
    }
    #[doc = "Bit 5 - SPI1RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<5> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 8 - USR0RST"]
    #[inline(always)]
    #[must_use]
    pub fn usr0rst(&mut self) -> USR0RST_W<8> {
        USR0RST_W::new(self)
    }
    #[doc = "Bit 9 - USR1RST"]
    #[inline(always)]
    #[must_use]
    pub fn usr1rst(&mut self) -> USR1RST_W<9> {
        USR1RST_W::new(self)
    }
    #[doc = "Bit 10 - UR0RST"]
    #[inline(always)]
    #[must_use]
    pub fn ur0rst(&mut self) -> UR0RST_W<10> {
        UR0RST_W::new(self)
    }
    #[doc = "Bit 11 - UR1RST"]
    #[inline(always)]
    #[must_use]
    pub fn ur1rst(&mut self) -> UR1RST_W<11> {
        UR1RST_W::new(self)
    }
    #[doc = "Bit 14 - AFIORST"]
    #[inline(always)]
    #[must_use]
    pub fn afiorst(&mut self) -> AFIORST_W<14> {
        AFIORST_W::new(self)
    }
    #[doc = "Bit 15 - EXTIRST"]
    #[inline(always)]
    #[must_use]
    pub fn extirst(&mut self) -> EXTIRST_W<15> {
        EXTIRST_W::new(self)
    }
    #[doc = "Bit 24 - SCI0RST"]
    #[inline(always)]
    #[must_use]
    pub fn sci0rst(&mut self) -> SCI0RST_W<24> {
        SCI0RST_W::new(self)
    }
    #[doc = "Bit 25 - I2SRST"]
    #[inline(always)]
    #[must_use]
    pub fn i2srst(&mut self) -> I2SRST_W<25> {
        I2SRST_W::new(self)
    }
    #[doc = "Bit 27 - SCI1RST"]
    #[inline(always)]
    #[must_use]
    pub fn sci1rst(&mut self) -> SCI1RST_W<27> {
        SCI1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBPRSTR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbprstr0](index.html) module"]
pub struct APBPRSTR0_SPEC;
impl crate::RegisterSpec for APBPRSTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbprstr0::R](R) reader structure"]
impl crate::Readable for APBPRSTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbprstr0::W](W) writer structure"]
impl crate::Writable for APBPRSTR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBPRSTR0 to value 0"]
impl crate::Resettable for APBPRSTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

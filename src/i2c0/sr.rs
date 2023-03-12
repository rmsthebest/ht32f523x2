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
#[doc = "Field `STA` reader - STA"]
pub type STA_R = crate::BitReader<bool>;
#[doc = "Field `STA` writer - STA"]
pub type STA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `STO` reader - STO"]
pub type STO_R = crate::BitReader<bool>;
#[doc = "Field `STO` writer - STO"]
pub type STO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ADRS` reader - ADRS"]
pub type ADRS_R = crate::BitReader<bool>;
#[doc = "Field `ADRS` writer - ADRS"]
pub type ADRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `GCS` reader - GCS"]
pub type GCS_R = crate::BitReader<bool>;
#[doc = "Field `GCS` writer - GCS"]
pub type GCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ARBLOS` reader - ARBLOS"]
pub type ARBLOS_R = crate::BitReader<bool>;
#[doc = "Field `ARBLOS` writer - ARBLOS"]
pub type ARBLOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `RXNACK` reader - RXNACK"]
pub type RXNACK_R = crate::BitReader<bool>;
#[doc = "Field `RXNACK` writer - RXNACK"]
pub type RXNACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `BUSERR` reader - BUSERR"]
pub type BUSERR_R = crate::BitReader<bool>;
#[doc = "Field `BUSERR` writer - BUSERR"]
pub type BUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TOUTF` reader - TOUTF"]
pub type TOUTF_R = crate::BitReader<bool>;
#[doc = "Field `TOUTF` writer - TOUTF"]
pub type TOUTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `RXDNE` reader - RXDNE"]
pub type RXDNE_R = crate::BitReader<bool>;
#[doc = "Field `RXDNE` writer - RXDNE"]
pub type RXDNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TXDE` reader - TXDE"]
pub type TXDE_R = crate::BitReader<bool>;
#[doc = "Field `TXDE` writer - TXDE"]
pub type TXDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `RXBF` reader - RXBF"]
pub type RXBF_R = crate::BitReader<bool>;
#[doc = "Field `RXBF` writer - RXBF"]
pub type RXBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `BUSBUSY` reader - BUSBUSY"]
pub type BUSBUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSBUSY` writer - BUSBUSY"]
pub type BUSBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `MASTER` reader - MASTER"]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `MASTER` writer - MASTER"]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TXNRX` reader - TXNRX"]
pub type TXNRX_R = crate::BitReader<bool>;
#[doc = "Field `TXNRX` writer - TXNRX"]
pub type TXNRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - STA"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STO"]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADRS"]
    #[inline(always)]
    pub fn adrs(&self) -> ADRS_R {
        ADRS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GCS"]
    #[inline(always)]
    pub fn gcs(&self) -> GCS_R {
        GCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - ARBLOS"]
    #[inline(always)]
    pub fn arblos(&self) -> ARBLOS_R {
        ARBLOS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXNACK"]
    #[inline(always)]
    pub fn rxnack(&self) -> RXNACK_R {
        RXNACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BUSERR"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TOUTF"]
    #[inline(always)]
    pub fn toutf(&self) -> TOUTF_R {
        TOUTF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - RXDNE"]
    #[inline(always)]
    pub fn rxdne(&self) -> RXDNE_R {
        RXDNE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXDE"]
    #[inline(always)]
    pub fn txde(&self) -> TXDE_R {
        TXDE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RXBF"]
    #[inline(always)]
    pub fn rxbf(&self) -> RXBF_R {
        RXBF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BUSBUSY"]
    #[inline(always)]
    pub fn busbusy(&self) -> BUSBUSY_R {
        BUSBUSY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MASTER"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TXNRX"]
    #[inline(always)]
    pub fn txnrx(&self) -> TXNRX_R {
        TXNRX_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STA"]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> STA_W<0> {
        STA_W::new(self)
    }
    #[doc = "Bit 1 - STO"]
    #[inline(always)]
    #[must_use]
    pub fn sto(&mut self) -> STO_W<1> {
        STO_W::new(self)
    }
    #[doc = "Bit 2 - ADRS"]
    #[inline(always)]
    #[must_use]
    pub fn adrs(&mut self) -> ADRS_W<2> {
        ADRS_W::new(self)
    }
    #[doc = "Bit 3 - GCS"]
    #[inline(always)]
    #[must_use]
    pub fn gcs(&mut self) -> GCS_W<3> {
        GCS_W::new(self)
    }
    #[doc = "Bit 8 - ARBLOS"]
    #[inline(always)]
    #[must_use]
    pub fn arblos(&mut self) -> ARBLOS_W<8> {
        ARBLOS_W::new(self)
    }
    #[doc = "Bit 9 - RXNACK"]
    #[inline(always)]
    #[must_use]
    pub fn rxnack(&mut self) -> RXNACK_W<9> {
        RXNACK_W::new(self)
    }
    #[doc = "Bit 10 - BUSERR"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<10> {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 11 - TOUTF"]
    #[inline(always)]
    #[must_use]
    pub fn toutf(&mut self) -> TOUTF_W<11> {
        TOUTF_W::new(self)
    }
    #[doc = "Bit 16 - RXDNE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdne(&mut self) -> RXDNE_W<16> {
        RXDNE_W::new(self)
    }
    #[doc = "Bit 17 - TXDE"]
    #[inline(always)]
    #[must_use]
    pub fn txde(&mut self) -> TXDE_W<17> {
        TXDE_W::new(self)
    }
    #[doc = "Bit 18 - RXBF"]
    #[inline(always)]
    #[must_use]
    pub fn rxbf(&mut self) -> RXBF_W<18> {
        RXBF_W::new(self)
    }
    #[doc = "Bit 19 - BUSBUSY"]
    #[inline(always)]
    #[must_use]
    pub fn busbusy(&mut self) -> BUSBUSY_W<19> {
        BUSBUSY_W::new(self)
    }
    #[doc = "Bit 20 - MASTER"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<20> {
        MASTER_W::new(self)
    }
    #[doc = "Bit 21 - TXNRX"]
    #[inline(always)]
    #[must_use]
    pub fn txnrx(&mut self) -> TXNRX_W<21> {
        TXNRX_W::new(self)
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

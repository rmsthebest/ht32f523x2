#[doc = "Register `USART_RS485CR` reader"]
pub type R = crate::R<UsartRs485crSpec>;
#[doc = "Register `USART_RS485CR` writer"]
pub type W = crate::W<UsartRs485crSpec>;
#[doc = "Field `TXENP` reader - TXENP"]
pub type TxenpR = crate::BitReader;
#[doc = "Field `TXENP` writer - TXENP"]
pub type TxenpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSNMM` reader - RSNMM"]
pub type RsnmmR = crate::BitReader;
#[doc = "Field `RSNMM` writer - RSNMM"]
pub type RsnmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSAAD` reader - RSAAD"]
pub type RsaadR = crate::BitReader;
#[doc = "Field `RSAAD` writer - RSAAD"]
pub type RsaadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDMATCH` reader - ADDMATCH"]
pub type AddmatchR = crate::FieldReader;
#[doc = "Field `ADDMATCH` writer - ADDMATCH"]
pub type AddmatchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - TXENP"]
    #[inline(always)]
    pub fn txenp(&self) -> TxenpR {
        TxenpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSNMM"]
    #[inline(always)]
    pub fn rsnmm(&self) -> RsnmmR {
        RsnmmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RSAAD"]
    #[inline(always)]
    pub fn rsaad(&self) -> RsaadR {
        RsaadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - ADDMATCH"]
    #[inline(always)]
    pub fn addmatch(&self) -> AddmatchR {
        AddmatchR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXENP"]
    #[inline(always)]
    pub fn txenp(&mut self) -> TxenpW<'_, UsartRs485crSpec> {
        TxenpW::new(self, 0)
    }
    #[doc = "Bit 1 - RSNMM"]
    #[inline(always)]
    pub fn rsnmm(&mut self) -> RsnmmW<'_, UsartRs485crSpec> {
        RsnmmW::new(self, 1)
    }
    #[doc = "Bit 2 - RSAAD"]
    #[inline(always)]
    pub fn rsaad(&mut self) -> RsaadW<'_, UsartRs485crSpec> {
        RsaadW::new(self, 2)
    }
    #[doc = "Bits 8:15 - ADDMATCH"]
    #[inline(always)]
    pub fn addmatch(&mut self) -> AddmatchW<'_, UsartRs485crSpec> {
        AddmatchW::new(self, 8)
    }
}
#[doc = "USART_RS485CR\n\nYou can [`read`](crate::Reg::read) this register and get [`usart_rs485cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_rs485cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartRs485crSpec;
impl crate::RegisterSpec for UsartRs485crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_rs485cr::R`](R) reader structure"]
impl crate::Readable for UsartRs485crSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_rs485cr::W`](W) writer structure"]
impl crate::Writable for UsartRs485crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USART_RS485CR to value 0"]
impl crate::Resettable for UsartRs485crSpec {}

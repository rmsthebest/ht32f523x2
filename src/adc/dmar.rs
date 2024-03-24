#[doc = "Register `DMAR` reader"]
pub type R = crate::R<DmarSpec>;
#[doc = "Register `DMAR` writer"]
pub type W = crate::W<DmarSpec>;
#[doc = "Field `ADDMAS` reader - ADDMAS"]
pub type AddmasR = crate::BitReader;
#[doc = "Field `ADDMAS` writer - ADDMAS"]
pub type AddmasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDMAG` reader - ADDMAG"]
pub type AddmagR = crate::BitReader;
#[doc = "Field `ADDMAG` writer - ADDMAG"]
pub type AddmagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDMAC` reader - ADDMAC"]
pub type AddmacR = crate::BitReader;
#[doc = "Field `ADDMAC` writer - ADDMAC"]
pub type AddmacW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADDMAS"]
    #[inline(always)]
    pub fn addmas(&self) -> AddmasR {
        AddmasR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDMAG"]
    #[inline(always)]
    pub fn addmag(&self) -> AddmagR {
        AddmagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADDMAC"]
    #[inline(always)]
    pub fn addmac(&self) -> AddmacR {
        AddmacR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADDMAS"]
    #[inline(always)]
    #[must_use]
    pub fn addmas(&mut self) -> AddmasW<DmarSpec> {
        AddmasW::new(self, 0)
    }
    #[doc = "Bit 1 - ADDMAG"]
    #[inline(always)]
    #[must_use]
    pub fn addmag(&mut self) -> AddmagW<DmarSpec> {
        AddmagW::new(self, 1)
    }
    #[doc = "Bit 2 - ADDMAC"]
    #[inline(always)]
    #[must_use]
    pub fn addmac(&mut self) -> AddmacW<DmarSpec> {
        AddmacW::new(self, 2)
    }
}
#[doc = "DMAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmarSpec;
impl crate::RegisterSpec for DmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmar::R`](R) reader structure"]
impl crate::Readable for DmarSpec {}
#[doc = "`write(|w| ..)` method takes [`dmar::W`](W) writer structure"]
impl crate::Writable for DmarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAR to value 0"]
impl crate::Resettable for DmarSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Reader of register OIER"]
pub type R = crate::R<u32, super::OIER>;
#[doc = "Writer for register OIER"]
pub type W = crate::W<u32, super::OIER>;
#[doc = "Register OIER `reset()`'s with value 0"]
impl crate::ResetValue for super::OIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ORFIEN`"]
pub type ORFIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ORFIEN`"]
pub struct ORFIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ORFIEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ITADIEN`"]
pub type ITADIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITADIEN`"]
pub struct ITADIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ITADIEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `OBEIEN`"]
pub type OBEIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OBEIEN`"]
pub struct OBEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OBEIEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `IOCMIEN`"]
pub type IOCMIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOCMIEN`"]
pub struct IOCMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCMIEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `OREIEN`"]
pub type OREIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OREIEN`"]
pub struct OREIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OREIEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ORFIEN"]
    #[inline(always)]
    pub fn orfien(&self) -> ORFIEN_R {
        ORFIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ITADIEN"]
    #[inline(always)]
    pub fn itadien(&self) -> ITADIEN_R {
        ITADIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OBEIEN"]
    #[inline(always)]
    pub fn obeien(&self) -> OBEIEN_R {
        OBEIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IOCMIEN"]
    #[inline(always)]
    pub fn iocmien(&self) -> IOCMIEN_R {
        IOCMIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OREIEN"]
    #[inline(always)]
    pub fn oreien(&self) -> OREIEN_R {
        OREIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ORFIEN"]
    #[inline(always)]
    pub fn orfien(&mut self) -> ORFIEN_W {
        ORFIEN_W { w: self }
    }
    #[doc = "Bit 1 - ITADIEN"]
    #[inline(always)]
    pub fn itadien(&mut self) -> ITADIEN_W {
        ITADIEN_W { w: self }
    }
    #[doc = "Bit 2 - OBEIEN"]
    #[inline(always)]
    pub fn obeien(&mut self) -> OBEIEN_W {
        OBEIEN_W { w: self }
    }
    #[doc = "Bit 3 - IOCMIEN"]
    #[inline(always)]
    pub fn iocmien(&mut self) -> IOCMIEN_W {
        IOCMIEN_W { w: self }
    }
    #[doc = "Bit 4 - OREIEN"]
    #[inline(always)]
    pub fn oreien(&mut self) -> OREIEN_W {
        OREIEN_W { w: self }
    }
}

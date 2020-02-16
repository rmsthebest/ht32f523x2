#[doc = "Reader of register MCUDBGCR"]
pub type R = crate::R<u32, super::MCUDBGCR>;
#[doc = "Writer for register MCUDBGCR"]
pub type W = crate::W<u32, super::MCUDBGCR>;
#[doc = "Register MCUDBGCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCUDBGCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBSLP`"]
pub type DBSLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSLP`"]
pub struct DBSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSLP_W<'a> {
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
#[doc = "Reader of field `DBDSLP1`"]
pub type DBDSLP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBDSLP1`"]
pub struct DBDSLP1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBDSLP1_W<'a> {
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
#[doc = "Reader of field `DBPD`"]
pub type DBPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBPD`"]
pub struct DBPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBPD_W<'a> {
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
#[doc = "Reader of field `DBWDT`"]
pub type DBWDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBWDT`"]
pub struct DBWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBWDT_W<'a> {
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
#[doc = "Reader of field `DBMCTM0`"]
pub type DBMCTM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBMCTM0`"]
pub struct DBMCTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBMCTM0_W<'a> {
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
#[doc = "Reader of field `DBGPTM0`"]
pub type DBGPTM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGPTM0`"]
pub struct DBGPTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGPTM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DBGPTM1`"]
pub type DBGPTM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGPTM1`"]
pub struct DBGPTM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGPTM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DBUSR0`"]
pub type DBUSR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBUSR0`"]
pub struct DBUSR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUSR0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DBUSR1`"]
pub type DBUSR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBUSR1`"]
pub struct DBUSR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUSR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DBSPI0`"]
pub type DBSPI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSPI0`"]
pub struct DBSPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSPI0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DBSPI1`"]
pub type DBSPI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSPI1`"]
pub struct DBSPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSPI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DBI2C0`"]
pub type DBI2C0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBI2C0`"]
pub struct DBI2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI2C0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DBI2C1`"]
pub type DBI2C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBI2C1`"]
pub struct DBI2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI2C1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `DBDSLP2`"]
pub type DBDSLP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBDSLP2`"]
pub struct DBDSLP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBDSLP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DBSCI0`"]
pub type DBSCI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSCI0`"]
pub struct DBSCI0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSCI0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DBBFTM0`"]
pub type DBBFTM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBBFTM0`"]
pub struct DBBFTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBBFTM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DBBFTM1`"]
pub type DBBFTM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBBFTM1`"]
pub struct DBBFTM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBBFTM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DBUR0`"]
pub type DBUR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBUR0`"]
pub struct DBUR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUR0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DBUR1`"]
pub type DBUR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBUR1`"]
pub struct DBUR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DBTRACE`"]
pub type DBTRACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBTRACE`"]
pub struct DBTRACE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBTRACE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DBSCI1`"]
pub type DBSCI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSCI1`"]
pub struct DBSCI1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSCI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DBSCTM0`"]
pub type DBSCTM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSCTM0`"]
pub struct DBSCTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSCTM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `DBSCTM1`"]
pub type DBSCTM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSCTM1`"]
pub struct DBSCTM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSCTM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    pub fn dbslp(&self) -> DBSLP_R {
        DBSLP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    pub fn dbdslp1(&self) -> DBDSLP1_R {
        DBDSLP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DBPD"]
    #[inline(always)]
    pub fn dbpd(&self) -> DBPD_R {
        DBPD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    pub fn dbwdt(&self) -> DBWDT_R {
        DBWDT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DBMCTM0"]
    #[inline(always)]
    pub fn dbmctm0(&self) -> DBMCTM0_R {
        DBMCTM0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DBGPTM0"]
    #[inline(always)]
    pub fn dbgptm0(&self) -> DBGPTM0_R {
        DBGPTM0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DBGPTM1"]
    #[inline(always)]
    pub fn dbgptm1(&self) -> DBGPTM1_R {
        DBGPTM1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DBUSR0"]
    #[inline(always)]
    pub fn dbusr0(&self) -> DBUSR0_R {
        DBUSR0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DBUSR1"]
    #[inline(always)]
    pub fn dbusr1(&self) -> DBUSR1_R {
        DBUSR1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DBSPI0"]
    #[inline(always)]
    pub fn dbspi0(&self) -> DBSPI0_R {
        DBSPI0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DBSPI1"]
    #[inline(always)]
    pub fn dbspi1(&self) -> DBSPI1_R {
        DBSPI1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DBI2C0"]
    #[inline(always)]
    pub fn dbi2c0(&self) -> DBI2C0_R {
        DBI2C0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DBI2C1"]
    #[inline(always)]
    pub fn dbi2c1(&self) -> DBI2C1_R {
        DBI2C1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    pub fn dbdslp2(&self) -> DBDSLP2_R {
        DBDSLP2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DBSCI0"]
    #[inline(always)]
    pub fn dbsci0(&self) -> DBSCI0_R {
        DBSCI0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DBBFTM0"]
    #[inline(always)]
    pub fn dbbftm0(&self) -> DBBFTM0_R {
        DBBFTM0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DBBFTM1"]
    #[inline(always)]
    pub fn dbbftm1(&self) -> DBBFTM1_R {
        DBBFTM1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DBUR0"]
    #[inline(always)]
    pub fn dbur0(&self) -> DBUR0_R {
        DBUR0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DBUR1"]
    #[inline(always)]
    pub fn dbur1(&self) -> DBUR1_R {
        DBUR1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DBTRACE"]
    #[inline(always)]
    pub fn dbtrace(&self) -> DBTRACE_R {
        DBTRACE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DBSCI1"]
    #[inline(always)]
    pub fn dbsci1(&self) -> DBSCI1_R {
        DBSCI1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DBSCTM0"]
    #[inline(always)]
    pub fn dbsctm0(&self) -> DBSCTM0_R {
        DBSCTM0_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DBSCTM1"]
    #[inline(always)]
    pub fn dbsctm1(&self) -> DBSCTM1_R {
        DBSCTM1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    pub fn dbslp(&mut self) -> DBSLP_W {
        DBSLP_W { w: self }
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    pub fn dbdslp1(&mut self) -> DBDSLP1_W {
        DBDSLP1_W { w: self }
    }
    #[doc = "Bit 2 - DBPD"]
    #[inline(always)]
    pub fn dbpd(&mut self) -> DBPD_W {
        DBPD_W { w: self }
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    pub fn dbwdt(&mut self) -> DBWDT_W {
        DBWDT_W { w: self }
    }
    #[doc = "Bit 4 - DBMCTM0"]
    #[inline(always)]
    pub fn dbmctm0(&mut self) -> DBMCTM0_W {
        DBMCTM0_W { w: self }
    }
    #[doc = "Bit 6 - DBGPTM0"]
    #[inline(always)]
    pub fn dbgptm0(&mut self) -> DBGPTM0_W {
        DBGPTM0_W { w: self }
    }
    #[doc = "Bit 7 - DBGPTM1"]
    #[inline(always)]
    pub fn dbgptm1(&mut self) -> DBGPTM1_W {
        DBGPTM1_W { w: self }
    }
    #[doc = "Bit 8 - DBUSR0"]
    #[inline(always)]
    pub fn dbusr0(&mut self) -> DBUSR0_W {
        DBUSR0_W { w: self }
    }
    #[doc = "Bit 9 - DBUSR1"]
    #[inline(always)]
    pub fn dbusr1(&mut self) -> DBUSR1_W {
        DBUSR1_W { w: self }
    }
    #[doc = "Bit 10 - DBSPI0"]
    #[inline(always)]
    pub fn dbspi0(&mut self) -> DBSPI0_W {
        DBSPI0_W { w: self }
    }
    #[doc = "Bit 11 - DBSPI1"]
    #[inline(always)]
    pub fn dbspi1(&mut self) -> DBSPI1_W {
        DBSPI1_W { w: self }
    }
    #[doc = "Bit 12 - DBI2C0"]
    #[inline(always)]
    pub fn dbi2c0(&mut self) -> DBI2C0_W {
        DBI2C0_W { w: self }
    }
    #[doc = "Bit 13 - DBI2C1"]
    #[inline(always)]
    pub fn dbi2c1(&mut self) -> DBI2C1_W {
        DBI2C1_W { w: self }
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    pub fn dbdslp2(&mut self) -> DBDSLP2_W {
        DBDSLP2_W { w: self }
    }
    #[doc = "Bit 15 - DBSCI0"]
    #[inline(always)]
    pub fn dbsci0(&mut self) -> DBSCI0_W {
        DBSCI0_W { w: self }
    }
    #[doc = "Bit 16 - DBBFTM0"]
    #[inline(always)]
    pub fn dbbftm0(&mut self) -> DBBFTM0_W {
        DBBFTM0_W { w: self }
    }
    #[doc = "Bit 17 - DBBFTM1"]
    #[inline(always)]
    pub fn dbbftm1(&mut self) -> DBBFTM1_W {
        DBBFTM1_W { w: self }
    }
    #[doc = "Bit 18 - DBUR0"]
    #[inline(always)]
    pub fn dbur0(&mut self) -> DBUR0_W {
        DBUR0_W { w: self }
    }
    #[doc = "Bit 19 - DBUR1"]
    #[inline(always)]
    pub fn dbur1(&mut self) -> DBUR1_W {
        DBUR1_W { w: self }
    }
    #[doc = "Bit 20 - DBTRACE"]
    #[inline(always)]
    pub fn dbtrace(&mut self) -> DBTRACE_W {
        DBTRACE_W { w: self }
    }
    #[doc = "Bit 21 - DBSCI1"]
    #[inline(always)]
    pub fn dbsci1(&mut self) -> DBSCI1_W {
        DBSCI1_W { w: self }
    }
    #[doc = "Bit 22 - DBSCTM0"]
    #[inline(always)]
    pub fn dbsctm0(&mut self) -> DBSCTM0_W {
        DBSCTM0_W { w: self }
    }
    #[doc = "Bit 23 - DBSCTM1"]
    #[inline(always)]
    pub fn dbsctm1(&mut self) -> DBSCTM1_W {
        DBSCTM1_W { w: self }
    }
}

#[doc = "Register `dfe_ctrl_12` reader"]
pub type R = crate::R<DfeCtrl12Spec>;
#[doc = "Register `dfe_ctrl_12` writer"]
pub type W = crate::W<DfeCtrl12Spec>;
#[doc = "Field `tx_dvga_gain_qdb_gc0` reader - "]
pub type TxDvgaGainQdbGc0R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc0` writer - "]
pub type TxDvgaGainQdbGc0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc1` reader - "]
pub type TxDvgaGainQdbGc1R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc1` writer - "]
pub type TxDvgaGainQdbGc1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc2` reader - "]
pub type TxDvgaGainQdbGc2R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc2` writer - "]
pub type TxDvgaGainQdbGc2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc3` reader - "]
pub type TxDvgaGainQdbGc3R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc3` writer - "]
pub type TxDvgaGainQdbGc3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc0(&self) -> TxDvgaGainQdbGc0R {
        TxDvgaGainQdbGc0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc1(&self) -> TxDvgaGainQdbGc1R {
        TxDvgaGainQdbGc1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc2(&self) -> TxDvgaGainQdbGc2R {
        TxDvgaGainQdbGc2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc3(&self) -> TxDvgaGainQdbGc3R {
        TxDvgaGainQdbGc3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc0(&mut self) -> TxDvgaGainQdbGc0W<'_, DfeCtrl12Spec> {
        TxDvgaGainQdbGc0W::new(self, 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc1(&mut self) -> TxDvgaGainQdbGc1W<'_, DfeCtrl12Spec> {
        TxDvgaGainQdbGc1W::new(self, 8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc2(&mut self) -> TxDvgaGainQdbGc2W<'_, DfeCtrl12Spec> {
        TxDvgaGainQdbGc2W::new(self, 16)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc3(&mut self) -> TxDvgaGainQdbGc3W<'_, DfeCtrl12Spec> {
        TxDvgaGainQdbGc3W::new(self, 24)
    }
}
#[doc = "dfe_ctrl_12.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl12Spec;
impl crate::RegisterSpec for DfeCtrl12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_12::R`](R) reader structure"]
impl crate::Readable for DfeCtrl12Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_12::W`](W) writer structure"]
impl crate::Writable for DfeCtrl12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_12 to value 0"]
impl crate::Resettable for DfeCtrl12Spec {}

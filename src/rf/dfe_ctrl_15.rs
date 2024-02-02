#[doc = "Register `dfe_ctrl_15` reader"]
pub type R = crate::R<DfeCtrl15Spec>;
#[doc = "Register `dfe_ctrl_15` writer"]
pub type W = crate::W<DfeCtrl15Spec>;
#[doc = "Field `tx_dvga_gain_qdb_gc12` reader - "]
pub type TxDvgaGainQdbGc12R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc12` writer - "]
pub type TxDvgaGainQdbGc12W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc13` reader - "]
pub type TxDvgaGainQdbGc13R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc13` writer - "]
pub type TxDvgaGainQdbGc13W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc14` reader - "]
pub type TxDvgaGainQdbGc14R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc14` writer - "]
pub type TxDvgaGainQdbGc14W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc15` reader - "]
pub type TxDvgaGainQdbGc15R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc15` writer - "]
pub type TxDvgaGainQdbGc15W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc12(&self) -> TxDvgaGainQdbGc12R {
        TxDvgaGainQdbGc12R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc13(&self) -> TxDvgaGainQdbGc13R {
        TxDvgaGainQdbGc13R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc14(&self) -> TxDvgaGainQdbGc14R {
        TxDvgaGainQdbGc14R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc15(&self) -> TxDvgaGainQdbGc15R {
        TxDvgaGainQdbGc15R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc12(&mut self) -> TxDvgaGainQdbGc12W<'_, DfeCtrl15Spec> {
        TxDvgaGainQdbGc12W::new(self, 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc13(&mut self) -> TxDvgaGainQdbGc13W<'_, DfeCtrl15Spec> {
        TxDvgaGainQdbGc13W::new(self, 8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc14(&mut self) -> TxDvgaGainQdbGc14W<'_, DfeCtrl15Spec> {
        TxDvgaGainQdbGc14W::new(self, 16)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc15(&mut self) -> TxDvgaGainQdbGc15W<'_, DfeCtrl15Spec> {
        TxDvgaGainQdbGc15W::new(self, 24)
    }
}
#[doc = "dfe_ctrl_15.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl15Spec;
impl crate::RegisterSpec for DfeCtrl15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_15::R`](R) reader structure"]
impl crate::Readable for DfeCtrl15Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_15::W`](W) writer structure"]
impl crate::Writable for DfeCtrl15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_15 to value 0"]
impl crate::Resettable for DfeCtrl15Spec {}

#[doc = "Register `dfe_ctrl_18` reader"]
pub type R = crate::R<DfeCtrl18Spec>;
#[doc = "Register `dfe_ctrl_18` writer"]
pub type W = crate::W<DfeCtrl18Spec>;
#[doc = "Field `tx_dvga_gain_qdb_ble_gc0` reader - "]
pub type TxDvgaGainQdbBleGc0R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_ble_gc0` writer - "]
pub type TxDvgaGainQdbBleGc0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_ble_gc1` reader - "]
pub type TxDvgaGainQdbBleGc1R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_ble_gc1` writer - "]
pub type TxDvgaGainQdbBleGc1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_ble_gc2` reader - "]
pub type TxDvgaGainQdbBleGc2R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_ble_gc2` writer - "]
pub type TxDvgaGainQdbBleGc2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc0(&self) -> TxDvgaGainQdbBleGc0R {
        TxDvgaGainQdbBleGc0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc1(&self) -> TxDvgaGainQdbBleGc1R {
        TxDvgaGainQdbBleGc1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc2(&self) -> TxDvgaGainQdbBleGc2R {
        TxDvgaGainQdbBleGc2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc0(&mut self) -> TxDvgaGainQdbBleGc0W<'_, DfeCtrl18Spec> {
        TxDvgaGainQdbBleGc0W::new(self, 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc1(&mut self) -> TxDvgaGainQdbBleGc1W<'_, DfeCtrl18Spec> {
        TxDvgaGainQdbBleGc1W::new(self, 8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc2(&mut self) -> TxDvgaGainQdbBleGc2W<'_, DfeCtrl18Spec> {
        TxDvgaGainQdbBleGc2W::new(self, 16)
    }
}
#[doc = "dfe_ctrl_18.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl18Spec;
impl crate::RegisterSpec for DfeCtrl18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_18::R`](R) reader structure"]
impl crate::Readable for DfeCtrl18Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_18::W`](W) writer structure"]
impl crate::Writable for DfeCtrl18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_18 to value 0"]
impl crate::Resettable for DfeCtrl18Spec {}

#[doc = "Register `dfe_ctrl_13` reader"]
pub type R = crate::R<DfeCtrl13Spec>;
#[doc = "Register `dfe_ctrl_13` writer"]
pub type W = crate::W<DfeCtrl13Spec>;
#[doc = "Field `tx_dvga_gain_qdb_gc4` reader - "]
pub type TxDvgaGainQdbGc4R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc4` writer - "]
pub type TxDvgaGainQdbGc4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc5` reader - "]
pub type TxDvgaGainQdbGc5R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc5` writer - "]
pub type TxDvgaGainQdbGc5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc6` reader - "]
pub type TxDvgaGainQdbGc6R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc6` writer - "]
pub type TxDvgaGainQdbGc6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc7` reader - "]
pub type TxDvgaGainQdbGc7R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc7` writer - "]
pub type TxDvgaGainQdbGc7W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc4(&self) -> TxDvgaGainQdbGc4R {
        TxDvgaGainQdbGc4R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc5(&self) -> TxDvgaGainQdbGc5R {
        TxDvgaGainQdbGc5R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc6(&self) -> TxDvgaGainQdbGc6R {
        TxDvgaGainQdbGc6R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc7(&self) -> TxDvgaGainQdbGc7R {
        TxDvgaGainQdbGc7R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc4(&mut self) -> TxDvgaGainQdbGc4W<'_, DfeCtrl13Spec> {
        TxDvgaGainQdbGc4W::new(self, 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc5(&mut self) -> TxDvgaGainQdbGc5W<'_, DfeCtrl13Spec> {
        TxDvgaGainQdbGc5W::new(self, 8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc6(&mut self) -> TxDvgaGainQdbGc6W<'_, DfeCtrl13Spec> {
        TxDvgaGainQdbGc6W::new(self, 16)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc7(&mut self) -> TxDvgaGainQdbGc7W<'_, DfeCtrl13Spec> {
        TxDvgaGainQdbGc7W::new(self, 24)
    }
}
#[doc = "dfe_ctrl_13.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl13Spec;
impl crate::RegisterSpec for DfeCtrl13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_13::R`](R) reader structure"]
impl crate::Readable for DfeCtrl13Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_13::W`](W) writer structure"]
impl crate::Writable for DfeCtrl13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_13 to value 0"]
impl crate::Resettable for DfeCtrl13Spec {}

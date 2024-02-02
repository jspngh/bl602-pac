#[doc = "Register `dfe_ctrl_14` reader"]
pub type R = crate::R<DfeCtrl14Spec>;
#[doc = "Register `dfe_ctrl_14` writer"]
pub type W = crate::W<DfeCtrl14Spec>;
#[doc = "Field `tx_dvga_gain_qdb_gc8` reader - "]
pub type TxDvgaGainQdbGc8R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc8` writer - "]
pub type TxDvgaGainQdbGc8W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc9` reader - "]
pub type TxDvgaGainQdbGc9R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc9` writer - "]
pub type TxDvgaGainQdbGc9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc10` reader - "]
pub type TxDvgaGainQdbGc10R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc10` writer - "]
pub type TxDvgaGainQdbGc10W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tx_dvga_gain_qdb_gc11` reader - "]
pub type TxDvgaGainQdbGc11R = crate::FieldReader;
#[doc = "Field `tx_dvga_gain_qdb_gc11` writer - "]
pub type TxDvgaGainQdbGc11W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc8(&self) -> TxDvgaGainQdbGc8R {
        TxDvgaGainQdbGc8R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc9(&self) -> TxDvgaGainQdbGc9R {
        TxDvgaGainQdbGc9R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc10(&self) -> TxDvgaGainQdbGc10R {
        TxDvgaGainQdbGc10R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc11(&self) -> TxDvgaGainQdbGc11R {
        TxDvgaGainQdbGc11R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc8(&mut self) -> TxDvgaGainQdbGc8W<'_, DfeCtrl14Spec> {
        TxDvgaGainQdbGc8W::new(self, 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc9(&mut self) -> TxDvgaGainQdbGc9W<'_, DfeCtrl14Spec> {
        TxDvgaGainQdbGc9W::new(self, 8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc10(&mut self) -> TxDvgaGainQdbGc10W<'_, DfeCtrl14Spec> {
        TxDvgaGainQdbGc10W::new(self, 16)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc11(&mut self) -> TxDvgaGainQdbGc11W<'_, DfeCtrl14Spec> {
        TxDvgaGainQdbGc11W::new(self, 24)
    }
}
#[doc = "dfe_ctrl_14.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl14Spec;
impl crate::RegisterSpec for DfeCtrl14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_14::R`](R) reader structure"]
impl crate::Readable for DfeCtrl14Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_14::W`](W) writer structure"]
impl crate::Writable for DfeCtrl14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_14 to value 0"]
impl crate::Resettable for DfeCtrl14Spec {}

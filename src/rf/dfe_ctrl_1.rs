#[doc = "Register `dfe_ctrl_1` reader"]
pub type R = crate::R<DfeCtrl1Spec>;
#[doc = "Register `dfe_ctrl_1` writer"]
pub type W = crate::W<DfeCtrl1Spec>;
#[doc = "Field `tx_dac_os_i` reader - "]
pub type TxDacOsIR = crate::FieldReader<u16>;
#[doc = "Field `tx_dac_os_i` writer - "]
pub type TxDacOsIW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `tx_dac_os_q` reader - "]
pub type TxDacOsQR = crate::FieldReader<u16>;
#[doc = "Field `tx_dac_os_q` writer - "]
pub type TxDacOsQW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `tx_dac_dat_format` reader - "]
pub type TxDacDatFormatR = crate::BitReader;
#[doc = "Field `tx_dac_dat_format` writer - "]
pub type TxDacDatFormatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_dac_iq_swap` reader - "]
pub type TxDacIqSwapR = crate::BitReader;
#[doc = "Field `tx_dac_iq_swap` writer - "]
pub type TxDacIqSwapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tx_dac_os_i(&self) -> TxDacOsIR {
        TxDacOsIR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tx_dac_os_q(&self) -> TxDacOsQR {
        TxDacOsQR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tx_dac_dat_format(&self) -> TxDacDatFormatR {
        TxDacDatFormatR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dac_iq_swap(&self) -> TxDacIqSwapR {
        TxDacIqSwapR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tx_dac_os_i(&mut self) -> TxDacOsIW<'_, DfeCtrl1Spec> {
        TxDacOsIW::new(self, 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tx_dac_os_q(&mut self) -> TxDacOsQW<'_, DfeCtrl1Spec> {
        TxDacOsQW::new(self, 16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tx_dac_dat_format(&mut self) -> TxDacDatFormatW<'_, DfeCtrl1Spec> {
        TxDacDatFormatW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dac_iq_swap(&mut self) -> TxDacIqSwapW<'_, DfeCtrl1Spec> {
        TxDacIqSwapW::new(self, 31)
    }
}
#[doc = "dfe_ctrl_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl1Spec;
impl crate::RegisterSpec for DfeCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_1::R`](R) reader structure"]
impl crate::Readable for DfeCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_1::W`](W) writer structure"]
impl crate::Writable for DfeCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_1 to value 0"]
impl crate::Resettable for DfeCtrl1Spec {}

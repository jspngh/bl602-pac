#[doc = "Register `dfe_ctrl_6` reader"]
pub type R = crate::R<DfeCtrl6Spec>;
#[doc = "Register `dfe_ctrl_6` writer"]
pub type W = crate::W<DfeCtrl6Spec>;
#[doc = "Field `rx_pm_freqshift_cw` reader - "]
pub type RxPmFreqshiftCwR = crate::FieldReader<u32>;
#[doc = "Field `rx_pm_freqshift_cw` writer - "]
pub type RxPmFreqshiftCwW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `rx_pm_freqshift_en` reader - "]
pub type RxPmFreqshiftEnR = crate::BitReader;
#[doc = "Field `rx_pm_freqshift_en` writer - "]
pub type RxPmFreqshiftEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_pm_done` reader - "]
pub type RxPmDoneR = crate::BitReader;
#[doc = "Field `rx_pm_done` writer - "]
pub type RxPmDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_pm_en` reader - "]
pub type RxPmEnR = crate::BitReader;
#[doc = "Field `rx_pm_en` writer - "]
pub type RxPmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_pm_in_sel` reader - "]
pub type RxPmInSelR = crate::FieldReader;
#[doc = "Field `rx_pm_in_sel` writer - "]
pub type RxPmInSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rx_pm_freqshift_cw(&self) -> RxPmFreqshiftCwR {
        RxPmFreqshiftCwR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_pm_freqshift_en(&self) -> RxPmFreqshiftEnR {
        RxPmFreqshiftEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_pm_done(&self) -> RxPmDoneR {
        RxPmDoneR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_pm_en(&self) -> RxPmEnR {
        RxPmEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rx_pm_in_sel(&self) -> RxPmInSelR {
        RxPmInSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rx_pm_freqshift_cw(&mut self) -> RxPmFreqshiftCwW<'_, DfeCtrl6Spec> {
        RxPmFreqshiftCwW::new(self, 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_pm_freqshift_en(&mut self) -> RxPmFreqshiftEnW<'_, DfeCtrl6Spec> {
        RxPmFreqshiftEnW::new(self, 20)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_pm_done(&mut self) -> RxPmDoneW<'_, DfeCtrl6Spec> {
        RxPmDoneW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_pm_en(&mut self) -> RxPmEnW<'_, DfeCtrl6Spec> {
        RxPmEnW::new(self, 29)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rx_pm_in_sel(&mut self) -> RxPmInSelW<'_, DfeCtrl6Spec> {
        RxPmInSelW::new(self, 30)
    }
}
#[doc = "dfe_ctrl_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl6Spec;
impl crate::RegisterSpec for DfeCtrl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_6::R`](R) reader structure"]
impl crate::Readable for DfeCtrl6Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_6::W`](W) writer structure"]
impl crate::Writable for DfeCtrl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_6 to value 0"]
impl crate::Resettable for DfeCtrl6Spec {}

#[doc = "Register `adda2` reader"]
pub type R = crate::R<Adda2Spec>;
#[doc = "Register `adda2` writer"]
pub type W = crate::W<Adda2Spec>;
#[doc = "Field `adc_vref_sel` reader - "]
pub type AdcVrefSelR = crate::FieldReader;
#[doc = "Field `adc_vref_sel` writer - "]
pub type AdcVrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_dly_ctl` reader - "]
pub type AdcDlyCtlR = crate::FieldReader;
#[doc = "Field `adc_dly_ctl` writer - "]
pub type AdcDlyCtlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_dvdd_sel` reader - "]
pub type AdcDvddSelR = crate::FieldReader;
#[doc = "Field `adc_dvdd_sel` writer - "]
pub type AdcDvddSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_sar_ascal_en` reader - "]
pub type AdcSarAscalEnR = crate::BitReader;
#[doc = "Field `adc_sar_ascal_en` writer - "]
pub type AdcSarAscalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_gt_rm` reader - "]
pub type AdcGtRmR = crate::BitReader;
#[doc = "Field `adc_gt_rm` writer - "]
pub type AdcGtRmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_clk_sync_inv` reader - "]
pub type AdcClkSyncInvR = crate::BitReader;
#[doc = "Field `adc_clk_sync_inv` writer - "]
pub type AdcClkSyncInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_clk_inv` reader - "]
pub type AdcClkInvR = crate::BitReader;
#[doc = "Field `adc_clk_inv` writer - "]
pub type AdcClkInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_clk_div_sel` reader - "]
pub type AdcClkDivSelR = crate::BitReader;
#[doc = "Field `adc_clk_div_sel` writer - "]
pub type AdcClkDivSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adc_vref_sel(&self) -> AdcVrefSelR {
        AdcVrefSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc_dly_ctl(&self) -> AdcDlyCtlR {
        AdcDlyCtlR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adc_dvdd_sel(&self) -> AdcDvddSelR {
        AdcDvddSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_sar_ascal_en(&self) -> AdcSarAscalEnR {
        AdcSarAscalEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adc_gt_rm(&self) -> AdcGtRmR {
        AdcGtRmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc_clk_sync_inv(&self) -> AdcClkSyncInvR {
        AdcClkSyncInvR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adc_clk_inv(&self) -> AdcClkInvR {
        AdcClkInvR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adc_clk_div_sel(&self) -> AdcClkDivSelR {
        AdcClkDivSelR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adc_vref_sel(&mut self) -> AdcVrefSelW<'_, Adda2Spec> {
        AdcVrefSelW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc_dly_ctl(&mut self) -> AdcDlyCtlW<'_, Adda2Spec> {
        AdcDlyCtlW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adc_dvdd_sel(&mut self) -> AdcDvddSelW<'_, Adda2Spec> {
        AdcDvddSelW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_sar_ascal_en(&mut self) -> AdcSarAscalEnW<'_, Adda2Spec> {
        AdcSarAscalEnW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adc_gt_rm(&mut self) -> AdcGtRmW<'_, Adda2Spec> {
        AdcGtRmW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc_clk_sync_inv(&mut self) -> AdcClkSyncInvW<'_, Adda2Spec> {
        AdcClkSyncInvW::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adc_clk_inv(&mut self) -> AdcClkInvW<'_, Adda2Spec> {
        AdcClkInvW::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adc_clk_div_sel(&mut self) -> AdcClkDivSelW<'_, Adda2Spec> {
        AdcClkDivSelW::new(self, 28)
    }
}
#[doc = "adda2.\n\nYou can [`read`](crate::Reg::read) this register and get [`adda2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adda2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adda2Spec;
impl crate::RegisterSpec for Adda2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adda2::R`](R) reader structure"]
impl crate::Readable for Adda2Spec {}
#[doc = "`write(|w| ..)` method takes [`adda2::W`](W) writer structure"]
impl crate::Writable for Adda2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adda2 to value 0"]
impl crate::Resettable for Adda2Spec {}

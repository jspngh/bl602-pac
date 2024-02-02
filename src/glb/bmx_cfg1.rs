#[doc = "Register `bmx_cfg1` reader"]
pub type R = crate::R<BmxCfg1Spec>;
#[doc = "Register `bmx_cfg1` writer"]
pub type W = crate::W<BmxCfg1Spec>;
#[doc = "Field `bmx_timeout_en` reader - "]
pub type BmxTimeoutEnR = crate::FieldReader;
#[doc = "Field `bmx_timeout_en` writer - "]
pub type BmxTimeoutEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bmx_arb_mode` reader - "]
pub type BmxArbModeR = crate::FieldReader;
#[doc = "Field `bmx_arb_mode` writer - "]
pub type BmxArbModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bmx_err_en` reader - "]
pub type BmxErrEnR = crate::BitReader;
#[doc = "Field `bmx_err_en` writer - "]
pub type BmxErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bmx_busy_option_dis` reader - "]
pub type BmxBusyOptionDisR = crate::BitReader;
#[doc = "Field `bmx_busy_option_dis` writer - "]
pub type BmxBusyOptionDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bmx_gating_dis` reader - "]
pub type BmxGatingDisR = crate::BitReader;
#[doc = "Field `bmx_gating_dis` writer - "]
pub type BmxGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsel_option` reader - "]
pub type HselOptionR = crate::FieldReader;
#[doc = "Field `hsel_option` writer - "]
pub type HselOptionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pds_apb_cfg` reader - "]
pub type PdsApbCfgR = crate::FieldReader;
#[doc = "Field `pds_apb_cfg` writer - "]
pub type PdsApbCfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `hbn_apb_cfg` reader - "]
pub type HbnApbCfgR = crate::FieldReader;
#[doc = "Field `hbn_apb_cfg` writer - "]
pub type HbnApbCfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn bmx_timeout_en(&self) -> BmxTimeoutEnR {
        BmxTimeoutEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn bmx_arb_mode(&self) -> BmxArbModeR {
        BmxArbModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bmx_err_en(&self) -> BmxErrEnR {
        BmxErrEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bmx_busy_option_dis(&self) -> BmxBusyOptionDisR {
        BmxBusyOptionDisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bmx_gating_dis(&self) -> BmxGatingDisR {
        BmxGatingDisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hsel_option(&self) -> HselOptionR {
        HselOptionR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pds_apb_cfg(&self) -> PdsApbCfgR {
        PdsApbCfgR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hbn_apb_cfg(&self) -> HbnApbCfgR {
        HbnApbCfgR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn bmx_timeout_en(&mut self) -> BmxTimeoutEnW<'_, BmxCfg1Spec> {
        BmxTimeoutEnW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn bmx_arb_mode(&mut self) -> BmxArbModeW<'_, BmxCfg1Spec> {
        BmxArbModeW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bmx_err_en(&mut self) -> BmxErrEnW<'_, BmxCfg1Spec> {
        BmxErrEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bmx_busy_option_dis(&mut self) -> BmxBusyOptionDisW<'_, BmxCfg1Spec> {
        BmxBusyOptionDisW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bmx_gating_dis(&mut self) -> BmxGatingDisW<'_, BmxCfg1Spec> {
        BmxGatingDisW::new(self, 10)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hsel_option(&mut self) -> HselOptionW<'_, BmxCfg1Spec> {
        HselOptionW::new(self, 12)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pds_apb_cfg(&mut self) -> PdsApbCfgW<'_, BmxCfg1Spec> {
        PdsApbCfgW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hbn_apb_cfg(&mut self) -> HbnApbCfgW<'_, BmxCfg1Spec> {
        HbnApbCfgW::new(self, 24)
    }
}
#[doc = "bmx_cfg1.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxCfg1Spec;
impl crate::RegisterSpec for BmxCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg1::R`](R) reader structure"]
impl crate::Readable for BmxCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg1::W`](W) writer structure"]
impl crate::Writable for BmxCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets bmx_cfg1 to value 0"]
impl crate::Resettable for BmxCfg1Spec {}

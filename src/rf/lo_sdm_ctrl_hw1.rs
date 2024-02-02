#[doc = "Register `lo_sdm_ctrl_hw1` reader"]
pub type R = crate::R<LoSdmCtrlHw1Spec>;
#[doc = "Register `lo_sdm_ctrl_hw1` writer"]
pub type W = crate::W<LoSdmCtrlHw1Spec>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2412` reader - "]
pub type LoSdmDitherSelWlan2412R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2412` writer - "]
pub type LoSdmDitherSelWlan2412W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2417` reader - "]
pub type LoSdmDitherSelWlan2417R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2417` writer - "]
pub type LoSdmDitherSelWlan2417W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2422` reader - "]
pub type LoSdmDitherSelWlan2422R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2422` writer - "]
pub type LoSdmDitherSelWlan2422W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2427` reader - "]
pub type LoSdmDitherSelWlan2427R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2427` writer - "]
pub type LoSdmDitherSelWlan2427W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2432` reader - "]
pub type LoSdmDitherSelWlan2432R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2432` writer - "]
pub type LoSdmDitherSelWlan2432W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2437` reader - "]
pub type LoSdmDitherSelWlan2437R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2437` writer - "]
pub type LoSdmDitherSelWlan2437W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2442` reader - "]
pub type LoSdmDitherSelWlan2442R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2442` writer - "]
pub type LoSdmDitherSelWlan2442W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2447` reader - "]
pub type LoSdmDitherSelWlan2447R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2447` writer - "]
pub type LoSdmDitherSelWlan2447W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2452` reader - "]
pub type LoSdmDitherSelWlan2452R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2452` writer - "]
pub type LoSdmDitherSelWlan2452W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2457` reader - "]
pub type LoSdmDitherSelWlan2457R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2457` writer - "]
pub type LoSdmDitherSelWlan2457W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2462` reader - "]
pub type LoSdmDitherSelWlan2462R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2462` writer - "]
pub type LoSdmDitherSelWlan2462W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2467` reader - "]
pub type LoSdmDitherSelWlan2467R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2467` writer - "]
pub type LoSdmDitherSelWlan2467W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2472` reader - "]
pub type LoSdmDitherSelWlan2472R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2472` writer - "]
pub type LoSdmDitherSelWlan2472W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_wlan_2484` reader - "]
pub type LoSdmDitherSelWlan2484R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_wlan_2484` writer - "]
pub type LoSdmDitherSelWlan2484W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2412(&self) -> LoSdmDitherSelWlan2412R {
        LoSdmDitherSelWlan2412R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2417(&self) -> LoSdmDitherSelWlan2417R {
        LoSdmDitherSelWlan2417R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2422(&self) -> LoSdmDitherSelWlan2422R {
        LoSdmDitherSelWlan2422R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2427(&self) -> LoSdmDitherSelWlan2427R {
        LoSdmDitherSelWlan2427R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2432(&self) -> LoSdmDitherSelWlan2432R {
        LoSdmDitherSelWlan2432R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2437(&self) -> LoSdmDitherSelWlan2437R {
        LoSdmDitherSelWlan2437R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2442(&self) -> LoSdmDitherSelWlan2442R {
        LoSdmDitherSelWlan2442R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2447(&self) -> LoSdmDitherSelWlan2447R {
        LoSdmDitherSelWlan2447R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2452(&self) -> LoSdmDitherSelWlan2452R {
        LoSdmDitherSelWlan2452R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2457(&self) -> LoSdmDitherSelWlan2457R {
        LoSdmDitherSelWlan2457R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2462(&self) -> LoSdmDitherSelWlan2462R {
        LoSdmDitherSelWlan2462R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2467(&self) -> LoSdmDitherSelWlan2467R {
        LoSdmDitherSelWlan2467R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2472(&self) -> LoSdmDitherSelWlan2472R {
        LoSdmDitherSelWlan2472R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2484(&self) -> LoSdmDitherSelWlan2484R {
        LoSdmDitherSelWlan2484R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2412(&mut self) -> LoSdmDitherSelWlan2412W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2412W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2417(&mut self) -> LoSdmDitherSelWlan2417W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2417W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2422(&mut self) -> LoSdmDitherSelWlan2422W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2422W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2427(&mut self) -> LoSdmDitherSelWlan2427W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2427W::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2432(&mut self) -> LoSdmDitherSelWlan2432W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2432W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2437(&mut self) -> LoSdmDitherSelWlan2437W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2437W::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2442(&mut self) -> LoSdmDitherSelWlan2442W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2442W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2447(&mut self) -> LoSdmDitherSelWlan2447W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2447W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2452(&mut self) -> LoSdmDitherSelWlan2452W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2452W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2457(&mut self) -> LoSdmDitherSelWlan2457W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2457W::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2462(&mut self) -> LoSdmDitherSelWlan2462W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2462W::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2467(&mut self) -> LoSdmDitherSelWlan2467W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2467W::new(self, 22)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2472(&mut self) -> LoSdmDitherSelWlan2472W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2472W::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2484(&mut self) -> LoSdmDitherSelWlan2484W<'_, LoSdmCtrlHw1Spec> {
        LoSdmDitherSelWlan2484W::new(self, 26)
    }
}
#[doc = "lo_sdm_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoSdmCtrlHw1Spec;
impl crate::RegisterSpec for LoSdmCtrlHw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_sdm_ctrl_hw1::R`](R) reader structure"]
impl crate::Readable for LoSdmCtrlHw1Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_sdm_ctrl_hw1::W`](W) writer structure"]
impl crate::Writable for LoSdmCtrlHw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw1 to value 0"]
impl crate::Resettable for LoSdmCtrlHw1Spec {}

#[doc = "Register `dcdc18_top_0` reader"]
pub type R = crate::R<Dcdc18Top0Spec>;
#[doc = "Register `dcdc18_top_0` writer"]
pub type W = crate::W<Dcdc18Top0Spec>;
#[doc = "Field `dcdc18_vout_sel_aon` reader - "]
pub type Dcdc18VoutSelAonR = crate::FieldReader;
#[doc = "Field `dcdc18_vout_sel_aon` writer - "]
pub type Dcdc18VoutSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dcdc18_vpfm_aon` reader - "]
pub type Dcdc18VpfmAonR = crate::FieldReader;
#[doc = "Field `dcdc18_vpfm_aon` writer - "]
pub type Dcdc18VpfmAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc18_osc_2m_mode_aon` reader - "]
pub type Dcdc18Osc2mModeAonR = crate::BitReader;
#[doc = "Field `dcdc18_osc_2m_mode_aon` writer - "]
pub type Dcdc18Osc2mModeAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_osc_freq_trim_aon` reader - "]
pub type Dcdc18OscFreqTrimAonR = crate::FieldReader;
#[doc = "Field `dcdc18_osc_freq_trim_aon` writer - "]
pub type Dcdc18OscFreqTrimAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc18_slope_curr_sel_aon` reader - "]
pub type Dcdc18SlopeCurrSelAonR = crate::FieldReader;
#[doc = "Field `dcdc18_slope_curr_sel_aon` writer - "]
pub type Dcdc18SlopeCurrSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dcdc18_stop_osc_aon` reader - "]
pub type Dcdc18StopOscAonR = crate::BitReader;
#[doc = "Field `dcdc18_stop_osc_aon` writer - "]
pub type Dcdc18StopOscAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_slow_osc_aon` reader - "]
pub type Dcdc18SlowOscAonR = crate::BitReader;
#[doc = "Field `dcdc18_slow_osc_aon` writer - "]
pub type Dcdc18SlowOscAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_osc_inhibit_t2_aon` reader - "]
pub type Dcdc18OscInhibitT2AonR = crate::BitReader;
#[doc = "Field `dcdc18_osc_inhibit_t2_aon` writer - "]
pub type Dcdc18OscInhibitT2AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_sstart_time_aon` reader - "]
pub type Dcdc18SstartTimeAonR = crate::FieldReader;
#[doc = "Field `dcdc18_sstart_time_aon` writer - "]
pub type Dcdc18SstartTimeAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dcdc18_rdy_aon` reader - "]
pub type Dcdc18RdyAonR = crate::BitReader;
impl R {
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn dcdc18_vout_sel_aon(&self) -> Dcdc18VoutSelAonR {
        Dcdc18VoutSelAonR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc18_vpfm_aon(&self) -> Dcdc18VpfmAonR {
        Dcdc18VpfmAonR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dcdc18_osc_2m_mode_aon(&self) -> Dcdc18Osc2mModeAonR {
        Dcdc18Osc2mModeAonR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_osc_freq_trim_aon(&self) -> Dcdc18OscFreqTrimAonR {
        Dcdc18OscFreqTrimAonR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn dcdc18_slope_curr_sel_aon(&self) -> Dcdc18SlopeCurrSelAonR {
        Dcdc18SlopeCurrSelAonR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc18_stop_osc_aon(&self) -> Dcdc18StopOscAonR {
        Dcdc18StopOscAonR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc18_slow_osc_aon(&self) -> Dcdc18SlowOscAonR {
        Dcdc18SlowOscAonR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc18_osc_inhibit_t2_aon(&self) -> Dcdc18OscInhibitT2AonR {
        Dcdc18OscInhibitT2AonR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc18_sstart_time_aon(&self) -> Dcdc18SstartTimeAonR {
        Dcdc18SstartTimeAonR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dcdc18_rdy_aon(&self) -> Dcdc18RdyAonR {
        Dcdc18RdyAonR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn dcdc18_vout_sel_aon(&mut self) -> Dcdc18VoutSelAonW<'_, Dcdc18Top0Spec> {
        Dcdc18VoutSelAonW::new(self, 1)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc18_vpfm_aon(&mut self) -> Dcdc18VpfmAonW<'_, Dcdc18Top0Spec> {
        Dcdc18VpfmAonW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dcdc18_osc_2m_mode_aon(&mut self) -> Dcdc18Osc2mModeAonW<'_, Dcdc18Top0Spec> {
        Dcdc18Osc2mModeAonW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_osc_freq_trim_aon(&mut self) -> Dcdc18OscFreqTrimAonW<'_, Dcdc18Top0Spec> {
        Dcdc18OscFreqTrimAonW::new(self, 16)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn dcdc18_slope_curr_sel_aon(&mut self) -> Dcdc18SlopeCurrSelAonW<'_, Dcdc18Top0Spec> {
        Dcdc18SlopeCurrSelAonW::new(self, 20)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc18_stop_osc_aon(&mut self) -> Dcdc18StopOscAonW<'_, Dcdc18Top0Spec> {
        Dcdc18StopOscAonW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc18_slow_osc_aon(&mut self) -> Dcdc18SlowOscAonW<'_, Dcdc18Top0Spec> {
        Dcdc18SlowOscAonW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc18_osc_inhibit_t2_aon(&mut self) -> Dcdc18OscInhibitT2AonW<'_, Dcdc18Top0Spec> {
        Dcdc18OscInhibitT2AonW::new(self, 27)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc18_sstart_time_aon(&mut self) -> Dcdc18SstartTimeAonW<'_, Dcdc18Top0Spec> {
        Dcdc18SstartTimeAonW::new(self, 28)
    }
}
#[doc = "dcdc18_top_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc18_top_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc18_top_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcdc18Top0Spec;
impl crate::RegisterSpec for Dcdc18Top0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc18_top_0::R`](R) reader structure"]
impl crate::Readable for Dcdc18Top0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdc18_top_0::W`](W) writer structure"]
impl crate::Writable for Dcdc18Top0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dcdc18_top_0 to value 0x8a58_0736"]
impl crate::Resettable for Dcdc18Top0Spec {
    const RESET_VALUE: u32 = 0x8a58_0736;
}

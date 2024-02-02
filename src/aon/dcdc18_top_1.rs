#[doc = "Register `dcdc18_top_1` reader"]
pub type R = crate::R<Dcdc18Top1Spec>;
#[doc = "Register `dcdc18_top_1` writer"]
pub type W = crate::W<Dcdc18Top1Spec>;
#[doc = "Field `dcdc18_force_cs_zvs_aon` reader - "]
pub type Dcdc18ForceCsZvsAonR = crate::BitReader;
#[doc = "Field `dcdc18_force_cs_zvs_aon` writer - "]
pub type Dcdc18ForceCsZvsAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_cs_delay_aon` reader - "]
pub type Dcdc18CsDelayAonR = crate::FieldReader;
#[doc = "Field `dcdc18_cs_delay_aon` writer - "]
pub type Dcdc18CsDelayAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc18_zvs_td_opt_aon` reader - "]
pub type Dcdc18ZvsTdOptAonR = crate::FieldReader;
#[doc = "Field `dcdc18_zvs_td_opt_aon` writer - "]
pub type Dcdc18ZvsTdOptAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc18_nonoverlap_td_aon` reader - "]
pub type Dcdc18NonoverlapTdAonR = crate::FieldReader;
#[doc = "Field `dcdc18_nonoverlap_td_aon` writer - "]
pub type Dcdc18NonoverlapTdAonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dcdc18_rc_sel_aon` reader - "]
pub type Dcdc18RcSelAonR = crate::FieldReader;
#[doc = "Field `dcdc18_rc_sel_aon` writer - "]
pub type Dcdc18RcSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc18_chf_sel_aon` reader - "]
pub type Dcdc18ChfSelAonR = crate::FieldReader;
#[doc = "Field `dcdc18_chf_sel_aon` writer - "]
pub type Dcdc18ChfSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc18_cfb_sel_aon` reader - "]
pub type Dcdc18CfbSelAonR = crate::FieldReader;
#[doc = "Field `dcdc18_cfb_sel_aon` writer - "]
pub type Dcdc18CfbSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc18_en_antiring_aon` reader - "]
pub type Dcdc18EnAntiringAonR = crate::BitReader;
#[doc = "Field `dcdc18_en_antiring_aon` writer - "]
pub type Dcdc18EnAntiringAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_pulldown_aon` reader - "]
pub type Dcdc18PulldownAonR = crate::BitReader;
#[doc = "Field `dcdc18_pulldown_aon` writer - "]
pub type Dcdc18PulldownAonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dcdc18_force_cs_zvs_aon(&self) -> Dcdc18ForceCsZvsAonR {
        Dcdc18ForceCsZvsAonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn dcdc18_cs_delay_aon(&self) -> Dcdc18CsDelayAonR {
        Dcdc18CsDelayAonR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc18_zvs_td_opt_aon(&self) -> Dcdc18ZvsTdOptAonR {
        Dcdc18ZvsTdOptAonR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn dcdc18_nonoverlap_td_aon(&self) -> Dcdc18NonoverlapTdAonR {
        Dcdc18NonoverlapTdAonR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_rc_sel_aon(&self) -> Dcdc18RcSelAonR {
        Dcdc18RcSelAonR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_chf_sel_aon(&self) -> Dcdc18ChfSelAonR {
        Dcdc18ChfSelAonR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dcdc18_cfb_sel_aon(&self) -> Dcdc18CfbSelAonR {
        Dcdc18CfbSelAonR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dcdc18_en_antiring_aon(&self) -> Dcdc18EnAntiringAonR {
        Dcdc18EnAntiringAonR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dcdc18_pulldown_aon(&self) -> Dcdc18PulldownAonR {
        Dcdc18PulldownAonR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dcdc18_force_cs_zvs_aon(&mut self) -> Dcdc18ForceCsZvsAonW<'_, Dcdc18Top1Spec> {
        Dcdc18ForceCsZvsAonW::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn dcdc18_cs_delay_aon(&mut self) -> Dcdc18CsDelayAonW<'_, Dcdc18Top1Spec> {
        Dcdc18CsDelayAonW::new(self, 1)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc18_zvs_td_opt_aon(&mut self) -> Dcdc18ZvsTdOptAonW<'_, Dcdc18Top1Spec> {
        Dcdc18ZvsTdOptAonW::new(self, 4)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn dcdc18_nonoverlap_td_aon(&mut self) -> Dcdc18NonoverlapTdAonW<'_, Dcdc18Top1Spec> {
        Dcdc18NonoverlapTdAonW::new(self, 8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_rc_sel_aon(&mut self) -> Dcdc18RcSelAonW<'_, Dcdc18Top1Spec> {
        Dcdc18RcSelAonW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_chf_sel_aon(&mut self) -> Dcdc18ChfSelAonW<'_, Dcdc18Top1Spec> {
        Dcdc18ChfSelAonW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dcdc18_cfb_sel_aon(&mut self) -> Dcdc18CfbSelAonW<'_, Dcdc18Top1Spec> {
        Dcdc18CfbSelAonW::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dcdc18_en_antiring_aon(&mut self) -> Dcdc18EnAntiringAonW<'_, Dcdc18Top1Spec> {
        Dcdc18EnAntiringAonW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dcdc18_pulldown_aon(&mut self) -> Dcdc18PulldownAonW<'_, Dcdc18Top1Spec> {
        Dcdc18PulldownAonW::new(self, 29)
    }
}
#[doc = "dcdc18_top_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc18_top_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc18_top_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcdc18Top1Spec;
impl crate::RegisterSpec for Dcdc18Top1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc18_top_1::R`](R) reader structure"]
impl crate::Readable for Dcdc18Top1Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdc18_top_1::W`](W) writer structure"]
impl crate::Writable for Dcdc18Top1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dcdc18_top_1 to value 0x1818_0048"]
impl crate::Resettable for Dcdc18Top1Spec {
    const RESET_VALUE: u32 = 0x1818_0048;
}

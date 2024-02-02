#[doc = "Register `tx_iq_gain_hw2` reader"]
pub type R = crate::R<TxIqGainHw2Spec>;
#[doc = "Register `tx_iq_gain_hw2` writer"]
pub type W = crate::W<TxIqGainHw2Spec>;
#[doc = "Field `tx_iq_phase_comp_gc2` reader - "]
pub type TxIqPhaseCompGc2R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_phase_comp_gc2` writer - "]
pub type TxIqPhaseCompGc2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tx_iq_gain_comp_gc2` reader - "]
pub type TxIqGainCompGc2R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_gain_comp_gc2` writer - "]
pub type TxIqGainCompGc2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc2(&self) -> TxIqPhaseCompGc2R {
        TxIqPhaseCompGc2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc2(&self) -> TxIqGainCompGc2R {
        TxIqGainCompGc2R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc2(&mut self) -> TxIqPhaseCompGc2W<'_, TxIqGainHw2Spec> {
        TxIqPhaseCompGc2W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc2(&mut self) -> TxIqGainCompGc2W<'_, TxIqGainHw2Spec> {
        TxIqGainCompGc2W::new(self, 16)
    }
}
#[doc = "tx_iq_gain_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxIqGainHw2Spec;
impl crate::RegisterSpec for TxIqGainHw2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_iq_gain_hw2::R`](R) reader structure"]
impl crate::Readable for TxIqGainHw2Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_iq_gain_hw2::W`](W) writer structure"]
impl crate::Writable for TxIqGainHw2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tx_iq_gain_hw2 to value 0"]
impl crate::Resettable for TxIqGainHw2Spec {}

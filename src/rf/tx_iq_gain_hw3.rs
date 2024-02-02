#[doc = "Register `tx_iq_gain_hw3` reader"]
pub type R = crate::R<TxIqGainHw3Spec>;
#[doc = "Register `tx_iq_gain_hw3` writer"]
pub type W = crate::W<TxIqGainHw3Spec>;
#[doc = "Field `tx_iq_phase_comp_gc3` reader - "]
pub type TxIqPhaseCompGc3R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_phase_comp_gc3` writer - "]
pub type TxIqPhaseCompGc3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tx_iq_gain_comp_gc3` reader - "]
pub type TxIqGainCompGc3R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_gain_comp_gc3` writer - "]
pub type TxIqGainCompGc3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc3(&self) -> TxIqPhaseCompGc3R {
        TxIqPhaseCompGc3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc3(&self) -> TxIqGainCompGc3R {
        TxIqGainCompGc3R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc3(&mut self) -> TxIqPhaseCompGc3W<'_, TxIqGainHw3Spec> {
        TxIqPhaseCompGc3W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc3(&mut self) -> TxIqGainCompGc3W<'_, TxIqGainHw3Spec> {
        TxIqGainCompGc3W::new(self, 16)
    }
}
#[doc = "tx_iq_gain_hw3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxIqGainHw3Spec;
impl crate::RegisterSpec for TxIqGainHw3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_iq_gain_hw3::R`](R) reader structure"]
impl crate::Readable for TxIqGainHw3Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_iq_gain_hw3::W`](W) writer structure"]
impl crate::Writable for TxIqGainHw3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tx_iq_gain_hw3 to value 0"]
impl crate::Resettable for TxIqGainHw3Spec {}

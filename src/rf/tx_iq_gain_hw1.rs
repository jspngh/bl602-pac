#[doc = "Register `tx_iq_gain_hw1` reader"]
pub type R = crate::R<TxIqGainHw1Spec>;
#[doc = "Register `tx_iq_gain_hw1` writer"]
pub type W = crate::W<TxIqGainHw1Spec>;
#[doc = "Field `tx_iq_phase_comp_gc1` reader - "]
pub type TxIqPhaseCompGc1R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_phase_comp_gc1` writer - "]
pub type TxIqPhaseCompGc1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tx_iq_gain_comp_gc1` reader - "]
pub type TxIqGainCompGc1R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_gain_comp_gc1` writer - "]
pub type TxIqGainCompGc1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc1(&self) -> TxIqPhaseCompGc1R {
        TxIqPhaseCompGc1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc1(&self) -> TxIqGainCompGc1R {
        TxIqGainCompGc1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc1(&mut self) -> TxIqPhaseCompGc1W<'_, TxIqGainHw1Spec> {
        TxIqPhaseCompGc1W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc1(&mut self) -> TxIqGainCompGc1W<'_, TxIqGainHw1Spec> {
        TxIqGainCompGc1W::new(self, 16)
    }
}
#[doc = "tx_iq_gain_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxIqGainHw1Spec;
impl crate::RegisterSpec for TxIqGainHw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_iq_gain_hw1::R`](R) reader structure"]
impl crate::Readable for TxIqGainHw1Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_iq_gain_hw1::W`](W) writer structure"]
impl crate::Writable for TxIqGainHw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tx_iq_gain_hw1 to value 0"]
impl crate::Resettable for TxIqGainHw1Spec {}

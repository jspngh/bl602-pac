#[doc = "Register `tx_iq_gain_hw6` reader"]
pub type R = crate::R<TxIqGainHw6Spec>;
#[doc = "Register `tx_iq_gain_hw6` writer"]
pub type W = crate::W<TxIqGainHw6Spec>;
#[doc = "Field `tx_iq_phase_comp_gc6` reader - "]
pub type TxIqPhaseCompGc6R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_phase_comp_gc6` writer - "]
pub type TxIqPhaseCompGc6W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tx_iq_gain_comp_gc6` reader - "]
pub type TxIqGainCompGc6R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_gain_comp_gc6` writer - "]
pub type TxIqGainCompGc6W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc6(&self) -> TxIqPhaseCompGc6R {
        TxIqPhaseCompGc6R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc6(&self) -> TxIqGainCompGc6R {
        TxIqGainCompGc6R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc6(&mut self) -> TxIqPhaseCompGc6W<'_, TxIqGainHw6Spec> {
        TxIqPhaseCompGc6W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc6(&mut self) -> TxIqGainCompGc6W<'_, TxIqGainHw6Spec> {
        TxIqGainCompGc6W::new(self, 16)
    }
}
#[doc = "tx_iq_gain_hw6.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxIqGainHw6Spec;
impl crate::RegisterSpec for TxIqGainHw6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_iq_gain_hw6::R`](R) reader structure"]
impl crate::Readable for TxIqGainHw6Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_iq_gain_hw6::W`](W) writer structure"]
impl crate::Writable for TxIqGainHw6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tx_iq_gain_hw6 to value 0"]
impl crate::Resettable for TxIqGainHw6Spec {}

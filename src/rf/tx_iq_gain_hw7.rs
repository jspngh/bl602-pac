#[doc = "Register `tx_iq_gain_hw7` reader"]
pub type R = crate::R<TxIqGainHw7Spec>;
#[doc = "Register `tx_iq_gain_hw7` writer"]
pub type W = crate::W<TxIqGainHw7Spec>;
#[doc = "Field `tx_iq_phase_comp_gc7` reader - "]
pub type TxIqPhaseCompGc7R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_phase_comp_gc7` writer - "]
pub type TxIqPhaseCompGc7W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tx_iq_gain_comp_gc7` reader - "]
pub type TxIqGainCompGc7R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_gain_comp_gc7` writer - "]
pub type TxIqGainCompGc7W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc7(&self) -> TxIqPhaseCompGc7R {
        TxIqPhaseCompGc7R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc7(&self) -> TxIqGainCompGc7R {
        TxIqGainCompGc7R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc7(&mut self) -> TxIqPhaseCompGc7W<'_, TxIqGainHw7Spec> {
        TxIqPhaseCompGc7W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc7(&mut self) -> TxIqGainCompGc7W<'_, TxIqGainHw7Spec> {
        TxIqGainCompGc7W::new(self, 16)
    }
}
#[doc = "tx_iq_gain_hw7.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxIqGainHw7Spec;
impl crate::RegisterSpec for TxIqGainHw7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_iq_gain_hw7::R`](R) reader structure"]
impl crate::Readable for TxIqGainHw7Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_iq_gain_hw7::W`](W) writer structure"]
impl crate::Writable for TxIqGainHw7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tx_iq_gain_hw7 to value 0"]
impl crate::Resettable for TxIqGainHw7Spec {}

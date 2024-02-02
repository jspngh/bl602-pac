#[doc = "Register `tx_iq_gain_hw4` reader"]
pub type R = crate::R<TxIqGainHw4Spec>;
#[doc = "Register `tx_iq_gain_hw4` writer"]
pub type W = crate::W<TxIqGainHw4Spec>;
#[doc = "Field `tx_iq_phase_comp_gc4` reader - "]
pub type TxIqPhaseCompGc4R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_phase_comp_gc4` writer - "]
pub type TxIqPhaseCompGc4W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tx_iq_gain_comp_gc4` reader - "]
pub type TxIqGainCompGc4R = crate::FieldReader<u16>;
#[doc = "Field `tx_iq_gain_comp_gc4` writer - "]
pub type TxIqGainCompGc4W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc4(&self) -> TxIqPhaseCompGc4R {
        TxIqPhaseCompGc4R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc4(&self) -> TxIqGainCompGc4R {
        TxIqGainCompGc4R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc4(&mut self) -> TxIqPhaseCompGc4W<'_, TxIqGainHw4Spec> {
        TxIqPhaseCompGc4W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc4(&mut self) -> TxIqGainCompGc4W<'_, TxIqGainHw4Spec> {
        TxIqGainCompGc4W::new(self, 16)
    }
}
#[doc = "tx_iq_gain_hw4.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxIqGainHw4Spec;
impl crate::RegisterSpec for TxIqGainHw4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_iq_gain_hw4::R`](R) reader structure"]
impl crate::Readable for TxIqGainHw4Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_iq_gain_hw4::W`](W) writer structure"]
impl crate::Writable for TxIqGainHw4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tx_iq_gain_hw4 to value 0"]
impl crate::Resettable for TxIqGainHw4Spec {}

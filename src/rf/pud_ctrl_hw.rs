#[doc = "Register `pud_ctrl_hw` reader"]
pub type R = crate::R<PudCtrlHwSpec>;
#[doc = "Register `pud_ctrl_hw` writer"]
pub type W = crate::W<PudCtrlHwSpec>;
#[doc = "Field `pud_vco_hw` reader - "]
pub type PudVcoHwR = crate::BitReader;
#[doc = "Field `pud_vco_hw` writer - "]
pub type PudVcoHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pud_vco_hw(&self) -> PudVcoHwR {
        PudVcoHwR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pud_vco_hw(&mut self) -> PudVcoHwW<'_, PudCtrlHwSpec> {
        PudVcoHwW::new(self, 20)
    }
}
#[doc = "pud_ctrl_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`pud_ctrl_hw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pud_ctrl_hw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PudCtrlHwSpec;
impl crate::RegisterSpec for PudCtrlHwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pud_ctrl_hw::R`](R) reader structure"]
impl crate::Readable for PudCtrlHwSpec {}
#[doc = "`write(|w| ..)` method takes [`pud_ctrl_hw::W`](W) writer structure"]
impl crate::Writable for PudCtrlHwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pud_ctrl_hw to value 0"]
impl crate::Resettable for PudCtrlHwSpec {}

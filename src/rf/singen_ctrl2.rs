#[doc = "Register `singen_ctrl2` reader"]
pub type R = crate::R<SingenCtrl2Spec>;
#[doc = "Register `singen_ctrl2` writer"]
pub type W = crate::W<SingenCtrl2Spec>;
#[doc = "Field `singen_gain_i` reader - "]
pub type SingenGainIR = crate::FieldReader<u16>;
#[doc = "Field `singen_gain_i` writer - "]
pub type SingenGainIW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `singen_start_addr1_i` reader - "]
pub type SingenStartAddr1IR = crate::FieldReader<u16>;
#[doc = "Field `singen_start_addr1_i` writer - "]
pub type SingenStartAddr1IW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `singen_start_addr0_i` reader - "]
pub type SingenStartAddr0IR = crate::FieldReader<u16>;
#[doc = "Field `singen_start_addr0_i` writer - "]
pub type SingenStartAddr0IW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn singen_gain_i(&self) -> SingenGainIR {
        SingenGainIR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn singen_start_addr1_i(&self) -> SingenStartAddr1IR {
        SingenStartAddr1IR::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn singen_start_addr0_i(&self) -> SingenStartAddr0IR {
        SingenStartAddr0IR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn singen_gain_i(&mut self) -> SingenGainIW<'_, SingenCtrl2Spec> {
        SingenGainIW::new(self, 0)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn singen_start_addr1_i(&mut self) -> SingenStartAddr1IW<'_, SingenCtrl2Spec> {
        SingenStartAddr1IW::new(self, 12)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn singen_start_addr0_i(&mut self) -> SingenStartAddr0IW<'_, SingenCtrl2Spec> {
        SingenStartAddr0IW::new(self, 22)
    }
}
#[doc = "singen_ctrl2.\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SingenCtrl2Spec;
impl crate::RegisterSpec for SingenCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singen_ctrl2::R`](R) reader structure"]
impl crate::Readable for SingenCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`singen_ctrl2::W`](W) writer structure"]
impl crate::Writable for SingenCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets singen_ctrl2 to value 0"]
impl crate::Resettable for SingenCtrl2Spec {}

#[doc = "Register `singen_ctrl3` reader"]
pub type R = crate::R<SingenCtrl3Spec>;
#[doc = "Register `singen_ctrl3` writer"]
pub type W = crate::W<SingenCtrl3Spec>;
#[doc = "Field `singen_gain_q` reader - "]
pub type SingenGainQR = crate::FieldReader<u16>;
#[doc = "Field `singen_gain_q` writer - "]
pub type SingenGainQW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `singen_start_addr1_q` reader - "]
pub type SingenStartAddr1QR = crate::FieldReader<u16>;
#[doc = "Field `singen_start_addr1_q` writer - "]
pub type SingenStartAddr1QW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `singen_start_addr0_q` reader - "]
pub type SingenStartAddr0QR = crate::FieldReader<u16>;
#[doc = "Field `singen_start_addr0_q` writer - "]
pub type SingenStartAddr0QW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn singen_gain_q(&self) -> SingenGainQR {
        SingenGainQR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn singen_start_addr1_q(&self) -> SingenStartAddr1QR {
        SingenStartAddr1QR::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn singen_start_addr0_q(&self) -> SingenStartAddr0QR {
        SingenStartAddr0QR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn singen_gain_q(&mut self) -> SingenGainQW<'_, SingenCtrl3Spec> {
        SingenGainQW::new(self, 0)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn singen_start_addr1_q(&mut self) -> SingenStartAddr1QW<'_, SingenCtrl3Spec> {
        SingenStartAddr1QW::new(self, 12)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn singen_start_addr0_q(&mut self) -> SingenStartAddr0QW<'_, SingenCtrl3Spec> {
        SingenStartAddr0QW::new(self, 22)
    }
}
#[doc = "singen_ctrl3.\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SingenCtrl3Spec;
impl crate::RegisterSpec for SingenCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singen_ctrl3::R`](R) reader structure"]
impl crate::Readable for SingenCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`singen_ctrl3::W`](W) writer structure"]
impl crate::Writable for SingenCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets singen_ctrl3 to value 0"]
impl crate::Resettable for SingenCtrl3Spec {}

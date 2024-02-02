#[doc = "Register `sdm3` reader"]
pub type R = crate::R<Sdm3Spec>;
#[doc = "Register `sdm3` writer"]
pub type W = crate::W<Sdm3Spec>;
#[doc = "Field `lo_sdmin_hw` reader - "]
pub type LoSdminHwR = crate::FieldReader<u32>;
#[doc = "Field `lo_sdmin_hw` writer - "]
pub type LoSdminHwW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin_hw(&self) -> LoSdminHwR {
        LoSdminHwR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin_hw(&mut self) -> LoSdminHwW<'_, Sdm3Spec> {
        LoSdminHwW::new(self, 0)
    }
}
#[doc = "sdm3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdm3Spec;
impl crate::RegisterSpec for Sdm3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm3::R`](R) reader structure"]
impl crate::Readable for Sdm3Spec {}
#[doc = "`write(|w| ..)` method takes [`sdm3::W`](W) writer structure"]
impl crate::Writable for Sdm3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sdm3 to value 0"]
impl crate::Resettable for Sdm3Spec {}

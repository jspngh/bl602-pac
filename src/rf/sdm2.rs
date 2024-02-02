#[doc = "Register `sdm2` reader"]
pub type R = crate::R<Sdm2Spec>;
#[doc = "Register `sdm2` writer"]
pub type W = crate::W<Sdm2Spec>;
#[doc = "Field `lo_sdmin` reader - "]
pub type LoSdminR = crate::FieldReader<u32>;
#[doc = "Field `lo_sdmin` writer - "]
pub type LoSdminW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin(&self) -> LoSdminR {
        LoSdminR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin(&mut self) -> LoSdminW<'_, Sdm2Spec> {
        LoSdminW::new(self, 0)
    }
}
#[doc = "sdm2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdm2Spec;
impl crate::RegisterSpec for Sdm2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm2::R`](R) reader structure"]
impl crate::Readable for Sdm2Spec {}
#[doc = "`write(|w| ..)` method takes [`sdm2::W`](W) writer structure"]
impl crate::Writable for Sdm2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sdm2 to value 0"]
impl crate::Resettable for Sdm2Spec {}

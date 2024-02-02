#[doc = "Register `cgen_cfg0` reader"]
pub type R = crate::R<CgenCfg0Spec>;
#[doc = "Register `cgen_cfg0` writer"]
pub type W = crate::W<CgenCfg0Spec>;
#[doc = "Field `cgen_m` reader - "]
pub type CgenMR = crate::FieldReader;
#[doc = "Field `cgen_m` writer - "]
pub type CgenMW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cgen_m(&self) -> CgenMR {
        CgenMR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cgen_m(&mut self) -> CgenMW<'_, CgenCfg0Spec> {
        CgenMW::new(self, 0)
    }
}
#[doc = "cgen_cfg0.\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgenCfg0Spec;
impl crate::RegisterSpec for CgenCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgen_cfg0::R`](R) reader structure"]
impl crate::Readable for CgenCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cgen_cfg0::W`](W) writer structure"]
impl crate::Writable for CgenCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cgen_cfg0 to value 0xff"]
impl crate::Resettable for CgenCfg0Spec {
    const RESET_VALUE: u32 = 0xff;
}

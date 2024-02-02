#[doc = "Register `GPIO_CFGCTL33` reader"]
pub type R = crate::R<GpioCfgctl33Spec>;
#[doc = "Register `GPIO_CFGCTL33` writer"]
pub type W = crate::W<GpioCfgctl33Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reserved according to SDK.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl33Spec;
impl crate::RegisterSpec for GpioCfgctl33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl33::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl33Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl33::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl33Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL33 to value 0"]
impl crate::Resettable for GpioCfgctl33Spec {}

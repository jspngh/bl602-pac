#[doc = "Register `WMR` reader"]
pub type R = crate::R<WmrSpec>;
#[doc = "Register `WMR` writer"]
pub type W = crate::W<WmrSpec>;
#[doc = "Field `wmr` reader - "]
pub type WmrR = crate::FieldReader<u16>;
#[doc = "Field `wmr` writer - "]
pub type WmrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wmr(&self) -> WmrR {
        WmrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wmr(&mut self) -> WmrW<'_, WmrSpec> {
        WmrW::new(self, 0)
    }
}
#[doc = "WMR.\n\nYou can [`read`](crate::Reg::read) this register and get [`wmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WmrSpec;
impl crate::RegisterSpec for WmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wmr::R`](R) reader structure"]
impl crate::Readable for WmrSpec {}
#[doc = "`write(|w| ..)` method takes [`wmr::W`](W) writer structure"]
impl crate::Writable for WmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WMR to value 0xffff"]
impl crate::Resettable for WmrSpec {
    const RESET_VALUE: u32 = 0xffff;
}

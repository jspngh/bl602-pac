#[doc = "Register `swrst_cfg0` reader"]
pub type R = crate::R<SwrstCfg0Spec>;
#[doc = "Register `swrst_cfg0` writer"]
pub type W = crate::W<SwrstCfg0Spec>;
#[doc = "Field `swrst_s00` reader - "]
pub type SwrstS00R = crate::BitReader;
#[doc = "Field `swrst_s00` writer - "]
pub type SwrstS00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s01` reader - "]
pub type SwrstS01R = crate::BitReader;
#[doc = "Field `swrst_s01` writer - "]
pub type SwrstS01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s20` reader - "]
pub type SwrstS20R = crate::BitReader;
#[doc = "Field `swrst_s20` writer - "]
pub type SwrstS20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s30` reader - "]
pub type SwrstS30R = crate::BitReader;
#[doc = "Field `swrst_s30` writer - "]
pub type SwrstS30W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s00(&self) -> SwrstS00R {
        SwrstS00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s01(&self) -> SwrstS01R {
        SwrstS01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s20(&self) -> SwrstS20R {
        SwrstS20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s30(&self) -> SwrstS30R {
        SwrstS30R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s00(&mut self) -> SwrstS00W<'_, SwrstCfg0Spec> {
        SwrstS00W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s01(&mut self) -> SwrstS01W<'_, SwrstCfg0Spec> {
        SwrstS01W::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s20(&mut self) -> SwrstS20W<'_, SwrstCfg0Spec> {
        SwrstS20W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s30(&mut self) -> SwrstS30W<'_, SwrstCfg0Spec> {
        SwrstS30W::new(self, 8)
    }
}
#[doc = "swrst_cfg0.\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstCfg0Spec;
impl crate::RegisterSpec for SwrstCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_cfg0::R`](R) reader structure"]
impl crate::Readable for SwrstCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`swrst_cfg0::W`](W) writer structure"]
impl crate::Writable for SwrstCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets swrst_cfg0 to value 0"]
impl crate::Resettable for SwrstCfg0Spec {}

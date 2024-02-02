#[doc = "Register `singen_ctrl0` reader"]
pub type R = crate::R<SingenCtrl0Spec>;
#[doc = "Register `singen_ctrl0` writer"]
pub type W = crate::W<SingenCtrl0Spec>;
#[doc = "Field `singen_inc_step1` reader - "]
pub type SingenIncStep1R = crate::FieldReader<u16>;
#[doc = "Field `singen_inc_step1` writer - "]
pub type SingenIncStep1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `singen_inc_step0` reader - "]
pub type SingenIncStep0R = crate::FieldReader<u16>;
#[doc = "Field `singen_inc_step0` writer - "]
pub type SingenIncStep0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `singen_unsign_en` reader - "]
pub type SingenUnsignEnR = crate::BitReader;
#[doc = "Field `singen_unsign_en` writer - "]
pub type SingenUnsignEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `singen_clkdiv_n` reader - "]
pub type SingenClkdivNR = crate::FieldReader;
#[doc = "Field `singen_clkdiv_n` writer - "]
pub type SingenClkdivNW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `singen_en` reader - "]
pub type SingenEnR = crate::BitReader;
#[doc = "Field `singen_en` writer - "]
pub type SingenEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_inc_step1(&self) -> SingenIncStep1R {
        SingenIncStep1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_inc_step0(&self) -> SingenIncStep0R {
        SingenIncStep0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_unsign_en(&self) -> SingenUnsignEnR {
        SingenUnsignEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn singen_clkdiv_n(&self) -> SingenClkdivNR {
        SingenClkdivNR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn singen_en(&self) -> SingenEnR {
        SingenEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_inc_step1(&mut self) -> SingenIncStep1W<'_, SingenCtrl0Spec> {
        SingenIncStep1W::new(self, 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_inc_step0(&mut self) -> SingenIncStep0W<'_, SingenCtrl0Spec> {
        SingenIncStep0W::new(self, 16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_unsign_en(&mut self) -> SingenUnsignEnW<'_, SingenCtrl0Spec> {
        SingenUnsignEnW::new(self, 28)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn singen_clkdiv_n(&mut self) -> SingenClkdivNW<'_, SingenCtrl0Spec> {
        SingenClkdivNW::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn singen_en(&mut self) -> SingenEnW<'_, SingenCtrl0Spec> {
        SingenEnW::new(self, 31)
    }
}
#[doc = "singen_ctrl0.\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SingenCtrl0Spec;
impl crate::RegisterSpec for SingenCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singen_ctrl0::R`](R) reader structure"]
impl crate::Readable for SingenCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`singen_ctrl0::W`](W) writer structure"]
impl crate::Writable for SingenCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets singen_ctrl0 to value 0"]
impl crate::Resettable for SingenCtrl0Spec {}

#[doc = "Register `singen_ctrl1` reader"]
pub type R = crate::R<SingenCtrl1Spec>;
#[doc = "Register `singen_ctrl1` writer"]
pub type W = crate::W<SingenCtrl1Spec>;
#[doc = "Field `singen_clkdiv_q` reader - "]
pub type SingenClkdivQR = crate::FieldReader<u16>;
#[doc = "Field `singen_clkdiv_q` writer - "]
pub type SingenClkdivQW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `singen_mode_q` reader - "]
pub type SingenModeQR = crate::FieldReader;
#[doc = "Field `singen_mode_q` writer - "]
pub type SingenModeQW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `singen_clkdiv_i` reader - "]
pub type SingenClkdivIR = crate::FieldReader<u16>;
#[doc = "Field `singen_clkdiv_i` writer - "]
pub type SingenClkdivIW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `singen_mode_i` reader - "]
pub type SingenModeIR = crate::FieldReader;
#[doc = "Field `singen_mode_i` writer - "]
pub type SingenModeIW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_clkdiv_q(&self) -> SingenClkdivQR {
        SingenClkdivQR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn singen_mode_q(&self) -> SingenModeQR {
        SingenModeQR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_clkdiv_i(&self) -> SingenClkdivIR {
        SingenClkdivIR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn singen_mode_i(&self) -> SingenModeIR {
        SingenModeIR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_clkdiv_q(&mut self) -> SingenClkdivQW<'_, SingenCtrl1Spec> {
        SingenClkdivQW::new(self, 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn singen_mode_q(&mut self) -> SingenModeQW<'_, SingenCtrl1Spec> {
        SingenModeQW::new(self, 12)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_clkdiv_i(&mut self) -> SingenClkdivIW<'_, SingenCtrl1Spec> {
        SingenClkdivIW::new(self, 16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn singen_mode_i(&mut self) -> SingenModeIW<'_, SingenCtrl1Spec> {
        SingenModeIW::new(self, 28)
    }
}
#[doc = "singen_ctrl1.\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SingenCtrl1Spec;
impl crate::RegisterSpec for SingenCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singen_ctrl1::R`](R) reader structure"]
impl crate::Readable for SingenCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`singen_ctrl1::W`](W) writer structure"]
impl crate::Writable for SingenCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets singen_ctrl1 to value 0"]
impl crate::Resettable for SingenCtrl1Spec {}

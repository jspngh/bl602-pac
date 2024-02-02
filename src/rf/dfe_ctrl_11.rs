#[doc = "Register `dfe_ctrl_11` reader"]
pub type R = crate::R<DfeCtrl11Spec>;
#[doc = "Register `dfe_ctrl_11` writer"]
pub type W = crate::W<DfeCtrl11Spec>;
#[doc = "Field `dfe_adc_raw_i` reader - "]
pub type DfeAdcRawIR = crate::FieldReader<u16>;
#[doc = "Field `dfe_adc_raw_i` writer - "]
pub type DfeAdcRawIW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `dfe_adc_raw_q` reader - "]
pub type DfeAdcRawQR = crate::FieldReader<u16>;
#[doc = "Field `dfe_adc_raw_q` writer - "]
pub type DfeAdcRawQW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn dfe_adc_raw_i(&self) -> DfeAdcRawIR {
        DfeAdcRawIR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn dfe_adc_raw_q(&self) -> DfeAdcRawQR {
        DfeAdcRawQR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn dfe_adc_raw_i(&mut self) -> DfeAdcRawIW<'_, DfeCtrl11Spec> {
        DfeAdcRawIW::new(self, 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn dfe_adc_raw_q(&mut self) -> DfeAdcRawQW<'_, DfeCtrl11Spec> {
        DfeAdcRawQW::new(self, 16)
    }
}
#[doc = "dfe_ctrl_11.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl11Spec;
impl crate::RegisterSpec for DfeCtrl11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_11::R`](R) reader structure"]
impl crate::Readable for DfeCtrl11Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_11::W`](W) writer structure"]
impl crate::Writable for DfeCtrl11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_11 to value 0"]
impl crate::Resettable for DfeCtrl11Spec {}

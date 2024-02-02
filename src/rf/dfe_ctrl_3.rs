#[doc = "Register `dfe_ctrl_3` reader"]
pub type R = crate::R<DfeCtrl3Spec>;
#[doc = "Register `dfe_ctrl_3` writer"]
pub type W = crate::W<DfeCtrl3Spec>;
#[doc = "Field `rx_adc_4s_i_val` reader - "]
pub type RxAdc4sIValR = crate::FieldReader<u16>;
#[doc = "Field `rx_adc_4s_i_val` writer - "]
pub type RxAdc4sIValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rx_adc_4s_i_en` reader - "]
pub type RxAdc4sIEnR = crate::BitReader;
#[doc = "Field `rx_adc_4s_i_en` writer - "]
pub type RxAdc4sIEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_adc_4s_q_val` reader - "]
pub type RxAdc4sQValR = crate::FieldReader<u16>;
#[doc = "Field `rx_adc_4s_q_val` writer - "]
pub type RxAdc4sQValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rx_adc_4s_q_en` reader - "]
pub type RxAdc4sQEnR = crate::BitReader;
#[doc = "Field `rx_adc_4s_q_en` writer - "]
pub type RxAdc4sQEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_4s_i_val(&self) -> RxAdc4sIValR {
        RxAdc4sIValR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_adc_4s_i_en(&self) -> RxAdc4sIEnR {
        RxAdc4sIEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_4s_q_val(&self) -> RxAdc4sQValR {
        RxAdc4sQValR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_adc_4s_q_en(&self) -> RxAdc4sQEnR {
        RxAdc4sQEnR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_4s_i_val(&mut self) -> RxAdc4sIValW<'_, DfeCtrl3Spec> {
        RxAdc4sIValW::new(self, 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_adc_4s_i_en(&mut self) -> RxAdc4sIEnW<'_, DfeCtrl3Spec> {
        RxAdc4sIEnW::new(self, 10)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_4s_q_val(&mut self) -> RxAdc4sQValW<'_, DfeCtrl3Spec> {
        RxAdc4sQValW::new(self, 16)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_adc_4s_q_en(&mut self) -> RxAdc4sQEnW<'_, DfeCtrl3Spec> {
        RxAdc4sQEnW::new(self, 26)
    }
}
#[doc = "dfe_ctrl_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl3Spec;
impl crate::RegisterSpec for DfeCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_3::R`](R) reader structure"]
impl crate::Readable for DfeCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_3::W`](W) writer structure"]
impl crate::Writable for DfeCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_3 to value 0"]
impl crate::Resettable for DfeCtrl3Spec {}

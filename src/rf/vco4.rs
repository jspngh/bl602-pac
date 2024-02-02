#[doc = "Register `vco4` reader"]
pub type R = crate::R<Vco4Spec>;
#[doc = "Register `vco4` writer"]
pub type W = crate::W<Vco4Spec>;
#[doc = "Field `fcal_cnt_start` reader - "]
pub type FcalCntStartR = crate::BitReader;
#[doc = "Field `fcal_cnt_start` writer - "]
pub type FcalCntStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fcal_inc_en_hw` reader - "]
pub type FcalIncEnHwR = crate::BitReader;
#[doc = "Field `fcal_inc_en_hw` writer - "]
pub type FcalIncEnHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fcal_inc_large_range` reader - "]
pub type FcalIncLargeRangeR = crate::BitReader;
#[doc = "Field `fcal_inc_large_range` writer - "]
pub type FcalIncLargeRangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fcal_cnt_rdy` reader - "]
pub type FcalCntRdyR = crate::BitReader;
#[doc = "Field `fcal_cnt_rdy` writer - "]
pub type FcalCntRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fcal_inc_vctrl_ud` reader - "]
pub type FcalIncVctrlUdR = crate::FieldReader;
#[doc = "Field `fcal_inc_vctrl_ud` writer - "]
pub type FcalIncVctrlUdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fcal_cnt_start(&self) -> FcalCntStartR {
        FcalCntStartR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcal_inc_en_hw(&self) -> FcalIncEnHwR {
        FcalIncEnHwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fcal_inc_large_range(&self) -> FcalIncLargeRangeR {
        FcalIncLargeRangeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fcal_cnt_rdy(&self) -> FcalCntRdyR {
        FcalCntRdyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn fcal_inc_vctrl_ud(&self) -> FcalIncVctrlUdR {
        FcalIncVctrlUdR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fcal_cnt_start(&mut self) -> FcalCntStartW<'_, Vco4Spec> {
        FcalCntStartW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcal_inc_en_hw(&mut self) -> FcalIncEnHwW<'_, Vco4Spec> {
        FcalIncEnHwW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fcal_inc_large_range(&mut self) -> FcalIncLargeRangeW<'_, Vco4Spec> {
        FcalIncLargeRangeW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fcal_cnt_rdy(&mut self) -> FcalCntRdyW<'_, Vco4Spec> {
        FcalCntRdyW::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn fcal_inc_vctrl_ud(&mut self) -> FcalIncVctrlUdW<'_, Vco4Spec> {
        FcalIncVctrlUdW::new(self, 24)
    }
}
#[doc = "vco4.\n\nYou can [`read`](crate::Reg::read) this register and get [`vco4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vco4Spec;
impl crate::RegisterSpec for Vco4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vco4::R`](R) reader structure"]
impl crate::Readable for Vco4Spec {}
#[doc = "`write(|w| ..)` method takes [`vco4::W`](W) writer structure"]
impl crate::Writable for Vco4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets vco4 to value 0"]
impl crate::Resettable for Vco4Spec {}

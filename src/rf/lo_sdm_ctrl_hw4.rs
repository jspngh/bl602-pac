#[doc = "Register `lo_sdm_ctrl_hw4` reader"]
pub type R = crate::R<LoSdmCtrlHw4Spec>;
#[doc = "Register `lo_sdm_ctrl_hw4` writer"]
pub type W = crate::W<LoSdmCtrlHw4Spec>;
#[doc = "Field `lo_sdm_dither_sel_ble_2466` reader - "]
pub type LoSdmDitherSelBle2466R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_ble_2466` writer - "]
pub type LoSdmDitherSelBle2466W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_ble_2468` reader - "]
pub type LoSdmDitherSelBle2468R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_ble_2468` writer - "]
pub type LoSdmDitherSelBle2468W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_ble_2470` reader - "]
pub type LoSdmDitherSelBle2470R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_ble_2470` writer - "]
pub type LoSdmDitherSelBle2470W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_ble_2472` reader - "]
pub type LoSdmDitherSelBle2472R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_ble_2472` writer - "]
pub type LoSdmDitherSelBle2472W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_ble_2474` reader - "]
pub type LoSdmDitherSelBle2474R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_ble_2474` writer - "]
pub type LoSdmDitherSelBle2474W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_ble_2476` reader - "]
pub type LoSdmDitherSelBle2476R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_ble_2476` writer - "]
pub type LoSdmDitherSelBle2476W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_ble_2478` reader - "]
pub type LoSdmDitherSelBle2478R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_ble_2478` writer - "]
pub type LoSdmDitherSelBle2478W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_ble_2480` reader - "]
pub type LoSdmDitherSelBle2480R = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_ble_2480` writer - "]
pub type LoSdmDitherSelBle2480W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_sdm_dither_sel_ble_tx` reader - "]
pub type LoSdmDitherSelBleTxR = crate::FieldReader;
#[doc = "Field `lo_sdm_dither_sel_ble_tx` writer - "]
pub type LoSdmDitherSelBleTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2466(&self) -> LoSdmDitherSelBle2466R {
        LoSdmDitherSelBle2466R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2468(&self) -> LoSdmDitherSelBle2468R {
        LoSdmDitherSelBle2468R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2470(&self) -> LoSdmDitherSelBle2470R {
        LoSdmDitherSelBle2470R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2472(&self) -> LoSdmDitherSelBle2472R {
        LoSdmDitherSelBle2472R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2474(&self) -> LoSdmDitherSelBle2474R {
        LoSdmDitherSelBle2474R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2476(&self) -> LoSdmDitherSelBle2476R {
        LoSdmDitherSelBle2476R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2478(&self) -> LoSdmDitherSelBle2478R {
        LoSdmDitherSelBle2478R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2480(&self) -> LoSdmDitherSelBle2480R {
        LoSdmDitherSelBle2480R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_tx(&self) -> LoSdmDitherSelBleTxR {
        LoSdmDitherSelBleTxR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2466(&mut self) -> LoSdmDitherSelBle2466W<'_, LoSdmCtrlHw4Spec> {
        LoSdmDitherSelBle2466W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2468(&mut self) -> LoSdmDitherSelBle2468W<'_, LoSdmCtrlHw4Spec> {
        LoSdmDitherSelBle2468W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2470(&mut self) -> LoSdmDitherSelBle2470W<'_, LoSdmCtrlHw4Spec> {
        LoSdmDitherSelBle2470W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2472(&mut self) -> LoSdmDitherSelBle2472W<'_, LoSdmCtrlHw4Spec> {
        LoSdmDitherSelBle2472W::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2474(&mut self) -> LoSdmDitherSelBle2474W<'_, LoSdmCtrlHw4Spec> {
        LoSdmDitherSelBle2474W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2476(&mut self) -> LoSdmDitherSelBle2476W<'_, LoSdmCtrlHw4Spec> {
        LoSdmDitherSelBle2476W::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2478(&mut self) -> LoSdmDitherSelBle2478W<'_, LoSdmCtrlHw4Spec> {
        LoSdmDitherSelBle2478W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2480(&mut self) -> LoSdmDitherSelBle2480W<'_, LoSdmCtrlHw4Spec> {
        LoSdmDitherSelBle2480W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_tx(&mut self) -> LoSdmDitherSelBleTxW<'_, LoSdmCtrlHw4Spec> {
        LoSdmDitherSelBleTxW::new(self, 16)
    }
}
#[doc = "lo_sdm_ctrl_hw4.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoSdmCtrlHw4Spec;
impl crate::RegisterSpec for LoSdmCtrlHw4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_sdm_ctrl_hw4::R`](R) reader structure"]
impl crate::Readable for LoSdmCtrlHw4Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_sdm_ctrl_hw4::W`](W) writer structure"]
impl crate::Writable for LoSdmCtrlHw4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw4 to value 0"]
impl crate::Resettable for LoSdmCtrlHw4Spec {}

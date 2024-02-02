#[doc = "Register `tosdac_ctrl_hw2` reader"]
pub type R = crate::R<TosdacCtrlHw2Spec>;
#[doc = "Register `tosdac_ctrl_hw2` writer"]
pub type W = crate::W<TosdacCtrlHw2Spec>;
#[doc = "Field `tbb_tosdac_i_gc2` reader - "]
pub type TbbTosdacIGc2R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_i_gc2` writer - "]
pub type TbbTosdacIGc2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_q_gc2` reader - "]
pub type TbbTosdacQGc2R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_q_gc2` writer - "]
pub type TbbTosdacQGc2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_i_gc3` reader - "]
pub type TbbTosdacIGc3R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_i_gc3` writer - "]
pub type TbbTosdacIGc3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_q_gc3` reader - "]
pub type TbbTosdacQGc3R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_q_gc3` writer - "]
pub type TbbTosdacQGc3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc2(&self) -> TbbTosdacIGc2R {
        TbbTosdacIGc2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc2(&self) -> TbbTosdacQGc2R {
        TbbTosdacQGc2R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc3(&self) -> TbbTosdacIGc3R {
        TbbTosdacIGc3R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc3(&self) -> TbbTosdacQGc3R {
        TbbTosdacQGc3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc2(&mut self) -> TbbTosdacIGc2W<'_, TosdacCtrlHw2Spec> {
        TbbTosdacIGc2W::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc2(&mut self) -> TbbTosdacQGc2W<'_, TosdacCtrlHw2Spec> {
        TbbTosdacQGc2W::new(self, 8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc3(&mut self) -> TbbTosdacIGc3W<'_, TosdacCtrlHw2Spec> {
        TbbTosdacIGc3W::new(self, 16)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc3(&mut self) -> TbbTosdacQGc3W<'_, TosdacCtrlHw2Spec> {
        TbbTosdacQGc3W::new(self, 24)
    }
}
#[doc = "tosdac_ctrl_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tosdac_ctrl_hw2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tosdac_ctrl_hw2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TosdacCtrlHw2Spec;
impl crate::RegisterSpec for TosdacCtrlHw2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tosdac_ctrl_hw2::R`](R) reader structure"]
impl crate::Readable for TosdacCtrlHw2Spec {}
#[doc = "`write(|w| ..)` method takes [`tosdac_ctrl_hw2::W`](W) writer structure"]
impl crate::Writable for TosdacCtrlHw2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tosdac_ctrl_hw2 to value 0"]
impl crate::Resettable for TosdacCtrlHw2Spec {}

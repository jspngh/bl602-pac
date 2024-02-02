#[doc = "Register `tosdac_ctrl_hw3` reader"]
pub type R = crate::R<TosdacCtrlHw3Spec>;
#[doc = "Register `tosdac_ctrl_hw3` writer"]
pub type W = crate::W<TosdacCtrlHw3Spec>;
#[doc = "Field `tbb_tosdac_i_gc4` reader - "]
pub type TbbTosdacIGc4R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_i_gc4` writer - "]
pub type TbbTosdacIGc4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_q_gc4` reader - "]
pub type TbbTosdacQGc4R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_q_gc4` writer - "]
pub type TbbTosdacQGc4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_i_gc5` reader - "]
pub type TbbTosdacIGc5R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_i_gc5` writer - "]
pub type TbbTosdacIGc5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_q_gc5` reader - "]
pub type TbbTosdacQGc5R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_q_gc5` writer - "]
pub type TbbTosdacQGc5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc4(&self) -> TbbTosdacIGc4R {
        TbbTosdacIGc4R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc4(&self) -> TbbTosdacQGc4R {
        TbbTosdacQGc4R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc5(&self) -> TbbTosdacIGc5R {
        TbbTosdacIGc5R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc5(&self) -> TbbTosdacQGc5R {
        TbbTosdacQGc5R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc4(&mut self) -> TbbTosdacIGc4W<'_, TosdacCtrlHw3Spec> {
        TbbTosdacIGc4W::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc4(&mut self) -> TbbTosdacQGc4W<'_, TosdacCtrlHw3Spec> {
        TbbTosdacQGc4W::new(self, 8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc5(&mut self) -> TbbTosdacIGc5W<'_, TosdacCtrlHw3Spec> {
        TbbTosdacIGc5W::new(self, 16)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc5(&mut self) -> TbbTosdacQGc5W<'_, TosdacCtrlHw3Spec> {
        TbbTosdacQGc5W::new(self, 24)
    }
}
#[doc = "tosdac_ctrl_hw3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tosdac_ctrl_hw3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tosdac_ctrl_hw3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TosdacCtrlHw3Spec;
impl crate::RegisterSpec for TosdacCtrlHw3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tosdac_ctrl_hw3::R`](R) reader structure"]
impl crate::Readable for TosdacCtrlHw3Spec {}
#[doc = "`write(|w| ..)` method takes [`tosdac_ctrl_hw3::W`](W) writer structure"]
impl crate::Writable for TosdacCtrlHw3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tosdac_ctrl_hw3 to value 0"]
impl crate::Resettable for TosdacCtrlHw3Spec {}

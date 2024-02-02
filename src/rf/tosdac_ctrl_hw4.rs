#[doc = "Register `tosdac_ctrl_hw4` reader"]
pub type R = crate::R<TosdacCtrlHw4Spec>;
#[doc = "Register `tosdac_ctrl_hw4` writer"]
pub type W = crate::W<TosdacCtrlHw4Spec>;
#[doc = "Field `tbb_tosdac_i_gc6` reader - "]
pub type TbbTosdacIGc6R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_i_gc6` writer - "]
pub type TbbTosdacIGc6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_q_gc6` reader - "]
pub type TbbTosdacQGc6R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_q_gc6` writer - "]
pub type TbbTosdacQGc6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_i_gc7` reader - "]
pub type TbbTosdacIGc7R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_i_gc7` writer - "]
pub type TbbTosdacIGc7W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_q_gc7` reader - "]
pub type TbbTosdacQGc7R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_q_gc7` writer - "]
pub type TbbTosdacQGc7W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc6(&self) -> TbbTosdacIGc6R {
        TbbTosdacIGc6R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc6(&self) -> TbbTosdacQGc6R {
        TbbTosdacQGc6R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc7(&self) -> TbbTosdacIGc7R {
        TbbTosdacIGc7R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc7(&self) -> TbbTosdacQGc7R {
        TbbTosdacQGc7R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc6(&mut self) -> TbbTosdacIGc6W<'_, TosdacCtrlHw4Spec> {
        TbbTosdacIGc6W::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc6(&mut self) -> TbbTosdacQGc6W<'_, TosdacCtrlHw4Spec> {
        TbbTosdacQGc6W::new(self, 8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc7(&mut self) -> TbbTosdacIGc7W<'_, TosdacCtrlHw4Spec> {
        TbbTosdacIGc7W::new(self, 16)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc7(&mut self) -> TbbTosdacQGc7W<'_, TosdacCtrlHw4Spec> {
        TbbTosdacQGc7W::new(self, 24)
    }
}
#[doc = "tosdac_ctrl_hw4.\n\nYou can [`read`](crate::Reg::read) this register and get [`tosdac_ctrl_hw4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tosdac_ctrl_hw4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TosdacCtrlHw4Spec;
impl crate::RegisterSpec for TosdacCtrlHw4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tosdac_ctrl_hw4::R`](R) reader structure"]
impl crate::Readable for TosdacCtrlHw4Spec {}
#[doc = "`write(|w| ..)` method takes [`tosdac_ctrl_hw4::W`](W) writer structure"]
impl crate::Writable for TosdacCtrlHw4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tosdac_ctrl_hw4 to value 0"]
impl crate::Resettable for TosdacCtrlHw4Spec {}

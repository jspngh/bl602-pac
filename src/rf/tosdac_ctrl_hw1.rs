#[doc = "Register `tosdac_ctrl_hw1` reader"]
pub type R = crate::R<TosdacCtrlHw1Spec>;
#[doc = "Register `tosdac_ctrl_hw1` writer"]
pub type W = crate::W<TosdacCtrlHw1Spec>;
#[doc = "Field `tbb_tosdac_i_gc0` reader - "]
pub type TbbTosdacIGc0R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_i_gc0` writer - "]
pub type TbbTosdacIGc0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_q_gc0` reader - "]
pub type TbbTosdacQGc0R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_q_gc0` writer - "]
pub type TbbTosdacQGc0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_i_gc1` reader - "]
pub type TbbTosdacIGc1R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_i_gc1` writer - "]
pub type TbbTosdacIGc1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tbb_tosdac_q_gc1` reader - "]
pub type TbbTosdacQGc1R = crate::FieldReader;
#[doc = "Field `tbb_tosdac_q_gc1` writer - "]
pub type TbbTosdacQGc1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc0(&self) -> TbbTosdacIGc0R {
        TbbTosdacIGc0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc0(&self) -> TbbTosdacQGc0R {
        TbbTosdacQGc0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc1(&self) -> TbbTosdacIGc1R {
        TbbTosdacIGc1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc1(&self) -> TbbTosdacQGc1R {
        TbbTosdacQGc1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc0(&mut self) -> TbbTosdacIGc0W<'_, TosdacCtrlHw1Spec> {
        TbbTosdacIGc0W::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc0(&mut self) -> TbbTosdacQGc0W<'_, TosdacCtrlHw1Spec> {
        TbbTosdacQGc0W::new(self, 8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc1(&mut self) -> TbbTosdacIGc1W<'_, TosdacCtrlHw1Spec> {
        TbbTosdacIGc1W::new(self, 16)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc1(&mut self) -> TbbTosdacQGc1W<'_, TosdacCtrlHw1Spec> {
        TbbTosdacQGc1W::new(self, 24)
    }
}
#[doc = "tosdac_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tosdac_ctrl_hw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tosdac_ctrl_hw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TosdacCtrlHw1Spec;
impl crate::RegisterSpec for TosdacCtrlHw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tosdac_ctrl_hw1::R`](R) reader structure"]
impl crate::Readable for TosdacCtrlHw1Spec {}
#[doc = "`write(|w| ..)` method takes [`tosdac_ctrl_hw1::W`](W) writer structure"]
impl crate::Writable for TosdacCtrlHw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tosdac_ctrl_hw1 to value 0"]
impl crate::Resettable for TosdacCtrlHw1Spec {}

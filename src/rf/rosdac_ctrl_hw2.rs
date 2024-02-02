#[doc = "Register `rosdac_ctrl_hw2` reader"]
pub type R = crate::R<RosdacCtrlHw2Spec>;
#[doc = "Register `rosdac_ctrl_hw2` writer"]
pub type W = crate::W<RosdacCtrlHw2Spec>;
#[doc = "Field `rosdac_i_gc2` reader - "]
pub type RosdacIGc2R = crate::FieldReader;
#[doc = "Field `rosdac_i_gc2` writer - "]
pub type RosdacIGc2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rosdac_q_gc2` reader - "]
pub type RosdacQGc2R = crate::FieldReader;
#[doc = "Field `rosdac_q_gc2` writer - "]
pub type RosdacQGc2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rosdac_i_gc3` reader - "]
pub type RosdacIGc3R = crate::FieldReader;
#[doc = "Field `rosdac_i_gc3` writer - "]
pub type RosdacIGc3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rosdac_q_gc3` reader - "]
pub type RosdacQGc3R = crate::FieldReader;
#[doc = "Field `rosdac_q_gc3` writer - "]
pub type RosdacQGc3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_i_gc2(&self) -> RosdacIGc2R {
        RosdacIGc2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_q_gc2(&self) -> RosdacQGc2R {
        RosdacQGc2R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_i_gc3(&self) -> RosdacIGc3R {
        RosdacIGc3R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_q_gc3(&self) -> RosdacQGc3R {
        RosdacQGc3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_i_gc2(&mut self) -> RosdacIGc2W<'_, RosdacCtrlHw2Spec> {
        RosdacIGc2W::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_q_gc2(&mut self) -> RosdacQGc2W<'_, RosdacCtrlHw2Spec> {
        RosdacQGc2W::new(self, 8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_i_gc3(&mut self) -> RosdacIGc3W<'_, RosdacCtrlHw2Spec> {
        RosdacIGc3W::new(self, 16)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_q_gc3(&mut self) -> RosdacQGc3W<'_, RosdacCtrlHw2Spec> {
        RosdacQGc3W::new(self, 24)
    }
}
#[doc = "rosdac_ctrl_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rosdac_ctrl_hw2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rosdac_ctrl_hw2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RosdacCtrlHw2Spec;
impl crate::RegisterSpec for RosdacCtrlHw2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rosdac_ctrl_hw2::R`](R) reader structure"]
impl crate::Readable for RosdacCtrlHw2Spec {}
#[doc = "`write(|w| ..)` method takes [`rosdac_ctrl_hw2::W`](W) writer structure"]
impl crate::Writable for RosdacCtrlHw2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rosdac_ctrl_hw2 to value 0"]
impl crate::Resettable for RosdacCtrlHw2Spec {}

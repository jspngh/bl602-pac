#[doc = "Register `rosdac_ctrl_hw1` reader"]
pub type R = crate::R<RosdacCtrlHw1Spec>;
#[doc = "Register `rosdac_ctrl_hw1` writer"]
pub type W = crate::W<RosdacCtrlHw1Spec>;
#[doc = "Field `rosdac_i_gc0` reader - "]
pub type RosdacIGc0R = crate::FieldReader;
#[doc = "Field `rosdac_i_gc0` writer - "]
pub type RosdacIGc0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rosdac_q_gc0` reader - "]
pub type RosdacQGc0R = crate::FieldReader;
#[doc = "Field `rosdac_q_gc0` writer - "]
pub type RosdacQGc0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rosdac_i_gc1` reader - "]
pub type RosdacIGc1R = crate::FieldReader;
#[doc = "Field `rosdac_i_gc1` writer - "]
pub type RosdacIGc1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rosdac_q_gc1` reader - "]
pub type RosdacQGc1R = crate::FieldReader;
#[doc = "Field `rosdac_q_gc1` writer - "]
pub type RosdacQGc1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_i_gc0(&self) -> RosdacIGc0R {
        RosdacIGc0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_q_gc0(&self) -> RosdacQGc0R {
        RosdacQGc0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_i_gc1(&self) -> RosdacIGc1R {
        RosdacIGc1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_q_gc1(&self) -> RosdacQGc1R {
        RosdacQGc1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_i_gc0(&mut self) -> RosdacIGc0W<'_, RosdacCtrlHw1Spec> {
        RosdacIGc0W::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_q_gc0(&mut self) -> RosdacQGc0W<'_, RosdacCtrlHw1Spec> {
        RosdacQGc0W::new(self, 8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_i_gc1(&mut self) -> RosdacIGc1W<'_, RosdacCtrlHw1Spec> {
        RosdacIGc1W::new(self, 16)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_q_gc1(&mut self) -> RosdacQGc1W<'_, RosdacCtrlHw1Spec> {
        RosdacQGc1W::new(self, 24)
    }
}
#[doc = "rosdac_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rosdac_ctrl_hw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rosdac_ctrl_hw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RosdacCtrlHw1Spec;
impl crate::RegisterSpec for RosdacCtrlHw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rosdac_ctrl_hw1::R`](R) reader structure"]
impl crate::Readable for RosdacCtrlHw1Spec {}
#[doc = "`write(|w| ..)` method takes [`rosdac_ctrl_hw1::W`](W) writer structure"]
impl crate::Writable for RosdacCtrlHw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rosdac_ctrl_hw1 to value 0"]
impl crate::Resettable for RosdacCtrlHw1Spec {}

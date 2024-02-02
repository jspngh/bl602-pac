#[doc = "Register `se_ctrl_reserved_2` reader"]
pub type R = crate::R<SeCtrlReserved2Spec>;
#[doc = "Register `se_ctrl_reserved_2` writer"]
pub type W = crate::W<SeCtrlReserved2Spec>;
#[doc = "Field `se_ctrl_reserved_2` reader - "]
pub type SeCtrlReserved2R = crate::FieldReader<u32>;
#[doc = "Field `se_ctrl_reserved_2` writer - "]
pub type SeCtrlReserved2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_ctrl_reserved_2(&self) -> SeCtrlReserved2R {
        SeCtrlReserved2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_ctrl_reserved_2(&mut self) -> SeCtrlReserved2W<'_, SeCtrlReserved2Spec> {
        SeCtrlReserved2W::new(self, 0)
    }
}
#[doc = "se_ctrl_reserved_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_reserved_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_ctrl_reserved_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeCtrlReserved2Spec;
impl crate::RegisterSpec for SeCtrlReserved2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_ctrl_reserved_2::R`](R) reader structure"]
impl crate::Readable for SeCtrlReserved2Spec {}
#[doc = "`write(|w| ..)` method takes [`se_ctrl_reserved_2::W`](W) writer structure"]
impl crate::Writable for SeCtrlReserved2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_ctrl_reserved_2 to value 0"]
impl crate::Resettable for SeCtrlReserved2Spec {}

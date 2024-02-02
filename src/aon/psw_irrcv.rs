#[doc = "Register `psw_irrcv` reader"]
pub type R = crate::R<PswIrrcvSpec>;
#[doc = "Register `psw_irrcv` writer"]
pub type W = crate::W<PswIrrcvSpec>;
#[doc = "Field `pu_ir_psw_aon` reader - "]
pub type PuIrPswAonR = crate::BitReader;
#[doc = "Field `pu_ir_psw_aon` writer - "]
pub type PuIrPswAonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ir_psw_aon(&self) -> PuIrPswAonR {
        PuIrPswAonR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ir_psw_aon(&mut self) -> PuIrPswAonW<'_, PswIrrcvSpec> {
        PuIrPswAonW::new(self, 0)
    }
}
#[doc = "psw_irrcv.\n\nYou can [`read`](crate::Reg::read) this register and get [`psw_irrcv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psw_irrcv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PswIrrcvSpec;
impl crate::RegisterSpec for PswIrrcvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psw_irrcv::R`](R) reader structure"]
impl crate::Readable for PswIrrcvSpec {}
#[doc = "`write(|w| ..)` method takes [`psw_irrcv::W`](W) writer structure"]
impl crate::Writable for PswIrrcvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets psw_irrcv to value 0"]
impl crate::Resettable for PswIrrcvSpec {}

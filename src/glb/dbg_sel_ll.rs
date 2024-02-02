#[doc = "Register `DBG_SEL_LL` reader"]
pub type R = crate::R<DbgSelLlSpec>;
#[doc = "Register `DBG_SEL_LL` writer"]
pub type W = crate::W<DbgSelLlSpec>;
#[doc = "Field `reg_dbg_ll_ctrl` reader - "]
pub type RegDbgLlCtrlR = crate::FieldReader<u32>;
#[doc = "Field `reg_dbg_ll_ctrl` writer - "]
pub type RegDbgLlCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_ll_ctrl(&self) -> RegDbgLlCtrlR {
        RegDbgLlCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_ll_ctrl(&mut self) -> RegDbgLlCtrlW<'_, DbgSelLlSpec> {
        RegDbgLlCtrlW::new(self, 0)
    }
}
#[doc = "DBG_SEL_LL.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_sel_ll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_sel_ll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgSelLlSpec;
impl crate::RegisterSpec for DbgSelLlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_sel_ll::R`](R) reader structure"]
impl crate::Readable for DbgSelLlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_sel_ll::W`](W) writer structure"]
impl crate::Writable for DbgSelLlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_SEL_LL to value 0"]
impl crate::Resettable for DbgSelLlSpec {}

#[doc = "Register `DBG_SEL_LH` reader"]
pub type R = crate::R<DbgSelLhSpec>;
#[doc = "Register `DBG_SEL_LH` writer"]
pub type W = crate::W<DbgSelLhSpec>;
#[doc = "Field `reg_dbg_lh_ctrl` reader - "]
pub type RegDbgLhCtrlR = crate::FieldReader<u32>;
#[doc = "Field `reg_dbg_lh_ctrl` writer - "]
pub type RegDbgLhCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_lh_ctrl(&self) -> RegDbgLhCtrlR {
        RegDbgLhCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_lh_ctrl(&mut self) -> RegDbgLhCtrlW<'_, DbgSelLhSpec> {
        RegDbgLhCtrlW::new(self, 0)
    }
}
#[doc = "DBG_SEL_LH.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_sel_lh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_sel_lh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgSelLhSpec;
impl crate::RegisterSpec for DbgSelLhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_sel_lh::R`](R) reader structure"]
impl crate::Readable for DbgSelLhSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_sel_lh::W`](W) writer structure"]
impl crate::Writable for DbgSelLhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_SEL_LH to value 0"]
impl crate::Resettable for DbgSelLhSpec {}

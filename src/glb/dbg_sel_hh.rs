#[doc = "Register `DBG_SEL_HH` reader"]
pub type R = crate::R<DbgSelHhSpec>;
#[doc = "Register `DBG_SEL_HH` writer"]
pub type W = crate::W<DbgSelHhSpec>;
#[doc = "Field `reg_dbg_hh_ctrl` reader - "]
pub type RegDbgHhCtrlR = crate::FieldReader<u32>;
#[doc = "Field `reg_dbg_hh_ctrl` writer - "]
pub type RegDbgHhCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_hh_ctrl(&self) -> RegDbgHhCtrlR {
        RegDbgHhCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_hh_ctrl(&mut self) -> RegDbgHhCtrlW<'_, DbgSelHhSpec> {
        RegDbgHhCtrlW::new(self, 0)
    }
}
#[doc = "DBG_SEL_HH.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_sel_hh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_sel_hh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgSelHhSpec;
impl crate::RegisterSpec for DbgSelHhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_sel_hh::R`](R) reader structure"]
impl crate::Readable for DbgSelHhSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_sel_hh::W`](W) writer structure"]
impl crate::Writable for DbgSelHhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_SEL_HH to value 0"]
impl crate::Resettable for DbgSelHhSpec {}

#[doc = "Register `DBG_SEL_HL` reader"]
pub type R = crate::R<DbgSelHlSpec>;
#[doc = "Register `DBG_SEL_HL` writer"]
pub type W = crate::W<DbgSelHlSpec>;
#[doc = "Field `reg_dbg_hl_ctrl` reader - "]
pub type RegDbgHlCtrlR = crate::FieldReader<u32>;
#[doc = "Field `reg_dbg_hl_ctrl` writer - "]
pub type RegDbgHlCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_hl_ctrl(&self) -> RegDbgHlCtrlR {
        RegDbgHlCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_hl_ctrl(&mut self) -> RegDbgHlCtrlW<'_, DbgSelHlSpec> {
        RegDbgHlCtrlW::new(self, 0)
    }
}
#[doc = "DBG_SEL_HL.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_sel_hl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_sel_hl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgSelHlSpec;
impl crate::RegisterSpec for DbgSelHlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_sel_hl::R`](R) reader structure"]
impl crate::Readable for DbgSelHlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_sel_hl::W`](W) writer structure"]
impl crate::Writable for DbgSelHlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_SEL_HL to value 0"]
impl crate::Resettable for DbgSelHlSpec {}

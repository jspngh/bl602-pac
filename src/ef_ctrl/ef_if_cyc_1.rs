#[doc = "Register `ef_if_cyc_1` reader"]
pub type R = crate::R<EfIfCyc1Spec>;
#[doc = "Register `ef_if_cyc_1` writer"]
pub type W = crate::W<EfIfCyc1Spec>;
#[doc = "Field `ef_if_cyc_pi` reader - "]
pub type EfIfCycPiR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_pi` writer - "]
pub type EfIfCycPiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_pp` reader - "]
pub type EfIfCycPpR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_pp` writer - "]
pub type EfIfCycPpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ef_if_cyc_wr_adr` reader - "]
pub type EfIfCycWrAdrR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_wr_adr` writer - "]
pub type EfIfCycWrAdrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_ps_cs` reader - "]
pub type EfIfCycPsCsR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_ps_cs` writer - "]
pub type EfIfCycPsCsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_pd_cs_h` reader - "]
pub type EfIfCycPdCsHR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_pd_cs_h` writer - "]
pub type EfIfCycPdCsHW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_pi(&self) -> EfIfCycPiR {
        EfIfCycPiR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn ef_if_cyc_pp(&self) -> EfIfCycPpR {
        EfIfCycPpR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn ef_if_cyc_wr_adr(&self) -> EfIfCycWrAdrR {
        EfIfCycWrAdrR::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25"]
    #[inline(always)]
    pub fn ef_if_cyc_ps_cs(&self) -> EfIfCycPsCsR {
        EfIfCycPsCsR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_h(&self) -> EfIfCycPdCsHR {
        EfIfCycPdCsHR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_pi(&mut self) -> EfIfCycPiW<'_, EfIfCyc1Spec> {
        EfIfCycPiW::new(self, 0)
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn ef_if_cyc_pp(&mut self) -> EfIfCycPpW<'_, EfIfCyc1Spec> {
        EfIfCycPpW::new(self, 6)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn ef_if_cyc_wr_adr(&mut self) -> EfIfCycWrAdrW<'_, EfIfCyc1Spec> {
        EfIfCycWrAdrW::new(self, 14)
    }
    #[doc = "Bits 20:25"]
    #[inline(always)]
    pub fn ef_if_cyc_ps_cs(&mut self) -> EfIfCycPsCsW<'_, EfIfCyc1Spec> {
        EfIfCycPsCsW::new(self, 20)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_h(&mut self) -> EfIfCycPdCsHW<'_, EfIfCyc1Spec> {
        EfIfCycPdCsHW::new(self, 26)
    }
}
#[doc = "ef_if_cyc_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_cyc_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_cyc_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfIfCyc1Spec;
impl crate::RegisterSpec for EfIfCyc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_if_cyc_1::R`](R) reader structure"]
impl crate::Readable for EfIfCyc1Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_if_cyc_1::W`](W) writer structure"]
impl crate::Writable for EfIfCyc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_if_cyc_1 to value 0x0020_6609"]
impl crate::Resettable for EfIfCyc1Spec {
    const RESET_VALUE: u32 = 0x0020_6609;
}

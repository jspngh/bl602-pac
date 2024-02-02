#[doc = "Register `ef_if_cyc_0` reader"]
pub type R = crate::R<EfIfCyc0Spec>;
#[doc = "Register `ef_if_cyc_0` writer"]
pub type W = crate::W<EfIfCyc0Spec>;
#[doc = "Field `ef_if_cyc_rd_dmy` reader - "]
pub type EfIfCycRdDmyR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_rd_dmy` writer - "]
pub type EfIfCycRdDmyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_rd_dat` reader - "]
pub type EfIfCycRdDatR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_rd_dat` writer - "]
pub type EfIfCycRdDatW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_rd_adr` reader - "]
pub type EfIfCycRdAdrR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_rd_adr` writer - "]
pub type EfIfCycRdAdrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_cs` reader - "]
pub type EfIfCycCsR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_cs` writer - "]
pub type EfIfCycCsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_pd_cs_s` reader - "]
pub type EfIfCycPdCsSR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_pd_cs_s` writer - "]
pub type EfIfCycPdCsSW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dmy(&self) -> EfIfCycRdDmyR {
        EfIfCycRdDmyR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dat(&self) -> EfIfCycRdDatR {
        EfIfCycRdDatR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_adr(&self) -> EfIfCycRdAdrR {
        EfIfCycRdAdrR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn ef_if_cyc_cs(&self) -> EfIfCycCsR {
        EfIfCycCsR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_s(&self) -> EfIfCycPdCsSR {
        EfIfCycPdCsSR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dmy(&mut self) -> EfIfCycRdDmyW<'_, EfIfCyc0Spec> {
        EfIfCycRdDmyW::new(self, 0)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dat(&mut self) -> EfIfCycRdDatW<'_, EfIfCyc0Spec> {
        EfIfCycRdDatW::new(self, 6)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_adr(&mut self) -> EfIfCycRdAdrW<'_, EfIfCyc0Spec> {
        EfIfCycRdAdrW::new(self, 12)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn ef_if_cyc_cs(&mut self) -> EfIfCycCsW<'_, EfIfCyc0Spec> {
        EfIfCycCsW::new(self, 18)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_s(&mut self) -> EfIfCycPdCsSW<'_, EfIfCyc0Spec> {
        EfIfCycPdCsSW::new(self, 24)
    }
}
#[doc = "ef_if_cyc_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_cyc_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_cyc_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfIfCyc0Spec;
impl crate::RegisterSpec for EfIfCyc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_if_cyc_0::R`](R) reader structure"]
impl crate::Readable for EfIfCyc0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_if_cyc_0::W`](W) writer structure"]
impl crate::Writable for EfIfCyc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_if_cyc_0 to value 0x1600_0040"]
impl crate::Resettable for EfIfCyc0Spec {
    const RESET_VALUE: u32 = 0x1600_0040;
}

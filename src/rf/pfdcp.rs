#[doc = "Register `pfdcp` reader"]
pub type R = crate::R<PfdcpSpec>;
#[doc = "Register `pfdcp` writer"]
pub type W = crate::W<PfdcpSpec>;
#[doc = "Field `lo_cp_sel` reader - "]
pub type LoCpSelR = crate::BitReader;
#[doc = "Field `lo_cp_sel` writer - "]
pub type LoCpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_cp_sel_hw` reader - "]
pub type LoCpSelHwR = crate::BitReader;
#[doc = "Field `lo_cp_sel_hw` writer - "]
pub type LoCpSelHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_cp_startup_en` reader - "]
pub type LoCpStartupEnR = crate::BitReader;
#[doc = "Field `lo_cp_startup_en` writer - "]
pub type LoCpStartupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_cp_ota_en` reader - "]
pub type LoCpOtaEnR = crate::BitReader;
#[doc = "Field `lo_cp_ota_en` writer - "]
pub type LoCpOtaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_cp_opamp_en` reader - "]
pub type LoCpOpampEnR = crate::BitReader;
#[doc = "Field `lo_cp_opamp_en` writer - "]
pub type LoCpOpampEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_cp_hiz` reader - "]
pub type LoCpHizR = crate::BitReader;
#[doc = "Field `lo_cp_hiz` writer - "]
pub type LoCpHizW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_pfd_rvdd_boost` reader - "]
pub type LoPfdRvddBoostR = crate::BitReader;
#[doc = "Field `lo_pfd_rvdd_boost` writer - "]
pub type LoPfdRvddBoostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_pfd_rst_csd` reader - "]
pub type LoPfdRstCsdR = crate::BitReader;
#[doc = "Field `lo_pfd_rst_csd` writer - "]
pub type LoPfdRstCsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_pfd_rst_csd_hw` reader - "]
pub type LoPfdRstCsdHwR = crate::BitReader;
#[doc = "Field `lo_pfd_rst_csd_hw` writer - "]
pub type LoPfdRstCsdHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_cp_sel(&self) -> LoCpSelR {
        LoCpSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_cp_sel_hw(&self) -> LoCpSelHwR {
        LoCpSelHwR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_cp_startup_en(&self) -> LoCpStartupEnR {
        LoCpStartupEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_cp_ota_en(&self) -> LoCpOtaEnR {
        LoCpOtaEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_cp_opamp_en(&self) -> LoCpOpampEnR {
        LoCpOpampEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_cp_hiz(&self) -> LoCpHizR {
        LoCpHizR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_pfd_rvdd_boost(&self) -> LoPfdRvddBoostR {
        LoPfdRvddBoostR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd(&self) -> LoPfdRstCsdR {
        LoPfdRstCsdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd_hw(&self) -> LoPfdRstCsdHwR {
        LoPfdRstCsdHwR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_cp_sel(&mut self) -> LoCpSelW<'_, PfdcpSpec> {
        LoCpSelW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_cp_sel_hw(&mut self) -> LoCpSelHwW<'_, PfdcpSpec> {
        LoCpSelHwW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_cp_startup_en(&mut self) -> LoCpStartupEnW<'_, PfdcpSpec> {
        LoCpStartupEnW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_cp_ota_en(&mut self) -> LoCpOtaEnW<'_, PfdcpSpec> {
        LoCpOtaEnW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_cp_opamp_en(&mut self) -> LoCpOpampEnW<'_, PfdcpSpec> {
        LoCpOpampEnW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_cp_hiz(&mut self) -> LoCpHizW<'_, PfdcpSpec> {
        LoCpHizW::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_pfd_rvdd_boost(&mut self) -> LoPfdRvddBoostW<'_, PfdcpSpec> {
        LoPfdRvddBoostW::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd(&mut self) -> LoPfdRstCsdW<'_, PfdcpSpec> {
        LoPfdRstCsdW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd_hw(&mut self) -> LoPfdRstCsdHwW<'_, PfdcpSpec> {
        LoPfdRstCsdHwW::new(self, 29)
    }
}
#[doc = "pfdcp.\n\nYou can [`read`](crate::Reg::read) this register and get [`pfdcp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfdcp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfdcpSpec;
impl crate::RegisterSpec for PfdcpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfdcp::R`](R) reader structure"]
impl crate::Readable for PfdcpSpec {}
#[doc = "`write(|w| ..)` method takes [`pfdcp::W`](W) writer structure"]
impl crate::Writable for PfdcpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pfdcp to value 0"]
impl crate::Resettable for PfdcpSpec {}

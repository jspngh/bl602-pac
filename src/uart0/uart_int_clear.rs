#[doc = "Register `uart_int_clear` reader"]
pub type R = crate::R<UartIntClearSpec>;
#[doc = "Register `uart_int_clear` writer"]
pub type W = crate::W<UartIntClearSpec>;
#[doc = "Field `cr_utx_end_clr` reader - "]
pub type CrUtxEndClrR = crate::BitReader;
#[doc = "Field `cr_utx_end_clr` writer - "]
pub type CrUtxEndClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_end_clr` reader - "]
pub type CrUrxEndClrR = crate::BitReader;
#[doc = "Field `cr_urx_end_clr` writer - "]
pub type CrUrxEndClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rsvd_2` reader - "]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `rsvd_2` writer - "]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rsvd_3` reader - "]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `rsvd_3` writer - "]
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_rto_clr` reader - "]
pub type CrUrxRtoClrR = crate::BitReader;
#[doc = "Field `cr_urx_rto_clr` writer - "]
pub type CrUrxRtoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_pce_clr` reader - "]
pub type CrUrxPceClrR = crate::BitReader;
#[doc = "Field `cr_urx_pce_clr` writer - "]
pub type CrUrxPceClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rsvd_6` reader - "]
pub type Rsvd6R = crate::BitReader;
#[doc = "Field `rsvd_6` writer - "]
pub type Rsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rsvd_7` reader - "]
pub type Rsvd7R = crate::BitReader;
#[doc = "Field `rsvd_7` writer - "]
pub type Rsvd7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_clr(&self) -> CrUtxEndClrR {
        CrUtxEndClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_clr(&self) -> CrUrxEndClrR {
        CrUrxEndClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsvd_3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_clr(&self) -> CrUrxRtoClrR {
        CrUrxRtoClrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_clr(&self) -> CrUrxPceClrR {
        CrUrxPceClrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rsvd_7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_clr(&mut self) -> CrUtxEndClrW<'_, UartIntClearSpec> {
        CrUtxEndClrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_clr(&mut self) -> CrUrxEndClrW<'_, UartIntClearSpec> {
        CrUrxEndClrW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, UartIntClearSpec> {
        Rsvd2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsvd_3(&mut self) -> Rsvd3W<'_, UartIntClearSpec> {
        Rsvd3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_clr(&mut self) -> CrUrxRtoClrW<'_, UartIntClearSpec> {
        CrUrxRtoClrW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_clr(&mut self) -> CrUrxPceClrW<'_, UartIntClearSpec> {
        CrUrxPceClrW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> Rsvd6W<'_, UartIntClearSpec> {
        Rsvd6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rsvd_7(&mut self) -> Rsvd7W<'_, UartIntClearSpec> {
        Rsvd7W::new(self, 7)
    }
}
#[doc = "UART interrupt clear\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIntClearSpec;
impl crate::RegisterSpec for UartIntClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_int_clear::R`](R) reader structure"]
impl crate::Readable for UartIntClearSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_int_clear::W`](W) writer structure"]
impl crate::Writable for UartIntClearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets uart_int_clear to value 0"]
impl crate::Resettable for UartIntClearSpec {}

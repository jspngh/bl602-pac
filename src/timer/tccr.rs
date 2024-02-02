#[doc = "Register `TCCR` reader"]
pub type R = crate::R<TccrSpec>;
#[doc = "Register `TCCR` writer"]
pub type W = crate::W<TccrSpec>;
#[doc = "Field `cs_1` reader - "]
pub type Cs1R = crate::FieldReader;
#[doc = "Field `cs_1` writer - "]
pub type Cs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED_4` reader - "]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `RESERVED_4` writer - "]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cs_2` reader - "]
pub type Cs2R = crate::FieldReader;
#[doc = "Field `cs_2` writer - "]
pub type Cs2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED_7` reader - "]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED_7` writer - "]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cs_wdt` reader - "]
pub type CsWdtR = crate::FieldReader;
#[doc = "Field `cs_wdt` writer - "]
pub type CsWdtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cs_1(&self) -> Cs1R {
        Cs1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reserved_4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cs_2(&self) -> Cs2R {
        Cs2R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cs_wdt(&self) -> CsWdtR {
        CsWdtR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cs_1(&mut self) -> Cs1W<'_, TccrSpec> {
        Cs1W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reserved_4(&mut self) -> Reserved4W<'_, TccrSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cs_2(&mut self) -> Cs2W<'_, TccrSpec> {
        Cs2W::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&mut self) -> Reserved7W<'_, TccrSpec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cs_wdt(&mut self) -> CsWdtW<'_, TccrSpec> {
        CsWdtW::new(self, 8)
    }
}
#[doc = "TCCR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TccrSpec;
impl crate::RegisterSpec for TccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tccr::R`](R) reader structure"]
impl crate::Readable for TccrSpec {}
#[doc = "`write(|w| ..)` method takes [`tccr::W`](W) writer structure"]
impl crate::Writable for TccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCCR to value 0"]
impl crate::Resettable for TccrSpec {}

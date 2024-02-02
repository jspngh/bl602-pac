#[doc = "Register `UART_SIG_SEL_0` reader"]
pub type R = crate::R<UartSigSel0Spec>;
#[doc = "Register `UART_SIG_SEL_0` writer"]
pub type W = crate::W<UartSigSel0Spec>;
#[doc = "Field `uart_sig_0_sel` reader - "]
pub type UartSig0SelR = crate::FieldReader;
#[doc = "Field `uart_sig_0_sel` writer - "]
pub type UartSig0SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `uart_sig_1_sel` reader - "]
pub type UartSig1SelR = crate::FieldReader;
#[doc = "Field `uart_sig_1_sel` writer - "]
pub type UartSig1SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `uart_sig_2_sel` reader - "]
pub type UartSig2SelR = crate::FieldReader;
#[doc = "Field `uart_sig_2_sel` writer - "]
pub type UartSig2SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `uart_sig_3_sel` reader - "]
pub type UartSig3SelR = crate::FieldReader;
#[doc = "Field `uart_sig_3_sel` writer - "]
pub type UartSig3SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `uart_sig_4_sel` reader - "]
pub type UartSig4SelR = crate::FieldReader;
#[doc = "Field `uart_sig_4_sel` writer - "]
pub type UartSig4SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `uart_sig_5_sel` reader - "]
pub type UartSig5SelR = crate::FieldReader;
#[doc = "Field `uart_sig_5_sel` writer - "]
pub type UartSig5SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `uart_sig_6_sel` reader - "]
pub type UartSig6SelR = crate::FieldReader;
#[doc = "Field `uart_sig_6_sel` writer - "]
pub type UartSig6SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `uart_sig_7_sel` reader - "]
pub type UartSig7SelR = crate::FieldReader;
#[doc = "Field `uart_sig_7_sel` writer - "]
pub type UartSig7SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn uart_sig_0_sel(&self) -> UartSig0SelR {
        UartSig0SelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn uart_sig_1_sel(&self) -> UartSig1SelR {
        UartSig1SelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uart_sig_2_sel(&self) -> UartSig2SelR {
        UartSig2SelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn uart_sig_3_sel(&self) -> UartSig3SelR {
        UartSig3SelR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn uart_sig_4_sel(&self) -> UartSig4SelR {
        UartSig4SelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn uart_sig_5_sel(&self) -> UartSig5SelR {
        UartSig5SelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn uart_sig_6_sel(&self) -> UartSig6SelR {
        UartSig6SelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn uart_sig_7_sel(&self) -> UartSig7SelR {
        UartSig7SelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn uart_sig_0_sel(&mut self) -> UartSig0SelW<'_, UartSigSel0Spec> {
        UartSig0SelW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn uart_sig_1_sel(&mut self) -> UartSig1SelW<'_, UartSigSel0Spec> {
        UartSig1SelW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uart_sig_2_sel(&mut self) -> UartSig2SelW<'_, UartSigSel0Spec> {
        UartSig2SelW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn uart_sig_3_sel(&mut self) -> UartSig3SelW<'_, UartSigSel0Spec> {
        UartSig3SelW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn uart_sig_4_sel(&mut self) -> UartSig4SelW<'_, UartSigSel0Spec> {
        UartSig4SelW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn uart_sig_5_sel(&mut self) -> UartSig5SelW<'_, UartSigSel0Spec> {
        UartSig5SelW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn uart_sig_6_sel(&mut self) -> UartSig6SelW<'_, UartSigSel0Spec> {
        UartSig6SelW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn uart_sig_7_sel(&mut self) -> UartSig7SelW<'_, UartSigSel0Spec> {
        UartSig7SelW::new(self, 28)
    }
}
#[doc = "UART_SIG_SEL_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_sig_sel_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_sig_sel_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartSigSel0Spec;
impl crate::RegisterSpec for UartSigSel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_sig_sel_0::R`](R) reader structure"]
impl crate::Readable for UartSigSel0Spec {}
#[doc = "`write(|w| ..)` method takes [`uart_sig_sel_0::W`](W) writer structure"]
impl crate::Writable for UartSigSel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_SIG_SEL_0 to value 0x7654_3210"]
impl crate::Resettable for UartSigSel0Spec {
    const RESET_VALUE: u32 = 0x7654_3210;
}

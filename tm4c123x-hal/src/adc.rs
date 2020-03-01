use hal::adc;
use tm4c123x::{ADC0};
//TODO: Macro this
trait AdcPIn{}

struct Pin1{
	enabled: bool,
}
struct Pin2{
	enabled: bool,
}
struct Pin3{
	enabled: bool,
}
struct Pin4{
	enabled: bool,
}
struct Pin5{
	enabled: bool,
}
struct Pin6{
	enabled: bool,
}
pub struct Tm4cAdc{
	adc: ADC0,
}
//Macro this
impl adc::Channel<Tm4cAdc> for Pin1{
	type ID = u8;
	fn channel() -> Self::ID{
		0
	}
}
impl adc::Channel<Tm4cAdc> for Pin2{
	type ID = u8;
	fn channel() -> Self::ID{
		1
	}
}
impl adc::Channel<Tm4cAdc> for Pin3{
	type ID = u8;
	fn channel() -> Self::ID{
		2
	}
}
impl adc::Channel<Tm4cAdc> for Pin4{
	type ID = u8;
	fn channel() -> Self::ID{
		3
	}
}
impl adc::Channel<Tm4cAdc> for Pin5{
	type ID = u8;
	fn channel() -> Self::ID{
		8
	}
}
impl adc::Channel<Tm4cAdc> for Pin6{
	type ID = u8;
	fn channel() -> Self::ID{
		9
	}
}




impl <U8, Pin> adc::OneShot<Tm4cAdc, U8, Pin> for Tm4cAdc
where
   Pin: hal::adc::Channel<Tm4cAdc, ID=u8>,
   U8:   From<u32>,
{
	type Error = u8;

	fn read(&mut self, _pin: &mut Pin) -> Result<U8, nb::Error<u8>>{
        //let p = unsafe { &*tm4c123x::ADC0::ptr() };
        self.adc.pssi.write(|w| unsafe{w.bits(0x0008)});
        while self.adc.ris.read().bits()&0x08==0 {};
        let out = self.adc.ssfifo3.read().bits()& 0x0FFF;
        self.adc.isc.write(|w| unsafe{w.bits(0x00008)});
		Ok(out.into())
		// Ok(u8::from(4))
	}

}

impl Tm4cAdc{
	pub fn adc0(adcin: tm4c123x::ADC0) -> Self{
		//let curradc = adcin;
		adcin.pc.write(|w| unsafe{w.bits(adcin.pc.read().bits() & !0x0F) });
        adcin.pc.write(|w| unsafe{w.bits(adcin.pc.read().bits() | 0x01 )});
        adcin.sspri.write(|w| unsafe{w.bits(0x0123)});
        adcin.actss.write(|w| unsafe{w.bits(adcin.actss.read().bits() & !0x0008) });
        adcin.emux.write(|w| unsafe{w.bits(adcin.emux.read().bits() & !0xF000) });
        adcin.ssmux3.write(|w| unsafe{w.bits(adcin.ssmux3.read().bits() & !0x000F )});
        adcin.ssmux3.write(|w| unsafe{w.bits(adcin.ssmux3.read().bits() + 9 )});
        adcin.ssctl3.write(|w| unsafe{w.bits(0x06)});
        adcin.im.write(|w| unsafe{w.bits(adcin.im.read().bits() & !0x0008 )});
        // let pins = [Pin{channel}]{

        // }
		let adc = Tm4cAdc{
			adc: adcin,
			//Pins: []

		};
		adc
	}


}
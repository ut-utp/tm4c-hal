use hal::adc;
use tm4c123x::{ADC0};
use crate::{sysctl, gpio::gpioe};
use crate::{gpio::*, gpio::gpioe::*};
//TODO: Macro this
trait AdcPIn{}

pub struct Tm4cAdc//<T: Sized>
{
	adc: ADC0,
	//pins: (T, T, T, T, T, T),   // having some recursive trait bound issues.
								  // just hardcoding pins for now
								  // TODO: Find a way to use generics here
	
	pins: (PE3<AnalogIn>, PE2<AnalogIn>, PE1<AnalogIn>, 
	PE0<AnalogIn>, PE5<AnalogIn>, PE4<AnalogIn>),						  
}
//Macro this
impl adc::Channel<Tm4cAdc> for PE3<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		0
	}
}
impl adc::Channel<Tm4cAdc> for PE2<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		1
	}
}
impl adc::Channel<Tm4cAdc> for PE1<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		2
	}
}
impl adc::Channel<Tm4cAdc> for PE0<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		3
	}
}
impl adc::Channel<Tm4cAdc> for PE5<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		8
	}
}
impl adc::Channel<Tm4cAdc> for PE4<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		9
	}
}

pub struct Channel_pe0;
pub struct Channel_pe1;
pub struct Channel_pe2;
pub struct Channel_pe3;
pub struct Channel_pe5;
pub struct Channel_pe4;

impl adc::Channel<Tm4cAdc> for Channel_pe3{
	type ID =u8;
	fn channel() -> Self::ID{
		0
	}
}

impl adc::Channel<Tm4cAdc> for Channel_pe2{
	type ID =u8;
	fn channel() -> Self::ID{
		1
	}
}

impl adc::Channel<Tm4cAdc> for Channel_pe1{
	type ID =u8;
	fn channel() -> Self::ID{
		2
	}
}

impl adc::Channel<Tm4cAdc> for Channel_pe0{
	type ID =u8;
	fn channel() -> Self::ID{
		3
	}
}

impl adc::Channel<Tm4cAdc> for Channel_pe5{
	type ID =u8;
	fn channel() -> Self::ID{
		8
	}
}

impl adc::Channel<Tm4cAdc> for Channel_pe4{
	type ID =u8;
	fn channel() -> Self::ID{
		9
	}
}
 impl From<u32> for Channel_pe0{
   fn from(x: u32)->Self{
     Channel_pe0
   }
 }

  impl From<u32> for Channel_pe1{
   fn from(x: u32)->Self{
     Channel_pe1
   }
 }

  impl From<u32> for Channel_pe2{
   fn from(x: u32)->Self{
     Channel_pe2
   }
 }

  impl From<u32> for Channel_pe3{
   fn from(x: u32)->Self{
     Channel_pe3
   }
 }

  impl From<u32> for Channel_pe5{
   fn from(x: u32)->Self{
     Channel_pe5
   }
 }

  impl From<u32> for Channel_pe4{
   fn from(x: u32)->Self{
     Channel_pe4
   }
 }


 //  impl Into<Channel_pe0> for u32{
 //   fn into(self)->Channel_pe0{
 //   		let mut res = Channel_pe0(-1);
 //   		if(self==0){
 // 	  	res = Channel_pe0(0);
 //    	}
 //    	res
 //   }
 // }

 //  impl Into<Channel_pe1> for u32{
 //   fn into(self)->Channel_pe1{
 //   		let mut res = Channel_pe1(-1);
 //   		if(self==1){
 // 	  	res = Channel_pe1(1);
 //    	}
 //    	res
 //   }
 // }

 //  impl Into<Channel_pe2> for u32{
 //   fn into(self)->Channel_pe2{
 //   		let mut res = Channel_pe2(-1);
 //   		if(self==2){
 // 	  	res = Channel_pe2(2);
 //    	}
 //    	res
 //   }
 // }

 //  impl Into<Channel_pe3> for u32{
 //   fn into(self)->Channel_pe3{
 //   		let mut res = Channel_pe3(-1);
 //   		if(self==3){
 // 	  	res = Channel_pe3(3);
 //    	}
 //    	res
 //   }
 // }

 //  impl Into<Channel_pe5> for u32{
 //   fn into(self)->Channel_pe5{
 //   		let mut res = Channel_pe5(-1);
 //   		if(self==5){
 // 	  	res = Channel_pe5(5);
 //    	}
 //    	res
 //   }
 // }

 //  impl Into<Channel_pe4> for u32{
 //   fn into(self)->Channel_pe4{
 //   		let mut res = Channel_pe4(-1);
 //   		if(self==4){
 // 	  	res = Channel_pe4(4);
 //    	}
 //    	res
 //   }
 // }
// impl Into<u32> for PE0<AnalogIn>{
//   fn into(self)->u32{
//     3
//   }
// }

// impl Into<u32> for PE1<AnalogIn>{
//   fn into(self)->u32{
//     2
//   }
// }




impl <U8, Pin> adc::OneShot<Tm4cAdc, U8, Pin> for Tm4cAdc
where
   Pin: hal::adc::Channel<Tm4cAdc, ID=u8>,
   U8:   From<u32>,
{
	type Error = u8;

	fn read(&mut self, _pin: &mut Pin) -> Result<U8, nb::Error<u8>>{
		//let p = *(_pin);
		let channel = Pin::channel();
		match channel{
                0 => {
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() & !0x000F )});
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() + 0 )});
                }
                1 => {
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() & !0x000F )});
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() + 1 )});
                }
                2 => {
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() & !0x000F )});
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() + 2 )});

                }
                3 => {
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() & !0x000F )});
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() + 3 )});
                }
                8 => {
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() & !0x000F )});
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() + 8 )});
                }
                9 => {
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() & !0x000F )});
                    self.adc.ssmux3.write(|w| unsafe{w.bits(self.adc.ssmux3.read().bits() + 9 )});
                }


                 _=> {

                }			
		}
        //let p = unsafe { &*tm4c123x::ADC0::ptr() };
        self.adc.pssi.write(|w| unsafe{w.bits(0x0008)});
        while self.adc.ris.read().bits()&0x08==0 {};
        let out = self.adc.ssfifo3.read().bits()& 0x0FFF;
        self.adc.isc.write(|w| unsafe{w.bits(0x00008)});
		Ok(out.into())
		// Ok(u8::from(4))
	}

}

impl Tm4cAdc
   
where

{
	pub fn adc0(adcin: tm4c123x::ADC0, power: &sysctl::PowerControl, 
				pins: (PE3<AnalogIn>, PE2<AnalogIn>, PE1<AnalogIn>, 
				PE0<AnalogIn>, PE5<AnalogIn>, PE4<AnalogIn>)) -> Self{
		//let curradc = adcin;
		//adcin.pc.write(|w| unsafe{w.bits(adcin.pc.read().bits() & !0x0F) });
        //adcin.pc.write(|w| unsafe{w.bits(adcin.pc.read().bits() | 0x01 )});
        sysctl::control_power(
            power, sysctl::Domain::Adc0,
            sysctl::RunMode::Run, sysctl::PowerState::On);
        sysctl::reset(power, sysctl::Domain::Adc0);
        adcin.sspri.write(|w| unsafe{w.bits(0x0123)});
        adcin.actss.write(|w| unsafe{w.bits(adcin.actss.read().bits() & !0x0008) });
        adcin.emux.write(|w| unsafe{w.bits(adcin.emux.read().bits() & !0xF000) });
        adcin.ssmux3.write(|w| unsafe{w.bits(adcin.ssmux3.read().bits() & !0x000F )});
        adcin.ssmux3.write(|w| unsafe{w.bits(adcin.ssmux3.read().bits() + 9 )});
        adcin.ssctl3.write(|w| unsafe{w.bits(0x06)});
        adcin.im.write(|w| unsafe{w.bits(adcin.im.read().bits() & !0x0008 )});
        // let pins = [Pin{channel}]{

        // }
		Tm4cAdc{
			adc: adcin,
			pins: pins,
			//Pins: []

		}
		//adc
	}


}
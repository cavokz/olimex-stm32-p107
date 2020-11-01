use stm32f1xx_hal::{afio, gpio, pac, rcc, serial};

pub use serial::Config;

pub type Rs232 = serial::Serial<
    pac::USART2,
    (gpio::gpiod::PD5<gpio::Alternate<gpio::PushPull>>, gpio::gpiod::PD6<gpio::Input<gpio::Floating>>),
>;

pub type Uext = serial::Serial<
    pac::USART3,
    (gpio::gpiod::PD8<gpio::Alternate<gpio::PushPull>>, gpio::gpiod::PD9<gpio::Input<gpio::Floating>>),
>;

pub fn rs232<M1, M2>(usart2: pac::USART2,
    pd5: gpio::gpiod::PD5<M1>,
    pd6: gpio::gpiod::PD6<M2>,
    crl: &mut gpio::gpiod::CRL,
    mapr: &mut afio::MAPR,
    config: serial::Config,
    clocks: rcc::Clocks,
    apb1: &mut rcc::APB1
) -> Rs232
    where M1: stm32f1xx_hal::gpio::Active,
          M2: stm32f1xx_hal::gpio::Active,
{
    serial::Serial::usart2(
        usart2,
        (pd5.into_alternate_push_pull(crl), pd6.into_floating_input(crl)),
        mapr,
        config,
        clocks,
        apb1,
    )
}

pub fn uext<M1, M2>(usart3: pac::USART3,
    pd8: gpio::gpiod::PD8<M1>,
    pd9: gpio::gpiod::PD9<M2>,
    crh: &mut gpio::gpiod::CRH,
    mapr: &mut afio::MAPR,
    config: serial::Config,
    clocks: rcc::Clocks,
    apb1: &mut rcc::APB1
) -> Uext
    where M1: stm32f1xx_hal::gpio::Active,
          M2: stm32f1xx_hal::gpio::Active,
{
    serial::Serial::usart3(
        usart3,
        (pd8.into_alternate_push_pull(crh), pd9.into_floating_input(crh)),
        mapr,
        config,
        clocks,
        apb1,
    )
}

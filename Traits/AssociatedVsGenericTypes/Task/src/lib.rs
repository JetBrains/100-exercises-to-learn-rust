// TODO: Визначте новий trait `Power` із методом `power`, який підносить `self`
//  до степеня `n`.
//  Визначення trait та його реалізацій має бути достатньо, щоб
//  тести скомпілювалися й пройшли.
//
// Рекомендація: може виникнути спокуса написати generic реалізацію, яка
// охопить усі випадки одразу. Однак це досить складно й потребує використання
// додаткових crates (crate: одиниця компіляції Rust), тобто `num-traits`.
// Але навіть тоді, можливо, краще скористатися простим макросом, щоб уникнути
// складності надто generic реалізації. Зазирніть у
// «Little book of Rust macros» (https://veykril.github.io/tlborm/), якщо
// хочете дізнатися про це більше.
// Утім, це не обов'язково: цілком нормально написати три окремі
// реалізації вручну. Заглиблюйтеся далі, лише якщо вам цікаво.

pub trait Power<Exponent = Self> {
    type Output;

    fn power(&self, n: Exponent) -> Self::Output;
}

impl Power<u16> for u32 {
    type Output = u32;

    fn power(&self, n: u16) -> Self::Output {
        self.pow(n.into())
    }
}

impl Power<&u32> for u32 {
    type Output = u32;

    fn power(&self, n: &u32) -> Self::Output {
        self.power(*n)
    }
}

impl Power<u32> for u32 {
    type Output = u32;

    fn power(&self, n: u32) -> Self::Output {
        self.pow(n)
    }
}

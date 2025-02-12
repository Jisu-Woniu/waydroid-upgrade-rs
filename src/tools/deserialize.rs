use std::{fmt, marker::PhantomData};

use serde::{
    de::{self, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

pub fn deserialize_max<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: PartialOrd + Deserialize<'de>,
{
    struct MaxVisitor<T>(
        /// `MaxVisitor` generates instances of `T`, so `fn() -> T` is used as parameter.
        PhantomData<fn() -> T>,
    );

    impl<'de, T> Visitor<'de> for MaxVisitor<T>
    where
        T: Deserialize<'de> + PartialOrd,
    {
        /// Return type of this visitor. This visitor computes the max of a
        /// sequence of values of type T, so the type of the maximum is T.
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a nonempty sequence of values")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<T, S::Error>
        where
            S: SeqAccess<'de>,
        {
            // Start with max equal to the first value in the seq.
            let mut max: T = seq.next_element()?.ok_or_else(||
                    // Cannot take the maximum of an empty seq.
                    de::Error::custom("no values in seq when looking for maximum"))?;

            // Update the max while there are additional values.
            while let Some(value) = seq.next_element()? {
                max = if max < value { value } else { max };
            }

            Ok(max)
        }
    }

    let visitor = MaxVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}

macro_rules! vtable {
    ($(
        $(#[doc = $doc:literal])*
        $ident:ident: fn($($input_name:ident: $input:ty),*) $(-> $output:ty)?,
    )* $(,)?) => {
        #[derive(Clone)]
        pub struct HostVTable {
            #[doc = "The version of the host's FAPI implementation."]
            version: semver::Version,
            $(
                $(
                    #[doc = $doc]
                )*
                $ident: extern "C" fn($($input),*) $(-> $output)?,
            )*
        }

        impl HostVTable {
            #[doc = "Returns a dummy vtable whose functions all panic."]
            pub fn dummy() -> Self {
                Self {
                    version: semver::Version::new(1, 0, 0),
                    $(
                        $ident: dummy_fns::$ident as _,
                    )*
                }
            }

            $(
                $(
                    #[doc = $doc]
                )*
                pub unsafe fn $ident(&self, $($input_name: $input),*) $(-> $output)? {
                    let f = self.$ident;
                    f($($input_name),*)
                }
            )*
        }

        impl Serialize for HostVTable {
            fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
            where
                S: Serializer,
            {
                let len = [$(stringify!($ident)),*].len() + 1;
                let mut state = serializer.serialize_struct("HostVTable", len)?;

                state.serialize_field("version", &self.version)?;

                $(
                    state.serialize_field(stringify!($ident), &(self.$ident as usize as u64))?;
                )*

                state.end()
            }
        }

        impl <'de> Deserialize<'de> for HostVTable {
            fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_struct("HostVTable", &["version", $(stringify!($ident)),*], HostVTableVisitor)
            }
        }

        #[allow(non_camel_case_types)]
        enum Field {
            version,
            $($ident,)*
        }

        impl <'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error> where D: Deserializer<'de> {
                struct FieldVisitor;

                impl <'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        write!(formatter, "vtable field")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E> where E: de::Error {
                        match value {
                            "version" => Ok(Field::version),
                            $(
                                stringify!($ident) => Ok(Field::$ident),
                            )*
                            _ => Err(de::Error::unknown_field(value, &[])),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct HostVTableVisitor;

        impl <'de> Visitor<'de> for HostVTableVisitor {
            type Value = HostVTable;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(formatter, "struct HostVTable")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut len = 0;
                let version = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(len, &self))?;

                $(
                    len += 1;
                    let $ident: u64 = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(len, &self))?;
                    let $ident = unsafe { mem::transmute::<*const (), _>($ident as usize as *const ()) };
                )*

                Ok(HostVTable {
                    version,
                    $(
                        $ident,
                    )*
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<HostVTable, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut version = None;
                $(
                    let mut $ident = None;
                )*

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::version => {
                            if version.is_some() {
                                return Err(de::Error::duplicate_field("version"));
                            }
                            version = Some(map.next_value()?);
                        }
                        $(
                            Field::$ident => {
                                if $ident.is_some() {
                                    return Err(de::Error::duplicate_field(stringify!($ident)));
                                }
                                $ident = Some(map.next_value()?);
                            }
                        )*
                    }
                }

                let version = version.ok_or_else(|| de::Error::missing_field("version"))?;
                $(
                    let $ident: u64 = $ident.ok_or_else(|| de::Error::missing_field(stringify!($ident)))?;
                    let $ident = unsafe { mem::transmute::<*const (), _>($ident as usize as *const ()) };
                )*

                Ok(HostVTable {
                    version,
                    $($ident,)*
                })
            }
        }

        #[doc = "Dummy functions for testing vtable functions. All functions panic when called."]
        mod dummy_fns {
            use super::*;
            $(
                #[allow(unused_variables)]
                pub extern "C" fn $ident($($input_name: $input),*) $(-> $output)? {
                    unreachable!()
                }
            )*
        }
    };
}

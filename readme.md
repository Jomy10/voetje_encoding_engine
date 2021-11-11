# Voetje Encoding Engine
Deze code wordt door ['t Voetje](https://voetje.jonaseveraert.be) gebruikt om tekst te encoderen, zowel op iOS als op Android.

## Inhoudstafel
- [Hoe het werkt](#hoe-het-werkt)
- [Waarom Rust?](#waarom-in-rust)
- [Hoe kan jij helpen?](#hoe-kan-jij-helpen)

## Hoe het werkt
Alle functies voor het encoderen worden geschreven in pure Rust en staan in [encoding_funcs.rs](src/encoding_funcs.rs).<br/>
Vervolgens wordt er gebruik gemaakt van `ffi` en `jni` zodat deze gebruikt kunnen worden in Swift (C) en Java (Swift voor iOS en Java voor Android). 

**Voorbeeld**
In [encoding_funcs.rs](src/encoding_funcs.rs) staat de functie voor het encoderen vaan *Jaartal*.
```Rust
pub fn encode_jaar_uni(input: &str, jaar: &str) -> (u8, String) {
    // ...
}
```

Deze wordt gebruikt in [lib.rs](src/lib.rs) om een link te maken naar C (voor Swift) en Java.
```Rust
#[no_mangle]
// Link tussen `encode_jaar_uni` en C
pub extern "C" fn encode_jaar(input: *const c_char, jaar: *const c_char) -> C_Return {
    // ...
}

#[cfg(target_os="ios")]
#[no_mangle]
// Memory vrij maken nadat `encode_jaar` gebruikt is (niet nodig in de Java functie)
pub extern "C" fn jaar_free(cret: C_Return) { /... }

// Android functions
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    // ...

    #[no_mangle]
    // Java heeft de namespace nodig; be.ksa.voetje.metods.encoderen.EncodingEngine is waar de referentie naar de functie java_encodeer_jaar zich bevindt
    pub unsafe extern fn Java_be_ksa_voetje_methods_encoderen_EncodingEngine_java_1encodeer_1jaar(env: JNIEnv, _: JClass, java_in: JString, java_jaar: JString) -> jstring {
        // ...
    }
}
```

## Waarom in Rust?
Rust is veilig en zeer efficient. Deze Rust library kan gemakkelijk gecompileerd worden om gebruikt te worden op iOS en Android. Er is ook de mogelijkheid om te compileren naar WebAssembly en een heleboel andere programeertalen. Daarnaast heeft Rust ook een grote community die je altijd zal helpen en zijn er een heleboel open-source libraries beschikbaar.

## Hoe kan jij helpen?
Je kan een functie schrijven in [encoding_funcs.rs](src/encoding_funcs.rs) om een van de coderingen te coderen die nog niet geschreven zijn, of je kan je eigen codering toevoegen.
Een lijst met alle coderingen die nog gedaan moeten worden vindt je in [todo.md](src/todo.md). Maar zoals ik al zei, mag je ook altijd zelf coderingen voorstellen.

**Heb je geen ervaring met Rust?**<br/>
In de toekomst zal ik misschien nog een link tussen Python en Rust toevoegen zodat functies ook in Python geschreven kunnen worden. 
Hulp hierbij is ook altijd welkom. 
Ken jij een andere programmeertaal en wil je helpen bij dit project? Laat het me weten, als er genoeg vraag naar is codeer ik nog meer linken zodat meer programmeertalen gebruikt kunnen worden.

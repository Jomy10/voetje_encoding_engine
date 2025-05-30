> [!CAUTION]
> This library has been replaced with [libcipher](https://github.com/Jomy10/libcipher). Libcipher has more functionality and is just as portable
> (or even more portable) than this library. This repository is therefore now archived.

# Voetje Encoding Engine
Deze code wordt door ['t Voetje](https://voetje.jonaseveraert.be) gebruikt om tekst te encoderen, zowel op iOS als op Android.

## Inhoudstafel
- [Hoe het werkt](#hoe-het-werkt)
- [Waarom Rust?](#waarom-in-rust)
- [Hoe kan ik helpen?](#hoe-kan-jij-helpen)
    - [Andere manieren om bij te dragen](#andere-manieren-om-bij-te-dragen)

## Hoe het werkt
Alle functies voor het encoderen worden geschreven in pure Rust en staan in [encoding_funcs.rs](src/encoding_funcs.rs).<br/>
Vervolgens wordt er gebruik gemaakt van `ffi` en `jni` zodat deze gebruikt kunnen worden in Swift (C) en Java (Swift voor iOS en Java voor Android). 

**Voorbeeld**<br/>
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
pub extern "C" fn jaar_free(cret: C_Return) { //... }

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

Alle comments in mijn code zijn geschreven in het Engels, uit gewoonte.

## Waarom in Rust?
Rust is veilig en zeer efficient. Deze Rust library kan gemakkelijk gecompileerd worden om gebruikt te worden op iOS en Android. Er is ook de mogelijkheid om te compileren naar WebAssembly en een heleboel andere programeertalen. Daarnaast heeft Rust ook een grote community die je altijd zal helpen en zijn er een heleboel open-source libraries beschikbaar.

## Hoe kan ik helpen?
Je kan een functie schrijven in [encoding_funcs.rs](src/encoding_funcs.rs) om een van de coderingen te coderen die nog niet geschreven zijn, of je kan je eigen codering toevoegen.
Een lijst met alle coderingen die nog gedaan moeten worden vindt je in [todo.md](src/todo.md). Maar zoals ik al zei, mag je ook altijd zelf coderingen voorstellen.

**Heb je geen ervaring met Rust?**<br/>
In de toekomst zal ik misschien nog een link tussen Python en Rust toevoegen zodat functies ook in Python geschreven kunnen worden. 
Hulp hierbij is ook altijd welkom. 
Ken jij een andere programmeertaal en wil je helpen bij dit project? Laat het me weten, als er genoeg vraag naar is codeer ik nog meer linken zodat meer programmeertalen gebruikt kunnen worden.

**Comments**<br/>
Voel je vrij om comments te schrijven in het Nederlands of het Engels.
Zorg er wel voor dat je code verstaanbaar is voor anderen, je kan nooit te veel comments schrijven!

**Functies**<br/>
De functies om te encoderen moeten String accepteren als input (en eventueel andere variabelen nodig voor de functie) en ze moeten een String teruggeven, eventueel ook andere variabelen. Zorg ervoor dat je code genoeg comments bevat over welke variabelen als input gebruikt worden en wat je functie teruggeven als output.

**Tests**<br/>
In [tests.rs](srs/tests.rs) vind je de tests voor elke functie. Deze zijn zeer gemakkelijk te schrijven in Rust en zeker voor dit soort van functies.

**Nog een laatste comment voordat je begint**<br/>
Het is voldoende om een functie te schrijven in [encoding_funcs.rs](src/encoding_funcs.rs). Ik zal dan wel de link functies schrijven in [lib.rs](src/lib.rs), maar voel je altijd vrij om deze ook zelf te schrijven.

### Andere manieren om bij te dragen
Heb je geen kennis van programmeren maar wil je toch bijdragen? Suggesties voor nieuwe coderingen zijn altijd welkom. Ook zijn sommige coderingen niet mogelijk met tekst (bijvoordbeeld: laddermethode, chinese cijfers en raamgeheimschrift). Voor deze coderingen zijn er lettertypes die je in de [fonts](fonts) folder kan vinden. Alle drie hebben wel een redesign nodig.<br/>
Nieuwe coderingen zijn ook altijd welkom, maar de voorkeur wordt gegeven aan tekst. Als je een nieuwe codering wil toevoegen, zet dan een bericht in de *Discussions* om te zien of deze codering geschikt is als afbeelding, of of deze toch beter als tekst geëncodeerd wordt.

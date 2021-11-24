//! This file contains the tests for the `encoding_funcs`.
//! 
//! Testen zijn belangrijk om zeker te zijn dat de functies
//! werken. Het is ook zeer gemakkelijk om er een te
//! voor dit soort van functies.

#[cfg(test)]
#[test]
/// Tests voor `jaartal`.
fn test_jaar() {
    let juiste_uitkomst = "GAJAOK NKRA UTHA DEEL ARTX DULX EGX LNX IAX / ";
    let uitkomst = crate::encoding_funcs::encode_jaar_uni("ga nu dadelijk terug naar het lokaal.", "1996").1;
    let test_result = uitkomst.eq(juiste_uitkomst);
    assert!(test_result);

    let juist2 = "DTSETS ÍIENET / ";
    let uikomst2 = crate::encoding_funcs::encode_jaar_uni("Dít is een test!", "2002").1;
    let test_result2 = uikomst2 == juist2;
    assert!(test_result2);

    let juist3 = "KT LX EX IX NX EX TX EX SX / ";
    let uitkomst3 = crate::encoding_funcs::encode_jaar_uni("kleine test :)", "9999").1;
    let test_result3 = uitkomst3 == juist3;
    assert!(test_result3);
}

#[cfg(test)]
#[test]
/// Tests voor `woord omkeren`.
fn test_woord_omkeren() {
    let juiste_uitkomst = "droow nerekmo.";
    let uitkomst = crate::encoding_funcs::encode_omkeren_uni("woord omkeren.");
    assert!(juiste_uitkomst == uitkomst);
}
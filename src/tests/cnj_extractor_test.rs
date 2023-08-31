#[path = "../cnj_extractor.rs"]
mod cnj_extractor;

use time::OffsetDateTime;

#[test]
fn test_extract_simple_cnj() {
    let input = "aqui está o cnj 0053087-35.2013.8.13.0693 veja bem";

    assert_eq!(
        cnj_extractor::extract(input),
        vec!["0053087-35.2013.8.13.0693"]
    )
}

#[test]
fn test_extract_cnj_without_dot_between_court_and_justice() {
    let input = "aqui está o cnj 0053087-35.2013.813.0693 veja bem";

    assert_eq!(
        cnj_extractor::extract(input),
        vec!["0053087-35.2013.8.13.0693"]
    )
}

#[test]
fn test_extract_multiple_cnjs() {
    let input = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod
        tempor invidunt ut labore et 0053087-35.2013.8.13.0693dolore magna
        aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo
        dolores et ea rebum. 0516710-11.2017.8.13.0000Stet clita kasd
        gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";

    assert_eq!(
        cnj_extractor::extract(input),
        vec!["0053087-35.2013.8.13.0693", "0516710-11.2017.8.13.0000"]
    )
}

#[test]
fn test_extract_cnj_without_hifen() {
    let input = "aqui está o cnj 005308735.2013.8.13.0693 veja bem";

    assert_eq!(
        cnj_extractor::extract(input),
        vec!["0053087-35.2013.8.13.0693"]
    );
}

#[test]
fn test_extract_cnj_without_punctuation() {
    let input = "aqui está o cnj 530873520138130693 veja bem";

    assert_eq!(
        cnj_extractor::extract(input),
        vec!["0053087-35.2013.8.13.0693"]
    );
}

#[test]
fn test_extract_cnj_without_left_zeros() {
    let input = "aqui está o cnj 5308735.2013.8.13.0693 veja bem";

    assert_eq!(
        cnj_extractor::extract(input),
        vec!["0053087-35.2013.8.13.0693"]
    );
}

#[test]
fn test_extract_cnj_with_line_break_and_missing_punctuation() {
    let input = "aqui está o cnj 53087\n3520138.13.0693 veja bem";

    assert_eq!(
        cnj_extractor::extract(input),
        vec!["0053087-35.2013.8.13.0693"]
    );
}

#[test]
fn test_extract_misbehaved_cnj() {
    let input = "aqui está o cnj 530873\n520138.13.0693 veja bem";

    assert_eq!(
        cnj_extractor::extract(input),
        vec!["0053087-35.2013.8.13.0693"]
    );
}

#[test]
fn test_extract_unique_cnjs() {
    let input = "0053087-35.2013.8.13.0693 texto\n                0516710-11.2017.8.13.0000 e 0516710-11.2017.8.13.0000";

    assert_eq!(
        cnj_extractor::extract(input),
        vec!["0053087-35.2013.8.13.0693", "0516710-11.2017.8.13.0000"]
    )
}

#[test]

fn test_extract_only_cnjs_with_valid_year() {
    let input = r"
        tempor invidunt ut labore et 0053087-35.2013.8.13.0693dolore magna
        dolores et ea rebum. 0000020-03.8100.0.04.2970Stet clita kasd
        tempor invidunt ut labore et 0064198-46.0013.9.24.1704dolore magna
    ";

    assert_eq!(
        cnj_extractor::extract(input),
        vec!["0053087-35.2013.8.13.0693"]
    )
}

#[test]
fn test_extract_cnj_with_current_year() {
    let input = format!("0053087-35.{}.8.13.0693", OffsetDateTime::now_utc().year());

    assert_eq!(cnj_extractor::extract(&input), vec![input.as_str()])
}

#[test]
fn test_extract_cnj_with_last_valid_year() {
    let input = format!("0053087-35.{}.8.13.0693", OffsetDateTime::now_utc().year() + 2);

    assert_eq!(cnj_extractor::extract(&input), vec![input.as_str()])
}

#[test]
fn test_extract_multiple_cnjs_from_excerpt() {
    let input = r"*** DGJUR - SECRETARIA DA 10ª CÂMARA CÍVEL *** ATO ORDINATÓRIO 010. AGRAVO DE \
            INSTRUMENTO - CÍVEL 0077777-12.2020.8.19.0000    R$       40552                      \
                                                                        Rio de Janeiro 24 de  \
            janeiro de 2020 \
            nº0011111-12.2020.8.19.0000 nª0022222-12.2020.8.19.0000 n°0033333-12.2020.8.19.0000";

    assert_eq!(
        cnj_extractor::extract(input),
        vec![
            "0077777-12.2020.8.19.0000",
            "0011111-12.2020.8.19.0000",
            "0022222-12.2020.8.19.0000",
            "0033333-12.2020.8.19.0000",
        ]
    )
}

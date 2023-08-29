use super::*;

#[test]
fn test_default_length() {
    assert_eq!(get_dashword(5,2,false).unwrap().len(), 8);
}

#[test]
fn test_default_with_zero_digits() {
    assert!(get_dashword(5,0,false).is_none());
}

#[test]
fn test_default_with_too_many_digits() {
    assert!(get_dashword(5,11,false).is_none());
}

#[test]
fn test_valid_length_inputs() {

    for i in 2..15 {
        assert!(get_dashword(i,2,false).is_some());
    }
}

#[test]
fn test_valid_length_outputs() {

    for i in 2..15 {
        assert_eq!(get_dashword(i,2,false).unwrap().len(), i+3);
    }
}

#[test]
fn test_too_short() {
    assert!(get_dashword(1,2,false).is_none());
}

#[test]
fn test_too_long() {
    assert!(get_dashword(16,2,false).is_none());
}




#[test]
fn test_default_with_zero_digits_simple() {
    assert!(get_dashword(5,0,true).is_none());
}

#[test]
fn test_default_with_too_many_digits_simple() {
    assert!(get_dashword(5,11,true).is_none());
}


#[test]
fn test_valid_length_inputs_simple() {

    for i in 2..10 {
        assert!(get_dashword(i,2,true).is_some());
    }
}

#[test]
fn test_valid_length_outputs_simple() {

    for i in 2..10 {
        assert_eq!(get_dashword(i,2,true).unwrap().len(), i+3);
    }
}

#[test]
fn test_too_short_simple() {
    assert!(get_dashword(1,2,true).is_none());
}

#[test]
fn test_too_long_simple() {
    assert!(get_dashword(10,2,true).is_none());
}

#[test]
fn test_correct_number_of_digits() {
    for i in 1..10 {
        assert_eq!(get_number(i).unwrap().len(), i);
    }
}

#[test]
fn test_get_number_returns_a_number() {
    for i in 1..10 {
        assert!(get_number(i).unwrap().parse::<usize>().is_ok());
    }
}

#[test]
fn test_get_curated_word_returns_correct_length() {
    for i in 2..10 {
        assert_eq!(get_curated_word(i).unwrap().len(), i);
    }
}

#[test]
fn test_get_curated_word_to_short() {
    assert!(get_curated_word(1).is_none());
}

#[test]
fn test_get_curated_word_is_zero() {
    assert!(get_curated_word(0).is_none());
}

#[test]
fn test_get_curated_word_to_long() {
    assert!(get_curated_word(11).is_none());
}




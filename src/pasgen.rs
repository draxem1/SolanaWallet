//Generates random passwords
//Called from newuser.rs
pub mod generator {
use rand::Rng;

    struct Elements {
        alphabet: [char;26],
        numbers: [char;10],
        special: [char;7],
        upper_case: [char;26],
    }

    pub fn call_gen() -> String{
        let password_length: u32 = rand::thread_rng().gen_range(8..24);
        let mut password = String::new();

            for _n in 1..password_length{
                password.push(random_character());
            }
        password
    }

    fn random_character() -> char {
        let selector: u32 = rand::thread_rng().gen_range(0..4);

        let elements = Elements {
            alphabet: ['a','b','c','d','e','f','g','h','i','j','k','l','m','n',
                        'o','p','q','r','s','t','u','v','w','x','y','z'],

            numbers: ['1','2','3','4','5','6','7','8','9','0'],

            special: ['!','@','$','%','&','=','?'],

            upper_case: ['A','B','C','D','E','F','G','H','I','J','K','L','O','M','N','P','Q','R'
                        ,'S','T','U','V','W','X','Y','Z'],
        };
        let end_range;

        match selector {
            0 => end_range = 26,
            1 => end_range = 10,
            2 => end_range = 7,
            3 => end_range = 26,
            _ => panic!("Index out of bounds!!!"),
        }

        let ind_of_char = rand::thread_rng().gen_range(0..end_range);
        let character;

        match selector {
            0 => character = &elements.alphabet[ind_of_char],
            1 => character = &elements.numbers[ind_of_char],
            2 => character = &elements.special[ind_of_char],
            3 => character = &elements.upper_case[ind_of_char],
            _ => panic!("Out of characters"),
        }

        *character
    }
}
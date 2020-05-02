use rand::Rng;

/// i don'T kNOW hOW To DeScRIbe THis FuNCTIOn
pub fn autistify(text: &str) -> String {
    let text = text.to_lowercase();
    let autistic_string = String::with_capacity(text.len());

    let mut rng = rand::thread_rng();
    text.chars().fold(autistic_string, |acc, c| {
        let c = if rng.gen() {
            c.to_uppercase().to_string()
        } else {
            c.to_string()
        };

        acc + &c
    })
}

/// I  D O N ' T  K N O W  H O W  T O  D E S C R I B E  T H I S  F U N C T I O N
pub fn shout(text: &str) -> String {
    let text = text.to_uppercase();
    let shouted = String::with_capacity(text.len() * 2);

    text.chars().fold(shouted, |acc, c| {
        if c.is_whitespace() {
            acc + &c.to_string()
        } else {
            acc + &format!("{} ", c)
        }
    })
}
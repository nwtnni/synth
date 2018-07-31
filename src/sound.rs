pub enum Sound {
    Seq(Vec<Sound>),
    Add(Box<Sound>, Box<Sound>),
    Sub(Box<Sound>, Box<Sound>),
    Clip(f64, Box<Sound>),
}

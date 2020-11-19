use ructe::{Ructe, RucteError};

fn main() -> Result<(), RucteError> {
    let mut ructe = Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("static")?;
    statics.add_files("static/js")?;
    statics.add_sass_file("static/css/spell_card.scss")?;
    statics.add_sass_file("static/css/index.scss")?;
    ructe.compile_templates("templates")
}

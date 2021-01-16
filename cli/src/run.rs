use crate::config::Options;

pub trait Runner {
    type Config: From<Options>;

    fn run(
        &mut self,
        dst: &mut dyn std::io::Write,
        config: Option<Self::Config>,
    ) -> Result<(), color_eyre::eyre::Report>;
}

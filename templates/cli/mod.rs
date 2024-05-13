use clap;
use icli_derive;

use crate::prelude::*;

mod {{ name }};

#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum {{ group }}Cmd {    {%- if has_c_attrs %}
    #[command({{ c_attrs }})]{% endif %}
    {{ name_v }}({{ name }}::{{ name_c }}Cmd),
}

pub struct ShadertoyShader {
    content: String,
}

impl ShadertoyShader {
    pub fn new(content: String) -> ShadertoyShader {
        let mut full = String::from(include_str!("res/fragmentHead.glsl"));
        full.push_str(content.as_str());
        full.push_str(include_str!("res/fragmentFooter.glsl"));

        ShadertoyShader { content: full }
    }

    pub fn as_glsl(&mut self) -> &str {
        &self.content
    }
}

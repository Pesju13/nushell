use crate::prompt_update::{POST_PROMPT_MARKER, PRE_PROMPT_MARKER};
#[cfg(windows)]
use nu_utils::enable_vt_processing;
use reedline::{
    DefaultPrompt, Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus,
    PromptViMode,
};
use std::borrow::Cow;

/// Nushell prompt definition
#[derive(Clone)]
pub struct NushellPrompt {
    shell_integration: bool,
    left_prompt_string: Option<String>,
    right_prompt_string: Option<String>,
    default_prompt_indicator: Option<String>,
    default_vi_insert_prompt_indicator: Option<String>,
    default_vi_normal_prompt_indicator: Option<String>,
    default_multiline_indicator: Option<String>,
    render_right_prompt_on_last_line: bool,
    
}

impl NushellPrompt {
    pub fn new(shell_integration: bool) -> NushellPrompt {
        NushellPrompt {
            shell_integration,
            left_prompt_string: None,
            right_prompt_string: None,
            default_prompt_indicator: None,
            default_vi_insert_prompt_indicator: None,
            default_vi_normal_prompt_indicator: None,
            default_multiline_indicator: None,
            render_right_prompt_on_last_line: false,
        }
    }

    pub fn update_prompt_left(&mut self, prompt_string: Option<String>) {
        self.left_prompt_string = prompt_string;
    }

    pub fn update_prompt_right(
        &mut self,
        prompt_string: Option<String>,
        render_right_prompt_on_last_line: bool,
    ) {
        self.right_prompt_string = prompt_string;
        self.render_right_prompt_on_last_line = render_right_prompt_on_last_line;
    }

    pub fn update_prompt_indicator(&mut self, prompt_indicator_string: Option<String>) {
        self.default_prompt_indicator = prompt_indicator_string;
    }

    pub fn update_prompt_vi_insert(&mut self, prompt_vi_insert_string: Option<String>) {
        self.default_vi_insert_prompt_indicator = prompt_vi_insert_string;
    }

    pub fn update_prompt_vi_normal(&mut self, prompt_vi_normal_string: Option<String>) {
        self.default_vi_normal_prompt_indicator = prompt_vi_normal_string;
    }

    pub fn update_prompt_multiline(&mut self, prompt_multiline_indicator_string: Option<String>) {
        self.default_multiline_indicator = prompt_multiline_indicator_string;
    }

    pub fn update_all_prompt_strings(
        &mut self,
        left_prompt_string: Option<String>,
        right_prompt_string: Option<String>,
        prompt_indicator_string: Option<String>,
        prompt_multiline_indicator_string: Option<String>,
        prompt_vi: (Option<String>, Option<String>),
        render_right_prompt_on_last_line: bool,
    ) {
        let (prompt_vi_insert_string, prompt_vi_normal_string) = prompt_vi;

        self.left_prompt_string = left_prompt_string;
        self.right_prompt_string = right_prompt_string;
        self.default_prompt_indicator = prompt_indicator_string;
        self.default_multiline_indicator = prompt_multiline_indicator_string;

        self.default_vi_insert_prompt_indicator = prompt_vi_insert_string;
        self.default_vi_normal_prompt_indicator = prompt_vi_normal_string;

        self.render_right_prompt_on_last_line = render_right_prompt_on_last_line;
    }
    #[allow(unused)]
    fn default_wrapped_custom_string(&self, str: String) -> String {
        format!("({str})")
    }
}

pub fn init_prompt_env() {
    unsafe {
        if let Ok(env) = std::env::var("NU_PROMPT") {
            let env = env.replace("$n", "\r\n");
            let sp: Vec<_> = env.splitn(2, "{prompt}").collect();
            println!();
            PROPMT = Some((sp[0].to_owned(), sp[1].to_owned()));
        }
    }
}

static mut PROPMT: Option<(String, String)> = None;
impl Prompt for NushellPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        #[cfg(windows)]
        {
            let _ = enable_vt_processing();
        }

        if let Some(prompt_string) = &self.left_prompt_string {
            let prompt = prompt_string.replace('\n', "\r\n");
            unsafe {
                #[allow(static_mut_refs)]
                if let Some(data) = &PROPMT {
                    let value = format!("{}{prompt}{}", data.0, data.1);
                    value
                } else {
                    prompt
                }
                .into()
            }
        } else {
            let default = DefaultPrompt::default();
            let prompt = default
                .render_prompt_left()
                .to_string()
                .replace('\n', "\r\n");
            if self.shell_integration {
                format!("{PRE_PROMPT_MARKER}{prompt}{POST_PROMPT_MARKER}").into()
            } else {
                prompt.into()
            }
        }
    }

    fn render_prompt_right(&self) -> Cow<str> {
        "".into()
    }

    fn render_prompt_indicator(&self, edit_mode: PromptEditMode) -> Cow<str> {
        let _: Cow<str> = match edit_mode {
            PromptEditMode::Default => match &self.default_prompt_indicator {
                Some(indicator) => indicator,
                None => "$ ",
            }
            .into(),
            PromptEditMode::Emacs => match &self.default_prompt_indicator {
                Some(indicator) => indicator,
                None => "$ ",
            }
            .into(),
            PromptEditMode::Vi(vi_mode) => match vi_mode {
                PromptViMode::Normal => match &self.default_vi_normal_prompt_indicator {
                    Some(indicator) => indicator,
                    None => "> ",
                },
                PromptViMode::Insert => match &self.default_vi_insert_prompt_indicator {
                    Some(indicator) => indicator,
                    None => ": ",
                },
            }
            .into(),
            PromptEditMode::Custom(str) => self.default_wrapped_custom_string(str).into(),
        };
        "$ ".into()
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        match &self.default_multiline_indicator {
            Some(indicator) => indicator,
            None => "::: ",
        }
        .into()
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };

        Cow::Owned(format!(
            "({}reverse-search: {})",
            prefix, history_search.term
        ))
    }

    fn right_prompt_on_last_line(&self) -> bool {
        self.render_right_prompt_on_last_line
    }
}

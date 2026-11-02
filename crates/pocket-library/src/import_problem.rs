//! What went wrong when a game was imported or started, in words a player can
//! act on. The explanations live in the import knowledge base
//! (`data/import_kb.json`, plus the library's own `import_kb.json`), so a new
//! kind of failure is explained by adding an entry there, not by a new release.

use crate::import_kb::KnowledgeBase;
use crate::LibraryError;

/// Which language to explain in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    ZhHant,
}

impl Lang {
    fn key(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::ZhHant => "zh-Hant",
        }
    }
}

/// A problem explained for a player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// The knowledge base entry that explains it, such as `no-game`.
    pub id: String,
    /// The technical message, for a bug report.
    pub detail: String,
    /// DLLs the game imports that nothing provides, if known.
    pub missing: Vec<String>,
    title: Vec<(String, String)>,
    hint: Vec<(String, String)>,
}

impl Problem {
    /// Explain why an import failed, with the built-in knowledge base.
    pub fn from_import(error: &LibraryError) -> Self {
        KnowledgeBase::builtin().explain_import(error)
    }

    /// Explain why a game failed to start or stopped, with the built-in
    /// knowledge base. `missing` is what the emulator's `missing_libraries`
    /// reported for it.
    pub fn from_run(message: &str, missing: &[String]) -> Self {
        KnowledgeBase::builtin().explain(message, missing)
    }

    pub(crate) fn new(
        id: String,
        detail: String,
        missing: Vec<String>,
        title: Vec<(String, String)>,
        hint: Vec<(String, String)>,
    ) -> Self {
        Self { id, detail, missing, title, hint }
    }

    fn pick(texts: &[(String, String)], lang: Lang) -> String {
        texts
            .iter()
            .find(|(key, _)| key == lang.key())
            .or_else(|| texts.iter().find(|(key, _)| key == "en"))
            .map(|(_, text)| text.clone())
            .unwrap_or_default()
    }

    /// What happened, in one sentence.
    pub fn title(&self, lang: Lang) -> String {
        Self::pick(&self.title, lang)
    }

    /// What to do about it.
    pub fn hint(&self, lang: Lang) -> String {
        Self::pick(&self.hint, lang).replace("{missing}", &self.missing.join(", "))
    }

    /// Title, hint and the technical detail as one message.
    pub fn message(&self, lang: Lang) -> String {
        let label = match lang {
            Lang::En => "Details",
            Lang::ZhHant => "技術細節",
        };
        format!("{}\n{}\n{label}: {}", self.title(lang), self.hint(lang), self.detail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_errors_are_explained_from_the_knowledge_base() {
        let cases = [
            (LibraryError::NoExecutable, "no-game"),
            (LibraryError::NotArmPe("setup.exe".into()), "desktop-program"),
            (LibraryError::NoPocketPcCab("x.exe".into()), "desktop-program"),
            (LibraryError::IsLibrary("a.dll".into()), "support-library"),
            (
                LibraryError::Cab(pocket_cab::CabError::Parse(
                    "Not a cabinet file (invalid file signature)".into(),
                )),
                "not-a-cab",
            ),
            (
                LibraryError::Io(std::io::Error::new(std::io::ErrorKind::StorageFull, "disk full")),
                "no-space",
            ),
        ];
        for (error, id) in cases {
            let problem = Problem::from_import(&error);
            assert_eq!(problem.id, id, "{error}");
            for lang in [Lang::En, Lang::ZhHant] {
                assert!(!problem.title(lang).is_empty());
                assert!(!problem.hint(lang).is_empty());
                assert!(problem.message(lang).contains(&error.to_string()));
            }
        }
    }

    #[test]
    fn a_missing_rom_dll_is_named_in_the_hint() {
        let problem = Problem::from_run("cpu error: WRITE unmapped at 0x14", &["mfcce300.dll".into()]);
        assert_eq!(problem.id, "missing-rom-library");
        assert!(problem.hint(Lang::ZhHant).contains("mfcce300.dll"));
    }

    #[test]
    fn run_errors_are_told_apart() {
        let id = |message: &str| Problem::from_run(message, &[]).id;
        assert_eq!(id("managed PE requires a .NET Compact Framework runtime"), "dotnet");
        assert_eq!(id("emu_start: unsupported instruction 0xafbf0014"), "other-processor");
        assert_eq!(id("cpu error: EXCEPTION (READ unmapped) at 0x0"), "crashed");
        assert_eq!(id("something new"), "unknown");
    }
}

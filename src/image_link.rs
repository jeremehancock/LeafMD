//! Turns an image dropped onto the raw text into a markdown image link, like
//! `![beach day](images/beach%20day.jpg)`.
//!
//! Like `typing.rs`, this works on plain paths with no window involved, so it
//! can be tested on its own. `document.rs` handles the drop itself and inserts
//! what this returns.

use std::path::{Component, Path};

use gtk::gio;

/// Why no link can be made for a dropped image.
#[derive(Debug, PartialEq, Eq)]
pub enum CantLink {
    /// The document has never been saved, so there's no folder to link from.
    Unsaved,
    /// LeafMD can't see the image where it says it is, or only through the
    /// document portal: a stand-in path that only works inside LeafMD's
    /// sandbox, and only for a while. Both happen for files outside the home
    /// folder. In the Flatpak, `/tmp` is LeafMD's own empty folder, so an
    /// image from the real `/tmp` arrives as a plain path that leads nowhere.
    OutsideHome,
}

impl CantLink {
    /// The message shown to the user.
    pub fn message(&self) -> &'static str {
        match self {
            CantLink::Unsaved => "Save the document first, so LeafMD knows where to link the image from",
            CantLink::OutsideHome => "LeafMD can only link to images in your home folder",
        }
    }
}

/// Whether a file is an image, judged by its name only (nothing is read), the
/// same way the Files panel decides which files to grey out.
pub fn is_image(name: &Path) -> bool {
    gio::content_type_guess(Some(name), None).0.starts_with("image/")
}

/// The markdown image link for `image`, written from the folder of
/// `document` (None if it's never been saved). `portal` is the folder the
/// document portal shares files through, whose paths make broken links.
/// `image_visible` says whether LeafMD can see a file at `image`.
pub fn link_for(
    document: Option<&Path>,
    image: &Path,
    image_visible: bool,
    portal: &Path,
) -> Result<String, CantLink> {
    let document = document.ok_or(CantLink::Unsaved)?;
    if !image_visible || document.starts_with(portal) || image.starts_with(portal) {
        return Err(CantLink::OutsideHome);
    }
    let folder = document.parent().unwrap_or(Path::new("/"));
    let description = image
        .file_stem()
        .map(|stem| escape_description(&stem.to_string_lossy()))
        .unwrap_or_default();
    Ok(format!("![{description}]({})", relative_address(folder, image)))
}

/// The image's path from `folder`, with `../` for each folder to go up, and
/// each part encoded so it can't break the link.
fn relative_address(folder: &Path, image: &Path) -> String {
    // Only the folder names matter: "/" and "." add nothing to compare
    let parts = |path: &Path| -> Vec<String> {
        path.components()
            .filter_map(|part| match part {
                Component::Normal(name) => Some(name.to_string_lossy().into_owned()),
                Component::ParentDir => Some("..".to_string()),
                _ => None,
            })
            .collect()
    };
    let from = parts(folder);
    let to = parts(image);
    let shared = from.iter().zip(&to).take_while(|(a, b)| a == b).count();
    let mut address: Vec<String> = vec!["..".to_string(); from.len() - shared];
    address.extend(to[shared..].iter().map(|part| encode(part)));
    address.join("/")
}

/// Writes the characters that would end or break a link address as `%xx`:
/// spaces, brackets, angle brackets, `%` itself, backslashes and control
/// characters, plus `#` and `?`, which in a web address start the "jump to
/// this spot" and "question" parts, so `#1.jpg` would be read as no file at
/// all. Everything else, including letters from any language, stays
/// readable.
fn encode(part: &str) -> String {
    let mut encoded = String::new();
    for c in part.chars() {
        if c == ' ' || "()<>%\\#?".contains(c) || c.is_control() {
            let mut bytes = [0; 4];
            for byte in c.encode_utf8(&mut bytes).bytes() {
                encoded.push_str(&format!("%{byte:02X}"));
            }
        } else {
            encoded.push(c);
        }
    }
    encoded
}

/// Puts a backslash before characters that would end the description early.
fn escape_description(name: &str) -> String {
    let mut escaped = String::new();
    for c in name.chars() {
        if matches!(c, '[' | ']' | '\\') {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    const PORTAL: &str = "/run/user/1000/doc";
    const DOC: &str = "/home/sam/notes/trip.md";

    fn link(image: &str) -> String {
        link_for(Some(Path::new(DOC)), Path::new(image), true, Path::new(PORTAL)).unwrap()
    }

    #[test]
    fn same_folder() {
        assert_eq!(link("/home/sam/notes/beach.png"), "![beach](beach.png)");
    }

    #[test]
    fn subfolder() {
        assert_eq!(link("/home/sam/notes/images/photo.png"), "![photo](images/photo.png)");
    }

    #[test]
    fn neighbouring_folder() {
        assert_eq!(link("/home/sam/Pictures/photo.png"), "![photo](../Pictures/photo.png)");
    }

    #[test]
    fn several_levels_up() {
        let doc = Path::new("/home/sam/a/b/c/notes.md");
        let image = Path::new("/home/sam/x/photo.png");
        assert_eq!(
            link_for(Some(doc), image, true, Path::new(PORTAL)).unwrap(),
            "![photo](../../../x/photo.png)"
        );
    }

    #[test]
    fn spaces() {
        assert_eq!(link("/home/sam/notes/beach day.jpg"), "![beach day](beach%20day.jpg)");
        assert_eq!(
            link("/home/sam/my pics/a b.png"),
            "![a b](../my%20pics/a%20b.png)"
        );
    }

    #[test]
    fn characters_that_break_links() {
        assert_eq!(link("/home/sam/notes/plan (v2).png"), "![plan (v2)](plan%20%28v2%29.png)");
        assert_eq!(link("/home/sam/notes/100%.png"), "![100%](100%25.png)");
        assert_eq!(link("/home/sam/notes/<a>.png"), "![<a>](%3Ca%3E.png)");
    }

    #[test]
    fn hash_and_question_marks() {
        // A real file name that made a broken link before # was encoded
        assert_eq!(
            link("/home/sam/notes/#00080, tour 10-45am on 09-08-2015, portrait, print single.jpg"),
            "![#00080, tour 10-45am on 09-08-2015, portrait, print single]\
             (%2300080,%20tour%2010-45am%20on%2009-08-2015,%20portrait,%20print%20single.jpg)"
        );
        assert_eq!(link("/home/sam/notes/why?.png"), "![why?](why%3F.png)");
    }

    #[test]
    fn square_brackets_in_the_description() {
        assert_eq!(link("/home/sam/notes/[draft].png"), "![\\[draft\\]]([draft].png)");
    }

    #[test]
    fn no_extension_and_two_dots() {
        assert_eq!(link("/home/sam/notes/photo"), "![photo](photo)");
        assert_eq!(link("/home/sam/notes/photo.final.png"), "![photo.final](photo.final.png)");
    }

    #[test]
    fn letters_from_other_languages_stay_readable() {
        assert_eq!(link("/home/sam/notes/café.png"), "![café](café.png)");
    }

    #[test]
    fn unsaved_document() {
        let image = Path::new("/home/sam/notes/beach.png");
        assert_eq!(link_for(None, image, true, Path::new(PORTAL)), Err(CantLink::Unsaved));
    }

    #[test]
    fn stand_in_paths() {
        let shared = Path::new("/run/user/1000/doc/abc123/beach.png");
        assert_eq!(
            link_for(Some(Path::new(DOC)), shared, true, Path::new(PORTAL)),
            Err(CantLink::OutsideHome)
        );
        let shared_doc = Path::new("/run/user/1000/doc/abc123/trip.md");
        let image = Path::new("/home/sam/notes/beach.png");
        assert_eq!(
            link_for(Some(shared_doc), image, true, Path::new(PORTAL)),
            Err(CantLink::OutsideHome)
        );
    }

    #[test]
    fn image_leafmd_cant_see() {
        // An image dropped from the real /tmp, which the Flatpak hides
        let image = Path::new("/tmp/beach.png");
        assert_eq!(
            link_for(Some(Path::new(DOC)), image, false, Path::new(PORTAL)),
            Err(CantLink::OutsideHome)
        );
    }

    #[test]
    fn images_by_name() {
        for name in ["a.png", "b.JPG", "c.jpeg", "d.svg", "e.webp", "f.gif"] {
            assert!(is_image(Path::new(name)), "{name} should be an image");
        }
        for name in ["notes.md", "README", "data.json"] {
            assert!(!is_image(Path::new(name)), "{name} shouldn't be an image");
        }
    }
}

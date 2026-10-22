//! Word count: counts the words in each scene, for the whole story or the scenes picked, and replies with
//! a document the writer can copy or save as a note.

use hammer::action::{Reply, Request};
use hammer::ops::{self, SceneKind, SceneNode};
use serde::Deserialize;

/// The action's first page: the manifest's `[[actions.field]]`.
#[derive(Deserialize, Default)]
struct CountInput {
    #[serde(default)]
    scenes: Vec<i32>,
}

/// Hammer calls this for every action in the manifest; tell them apart by name.
#[unsafe(no_mangle)]
pub extern "C" fn action() -> i32 {
    hammer::run(|| {
        let request: Request = hammer::input_json()?;
        match request.action.as_str() {
            "count" => count_words(&request)?.to_json(),
            _ => Reply::default().to_json(),
        }
    })
}

/// Counts the scenes picked, or the scene it was run from, or else the whole story.
fn count_words(request: &Request) -> hammer::Result<Reply> {
    let mut chosen = request.input::<CountInput>()?.scenes;
    if chosen.is_empty() && request.context.place == "scene" {
        chosen.extend(request.context.id.map(|id| id as i32));
    }
    let tree = ops::scene::tree(&request.project)?;
    let scenes = pick(&tree.nodes, &chosen);
    if scenes.is_empty() {
        return Ok(Reply::message("There are no scenes to count"));
    }

    let mut counts = Vec::new();
    for (i, scene) in scenes.iter().enumerate() {
        hammer::progress(Some(i as f64 / scenes.len() as f64), Some(&format!("Counting {}", scene.name)));
        let text = ops::scene::read(&request.project, scene.id)?;
        counts.push((scene.name.clone(), words_in(&text.markdown)));
    }
    let mut reply = Reply::document(report(&counts));
    if let Some(page) = reply.page.as_mut() {
        page.title = Some("Word count".into());
    }
    Ok(reply)
}

/// The story's scenes in order, groups opened up: those in `chosen`, or all of them when it's empty.
fn pick<'a>(nodes: &'a [SceneNode], chosen: &[i32]) -> Vec<&'a SceneNode> {
    let mut scenes = Vec::new();
    for node in nodes {
        if node.kind == SceneKind::Scene {
            if chosen.is_empty() || chosen.contains(&node.id) {
                scenes.push(node);
            }
        } else {
            scenes.extend(pick(&node.children, chosen));
        }
    }
    scenes
}

fn report(counts: &[(String, usize)]) -> String {
    let mut markdown = String::from("# Word count\n\n");
    for (name, words) in counts {
        markdown += &format!("- {name}: {words}\n");
    }
    if counts.len() > 1 {
        markdown += &format!("\n**Total: {}**\n", counts.iter().map(|(_, words)| words).sum::<usize>());
    }
    markdown
}

/// Words are runs that hold at least one letter or digit, so dashes and stray punctuation don't count.
fn words_in(text: &str) -> usize {
    text.split_whitespace().filter(|word| word.chars().any(char::is_alphanumeric)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scene(id: i32, name: &str) -> SceneNode {
        SceneNode { id, name: name.into(), ..Default::default() }
    }

    // The kit's types fill in what a test leaves out, as Hammer's own replies never do.
    fn story() -> Vec<SceneNode> {
        vec![SceneNode {
            id: 1,
            name: "Part One".into(),
            kind: SceneKind::Group,
            children: vec![scene(2, "Storm"), scene(3, "Calm")],
            ..Default::default()
        }]
    }

    #[test]
    fn picks_every_scene_or_those_chosen() {
        let story = story();
        let names = |chosen: &[i32]| pick(&story, chosen).iter().map(|s| s.name.clone()).collect::<Vec<_>>();
        assert_eq!(names(&[]), ["Storm", "Calm"]);
        assert_eq!(names(&[3]), ["Calm"]);
    }

    #[test]
    fn reports_each_scene_with_a_total() {
        let counts = [("Storm".to_string(), 3), ("Calm".to_string(), 1)];
        assert_eq!(report(&counts), "# Word count\n\n- Storm: 3\n- Calm: 1\n\n**Total: 4**\n");
        assert_eq!(report(&counts[..1]), "# Word count\n\n- Storm: 3\n");
    }

    #[test]
    fn words_are_runs_of_letters_and_digits() {
        assert_eq!(words_in("She said — twice — “no.”"), 4);
    }
}

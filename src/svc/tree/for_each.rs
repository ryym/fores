use serde_json::Value as JsonValue;

register_service!(ForEach);

pub trait ForEach {
    fn for_each_tree<F, T>(&self, tree: &JsonValue, init: T, mut f: F)
    where
        T: Clone,
        F: FnMut(&str, &JsonValue, T) -> T,
    {
        traverse(tree, "", init, &mut f);
    }
}

fn traverse<F, T>(tree: &JsonValue, name: &str, state: T, f: &mut F)
where
    T: Clone,
    F: FnMut(&str, &JsonValue, T) -> T,
{
    let next_state = f(name, tree, state);
    let tree = tree.as_object().unwrap();
    for (name, subtree) in tree.iter() {
        if name == "..id" {
            continue;
        }
        traverse(subtree, name, next_state.clone(), f);
    }
}

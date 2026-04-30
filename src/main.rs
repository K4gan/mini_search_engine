use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Document {
    id: String,
    body: String,
}

fn index(documents: &[Document]) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for doc in documents {
        for token in doc.body.split_whitespace().map(|word| word.to_lowercase()) {
            out.entry(token).or_default().push(doc.id.clone());
        }
    }
    out
}

fn main() {
    let docs = vec![
        Document { id: "runbook".into(), body: "cache latency incident".into() },
        Document { id: "notes".into(), body: "latency probe metrics".into() },
    ];
    let idx = index(&docs);
    println!("{:?}", idx.get("latency").cloned().unwrap_or_default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_index() {
        let docs = vec![Document { id: "a".into(), body: "one two".into() }];
        assert_eq!(index(&docs).get("one").unwrap(), &vec!["a".to_string()]);
    }
}

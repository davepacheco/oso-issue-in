use anyhow::Context;
use oso::{Oso, PolarClass};

#[derive(Clone, Copy, PolarClass)]
struct User {
    #[polar(attribute)]
    name: &'static str,
}

#[derive(Clone, Copy, PolarClass)]
struct Doc {
    #[polar(attribute)]
    id: Option<u8>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut oso = Oso::new();

    oso.register_class(User::get_polar_class())?;
    oso.register_class(Doc::get_polar_class())?;
    oso.load_str(
        r#"
        actor User {}

        resource Doc {
            permissions = [ "edit" ];
        }

        has_permission(actor: User, "edit", doc: Doc)
            if actor.name = "u1" and 2 in doc.id;
        has_permission(actor: User, "edit", doc: Doc)
            if actor.name = "u2" and doc.id = 2;
        has_permission(actor: User, "edit", doc: Doc)
            if actor.name = "u3" and doc.id.unwrap() = 2;

        allow(actor: Actor, action: String, resource: Resource) if
            has_permission(actor, action, resource);
    "#,
    )
    .context("loading policy file")?;

    let u1 = User { name: "u1" };
    let u2 = User { name: "u2" };
    let u3 = User { name: "u3" };
    let doc_none = Doc { id: None };
    let doc_one = Doc { id: Some(1) };
    let doc_two = Doc { id: Some(2) };

    // None of these rules works for doc_none.  The rule for "u3" even fails
    // altogether because it unwraps a None value.
    assert!(!oso.is_allowed(u1, "edit", doc_none).unwrap());
    assert!(!oso.is_allowed(u2, "edit", doc_none).unwrap());
    // let error = oso.is_allowed(u3, "edit", doc_none);
    // eprintln!("rule that unwraps None produced: {:?}", error);
    // assert!(matches!(error, Err(_)));

    // None of these rules works for doc_one because it has the wrong id.
    assert!(!oso.is_allowed(u1, "edit", doc_one).unwrap());
    assert!(!oso.is_allowed(u2, "edit", doc_one).unwrap());
    assert!(!oso.is_allowed(u3, "edit", doc_one).unwrap());

    // Which of these rules works for doc_two?
    eprintln!("{:?}", oso.is_allowed(u1, "edit", doc_two));
    eprintln!("{:?}", oso.is_allowed(u2, "edit", doc_two));
    eprintln!("{:?}", oso.is_allowed(u3, "edit", doc_two));
    Ok(())
}

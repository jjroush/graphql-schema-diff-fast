use graphql_schema_diff::{diff, Change, ChangeKind};
use std::fs;

fn filter_changes(changes: &[Change]) -> Vec<Change> {
    changes
        .iter()
        .filter(|change| {
            matches!(
                change.kind,
                ChangeKind::ChangeFieldType | ChangeKind::RemoveEnumValue
            )
        })
        .cloned()
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string("src/source.graphql")?;
    let target = fs::read_to_string("src/target.graphql")?;

    let changes = diff(&source, &target).unwrap();

    let filtered_changes = filter_changes(&*changes);

    for change in filtered_changes {
        println!("- Path: {}, Kind: {:?}", change.path, change.kind);
    }

    // assert_eq!(changes,
    //            &[
    //                Change {
    //                    path: String::from("Pizza.name"),
    //                    kind: ChangeKind::ChangeFieldType
    //                },
    //                Change {
    //                    path: String::from("PizzaName"),
    //                    kind: ChangeKind::AddObjectType
    //                },
    //                Change {
    //                    path: String::from("Topping.PINEAPPLE"),
    //                    kind: ChangeKind::RemoveEnumValue
    //                },
    //                Change {
    //                    path: String::from("Topping.POTATO"),
    //                    kind: ChangeKind::AddEnumValue
    //                }
    //            ]);

    Ok(())
}
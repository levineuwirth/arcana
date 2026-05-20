//! Brackish Blunder — `{1}{U}` instant. "Return target creature to its
//! owner's hand. If it was tapped, create a Map token."
//!
//! GAP: the target's tapped state cannot be queried after the bounce; the
//! Map token is created unconditionally as best-effort.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brackish Blunder");
    let _map = reg.interner_mut().intern("Map");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature to its owner's hand. If it was tapped, create a Map token.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let map = reg.interner().lookup("Map").expect("Map interned during register()");
    let token = TokenDefinition {
        name: map,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: SubtypeSet::default(),
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "if it was tapped" condition not queryable post-bounce.
    vec![
        Effect::ReturnToHand { target: *id },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}

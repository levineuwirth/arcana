//! Lost Days — `{4}{U}` instant — Lesson. "The owner of target creature or
//! enchantment puts it into their library second from the top or on the
//! bottom. You create a Clue token."
//!
//! GAP: no "library second from top or bottom (owner's choice)" placement;
//! modeled as put-on-bottom-of-library. Target restricted to a creature
//! (no creature-or-enchantment combined filter).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lost Days");
    let _clue = reg.interner_mut().intern("Clue");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::INSTANT),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "The owner of target creature or enchantment puts it into their library second from the top or on the bottom. You create a Clue token.".into(),
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
    let clue = reg.interner().lookup("Clue").expect("Clue interned during register()");
    let token = TokenDefinition {
        name: clue,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: SubtypeSet::default(),
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "library second from top or bottom" placement choice.
    vec![
        Effect::PutOnBottomOfLibrary { target: *id },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}

//! Brood Birthing — `{1}{R}` sorcery. "If you control an Eldrazi
//! Spawn, create three 0/1 colorless Eldrazi Spawn creature tokens.
//! They have 'Sacrifice this token: Add {C}.' Otherwise, create one of
//! those tokens."
//!
//! "If you control an Eldrazi Spawn" is checked via subtype_filter
//! count; create three tokens if so, else one.
//!
//! GAP: the token's "Sacrifice this token: Add {C}" activated ability
//! is not expressible on a TokenDefinition.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brood Birthing");
    let _spawn = reg.interner_mut().intern("Eldrazi Spawn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "If you control an Eldrazi Spawn, create three 0/1 colorless Eldrazi Spawn creature tokens. They have \"Sacrifice this token: Add {C}.\" Otherwise, create one of those tokens.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let spawn = reg.interner().lookup("Eldrazi Spawn").expect("Eldrazi Spawn interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spawn);
    let token = TokenDefinition {
        name: spawn,
        colors: ColorSet::new(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    let has_spawn = script::count_matching(
        state,
        &script::subtype_filter(reg, "Eldrazi Spawn").controlled_by(ControllerConstraint::You),
        entry.controller,
    ) > 0;
    // GAP: token's "Sacrifice this token: Add {C}" mana ability not expressible.
    if has_spawn {
        vec![
            Effect::CreateToken { controller: entry.controller, token: token.clone() },
            Effect::CreateToken { controller: entry.controller, token: token.clone() },
            Effect::CreateToken { controller: entry.controller, token },
        ]
    } else {
        vec![Effect::CreateToken { controller: entry.controller, token }]
    }
}

//! An Offer You Can't Refuse — `{U}` instant. "Counter target noncreature spell.
//! Its controller creates two Treasure tokens."
//!
//! # GAP: Treasure token (activated-ability artifact token with mana-producing sacrifice)
//! The engine's TokenDefinition has no built-in Treasure activated ability.
//! Counter is fully expressible; Treasure creation is approximated as two
//! generic colorless artifact tokens (no activated ability).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("An Offer You Can't Refuse");
    let _treasure = reg.interner_mut().intern("Treasure");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target noncreature spell. Its controller creates two Treasure tokens.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(
                        ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    let stack_id = match target {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    // Determine the spell's controller to give Treasures to.
    // GAP: no way to read the target spell's controller from StackEntry via the catalog API.
    // We give the Treasures to entry.controller as best approximation.
    let treasure = reg.interner().lookup("Treasure").expect("Treasure interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treasure);
    let token = TokenDefinition {
        name: treasure,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::Counter { target: stack_id },
        // GAP: Treasure token activated ability ("{T}, Sacrifice: Add one mana of any color") not expressible
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}

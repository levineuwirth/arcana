//! Pyretic Rebirth — `{2}{B}{R}` instant. "Return target artifact or
//! creature card from your graveyard to your hand. Pyretic Rebirth
//! deals damage equal to that card's mana value to up to one target
//! creature or planeswalker."
//!
//! GAP: the second clause's damage is dynamic on the first target
//! card's mana value, but there is no script helper for a target
//! object's mana value, and the second "up to one" target can't be
//! coupled to the first card's CMC. The graveyard return is emitted;
//! the conditional damage is gapped rather than emit a wrong literal.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pyretic Rebirth");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return target artifact or creature card from your graveyard to your hand. Pyretic Rebirth deals damage equal to that card's mana value to up to one target creature or planeswalker.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new().with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: "damage equal to that card's mana value to up to one target" — no target-CMC helper.
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}

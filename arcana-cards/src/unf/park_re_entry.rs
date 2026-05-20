//! Park Re-Entry — `{3}{W}{W}` sorcery. "Return up to two target
//! creature cards that each have a hat and/or mana value 3 or less
//! from your graveyard to the battlefield."
//!
//! GAP: 'has a hat' is flavor-only and 'mv 3 or less' restriction on
//! a TargetFilter::Card isn't expressible (with_max_cmc applies to
//! battlefield ObjectFilter, not Card-zone targets — its semantics on
//! graveyard cards aren't guaranteed in spec). We model two
//! up-to-two creature-graveyard reanimates with the cmc cap applied
//! via the filter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Park Re-Entry");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return up to two target creature cards that each have a hat and/or mana value 3 or less from your graveyard to the battlefield.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature().with_max_cmc(3),
                },
                count: TargetCount::UpTo(2),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for t in entry.targets.targets.iter() {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
        }
    }
    // GAP: 'has a hat' flavor union with the mv-3-or-less restriction.
    effects
}

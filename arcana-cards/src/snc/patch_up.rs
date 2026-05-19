//! Patch Up — `{2}{W}` Sorcery. "Return up to three target creature
//! cards each with mana value 3 or less from your graveyard to the
//! battlefield."
//!
//! # Implementation note
//! ReturnFromGraveyardToBattlefield is expressible for a single
//! target. The "up to 3" multi-target and the MV <= 3 constraint
//! are not fully enforceable.
//!
//! # GAP
//! UpTo(3) multi-target from graveyard and MV-constraint filtering not
//! fully expressible; single-target approximation used.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Patch Up");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to three target creature cards each with mana value 3 or less from your graveyard to the battlefield.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::UpTo(3),
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
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: MV <= 3 constraint not enforced on targets; multi-target iteration approximate
    entry.targets.targets.iter().filter_map(|t| {
        if let arcana_core::targets::TargetChoice::Object(id) = t {
            Some(Effect::ReturnFromGraveyardToBattlefield { target: *id })
        } else {
            None
        }
    }).collect()
}

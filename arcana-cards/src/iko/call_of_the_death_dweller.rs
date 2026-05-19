//! Call of the Death-Dweller — `{2}{B}` sorcery. "Return up to two target
//! creature cards with total mana value 3 or less from your graveyard to the
//! battlefield. Put a deathtouch counter on either of them. Then put a menace
//! counter on either of them."
//!
//! GAP: total-MV-3-or-less constraint on two graveyard targets, and deathtouch
//! / menace counters (CounterKind only supports PlusOnePlusOne) are not
//! expressible. Best effort: return up to two creature cards from graveyard.

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
    let name = reg.interner_mut().intern("Call of the Death-Dweller");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to two target creature cards with total mana value 3 or less from your graveyard to the battlefield. Put a deathtouch counter on either of them. Then put a menace counter on either of them.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() },
                    count: TargetCount::UpTo(2),
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
    // GAP: total-MV constraint, deathtouch counter, menace counter not supported
    entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::ReturnFromGraveyardToBattlefield { target: *id })
        } else {
            None
        }
    }).collect()
}

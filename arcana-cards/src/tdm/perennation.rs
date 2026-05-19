//! Perennation — `{3}{W}{B}{G}` sorcery, "Return target permanent card from
//! your graveyard to the battlefield with a hexproof counter and an
//! indestructible counter on it."
//!
//! GAP: hexproof counter and indestructible counter types are not in
//! CounterKind; only PlusOnePlusOne is available. The reanimate effect
//! is expressed; the counters are noted as gaps.

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
    let name = reg.interner_mut().intern("Perennation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target permanent card from your graveyard to the battlefield with a hexproof counter and an indestructible counter on it.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::permanent(),
                    },
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
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: hexproof counter kind not in CounterKind
    // GAP: indestructible counter kind not in CounterKind
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}

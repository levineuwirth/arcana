//! Lie in Wait — `{B}{G}{U}` sorcery. "Return target creature card from
//! your graveyard to the battlefield, then Lie in Wait deals damage equal
//! to that card's power to target creature or player."
//!
//! # GAP: damage equal to returned card's power (variable game-state
//! lookup at resolve time).

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
    let name = reg.interner_mut().intern("Lie in Wait");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature card from your graveyard to the battlefield, then Lie in Wait deals damage equal to that card's power to target creature or player.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::creature(),
                        },
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::any_target(),
                ],
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
    let Some(gy_target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(gy_id) = gy_target else { return Vec::new(); };
    // GAP: damage equal to returned card's power not expressible
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *gy_id }]
}

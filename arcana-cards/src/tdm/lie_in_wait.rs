//! Lie in Wait — `{B}{G}{U}` sorcery. "Return target creature card
//! from your graveyard to your hand. Lie in Wait deals damage equal
//! to that card's power to target creature."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return target creature card from your graveyard to your hand. Lie in Wait deals damage equal to that card's power to target creature.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut targets = entry.targets.targets.iter();
    let Some(TargetChoice::Object(gy_card)) = targets.next() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(creature)) = targets.next() else {
        return Vec::new();
    };
    // Read the graveyard card's power before it leaves the graveyard.
    let power = script::power_of(state, *gy_card).max(0) as u32;
    vec![
        Effect::ReturnFromGraveyardToHand { target: *gy_card },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*creature),
            amount: power,
        },
    ]
}

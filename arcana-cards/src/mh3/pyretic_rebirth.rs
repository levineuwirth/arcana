//! Pyretic Rebirth — `{2}{B}{R}` instant, "Return target artifact or
//! creature card from your graveyard to your hand. Pyretic Rebirth deals
//! damage equal to that card's mana value to up to one target creature or
//! planeswalker."
//! GAP: querying the mana value of a graveyard card at resolution time
//! (script helpers do not expose cmc/mana-value of a graveyard object).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target artifact or creature card from your graveyard to your hand. Pyretic Rebirth deals damage equal to that card's mana value to up to one target creature or planeswalker.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::new()
                                .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                        },
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
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
    let Some(graveyard_target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(gy_id) = graveyard_target else { return Vec::new(); };
    // GAP: mana value of a graveyard card not accessible via script helpers
    let mut effects = vec![Effect::ReturnFromGraveyardToHand { target: *gy_id }];
    if let Some(creature_target) = entry.targets.targets.get(1) {
        if let TargetChoice::Object(cid) = creature_target {
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(*cid),
                amount: 0, // GAP: should be mana value of gy_id
            });
        }
    }
    effects
}

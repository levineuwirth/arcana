//! Havi, the All-Father — `{3}{R}{G}{W}` 6/6 Legendary God Warrior.
//!
//! * Havi has indestructible as long as there are four or more historic
//!   cards in your graveyard. (Static; "historic" filter has no
//!   expressible predicate — GAP'd.)
//! * Sage Project — Whenever Havi or another legendary creature you
//!   control dies, return target legendary creature card with lesser
//!   mana value from your graveyard to the battlefield tapped.
//!
//! The death trigger is modeled as a ZoneChange (battlefield →
//! graveyard) of a legendary creature you control. The "lesser mana
//! value" restriction (relative to the dying creature) and the "tapped"
//! re-entry rider are not expressible, so both are documented GAPs.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Havi, the All-Father");
    let god = reg.interner_mut().intern("God");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: static "indestructible as long as four or more historic cards
    // in your graveyard" — no expressible historic-card filter / static.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: reanimate_legend,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: "with lesser mana value" (relative to the dying
            // creature) not expressible; target is any legendary
            // creature card in your graveyard.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature()
                        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn reanimate_legend(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "tapped" re-entry rider not expressible (no tapped variant of
    // ReturnFromGraveyardToBattlefield).
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}

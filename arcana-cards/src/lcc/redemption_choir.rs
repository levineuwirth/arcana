//! Redemption Choir — `{2}{W}{W}` 3/3 Vampire Cleric.
//! Lifelink.
//! Coven — Whenever this creature enters or attacks, if you control three or
//! more creatures with different powers, return target permanent card with mana
//! value 3 or less from your graveyard to the battlefield.
//!
//! Lifelink is a base characteristic. The Coven ability is split into two
//! triggers (enters, attacks), each returning a targeted permanent card with
//! mv ≤ 3 from the graveyard. The Coven intervening-if ("three or more
//! creatures with different powers") has no condition helper, so it is GAPped
//! (fires unconditionally — strictly broader than printed).

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

fn return_req() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::permanent()
                .with_max_cmc(3)
                .controlled_by(ControllerConstraint::You),
        },
        count: TargetCount::Exactly(1),
        controller: None,
    }
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Redemption Choir");
    let vampire = reg.interner_mut().intern("Vampire");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: intervening-if "if you control three or more creatures with
            // different powers" (Coven) — no condition helper for "different
            // powers"; fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: coven_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![return_req()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: coven_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![return_req()],
            }),
    )
}

fn coven_return(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}

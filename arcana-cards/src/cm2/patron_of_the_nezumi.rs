//! Patron of the Nezumi — `{5}{B}{B}` 6/6 Legendary Spirit.
//! Rat offering (alternative cast cost — Offering keyword, not modeled).
//! Whenever a permanent is put into an opponent's graveyard, that player loses
//! 1 life.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Patron of the Nezumi");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: keyword — "Rat offering" (Offering) is not an available KeywordAbility.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::permanent()
                    .controlled_by(ControllerConstraint::Opponent),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: opponent_loses_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn opponent_loses_life(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(them) = trig
        .dying_object()
        .and_then(|id| state.objects.get(id))
        .map(|o| o.controller)
    else {
        return Vec::new();
    };
    vec![Effect::LoseLife {
        player: them,
        amount: 1,
    }]
}

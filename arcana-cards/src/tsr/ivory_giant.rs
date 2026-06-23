//! Ivory Giant — `{5}{W}{W}` 3/4 white Giant.
//!
//! Oracle:
//! * When this creature enters, tap all nonwhite creatures.
//! * Suspend 5—{W}. (GAP: Suspend is not in the usable KeywordAbility
//!   surface — no `KeywordAbility::Suspend` variant; the suspend cast
//!   mechanic is unmodeled here.)
//!
//! Implemented:
//! * The ETB "tap all nonwhite creatures" via `ForEach` over every nonwhite
//!   creature on the battlefield.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ivory Giant");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: tap_all_nonwhite,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn tap_all_nonwhite(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().without_colors(ColorSet::white());
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Tap { target: NULL_OBJECT_ID }),
    }]
}

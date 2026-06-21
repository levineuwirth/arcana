//! Child of Alara — `{W}{U}{B}{R}{G}` 6/6 Legendary Creature — Avatar.
//!
//! Oracle:
//! * Trample
//! * When Child of Alara dies, destroy all nonland permanents. They can't be
//!   regenerated.
//!
//! The dies trigger sweeps all nonland permanents via `Effect::ForEach` over
//! `script::ids_matching` (permanents that are not lands). The "can't be
//! regenerated" rider is GAP'd — there is no demonstrated field on
//! `Effect::DestroyPermanent` for it.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Child of Alara");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(
            ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost"),
        ),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: destroy_all_nonland,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn destroy_all_nonland(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // All nonland permanents. (Regeneration-prevention rider GAP'd.)
    let filter = ObjectFilter::permanent().without_types(TypeLine::LAND.into());
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: NULL_OBJECT_ID,
        }),
    }]
}

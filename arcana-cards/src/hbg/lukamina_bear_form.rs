//! Lukamina, Bear Form — `{2}{G}{G}` 4/4 Legendary Creature — Bear Druid with Trample.
//! "Other creatures you control get +1/+1 and have trample.
//!  When Lukamina, Bear Form dies, it unspecializes. If it unspecializes this way,
//!  return it to the battlefield tapped."
//!
//! The board-wide anthem static is GAP'd. The dies-unspecialize trigger's effect is
//! GAP'd (there is no unspecialize Effect in the demonstrated API).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lukamina, Bear Form");
    let bear = reg.interner_mut().intern("Bear");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static "Other creatures you control get +1/+1 and have trample." — a
    // board-wide anthem + keyword-granting static is not expressible.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dies(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "it unspecializes. If it unspecializes this way, return it to the
    // battlefield tapped." — there is no unspecialize Effect; the conditional
    // return-tapped rider hinges on the unspecialize outcome, so the whole effect is
    // GAP'd.
    Vec::new()
}

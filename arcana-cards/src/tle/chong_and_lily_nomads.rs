//! Chong and Lily, Nomads — `{3}{R}` 3/3 Legendary Human Bard Ally.
//! "Whenever one or more Bards you control attack, choose one —
//!   • Put a lore counter on each of any number of target Sagas you control.
//!   • Creatures you control get +1/+0 until end of turn for each lore
//!     counter among Sagas you control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chong and Lily, Nomads");
    let human = reg.interner_mut().intern("Human");
    let bard = reg.interner_mut().intern("Bard");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(bard);
    subtypes.0.insert(ally);

    let bard_filter = script::subtype_filter(reg, "Bard").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: modal "choose one" on a triggered ability is not expressible
    //      (no modal field on TriggeredAbilityDef). Neither mode is cleanly
    //      expressible: mode 0 ("put a lore counter on each of any number of
    //      target Sagas you control") has no add-counter-to-any-number-of-
    //      targets effect; mode 1 ("creatures you control get +1/+0 for each
    //      lore counter among Sagas you control") has no lore-counter-across-
    //      Sagas counting helper.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: bard_filter,
                },
                intervening_if: None,
                effect: choose_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn choose_one(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: see register() — modal choice + neither mode's effect is expressible.
    Vec::new()
}

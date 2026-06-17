//! It That Betrays — `{12}` 11/11 Eldrazi.
//! Annihilator 2; "Whenever an opponent sacrifices a nontoken permanent,
//! put that card onto the battlefield under your control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("It That Betrays");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    // GAP: keyword — Annihilator 2 is not in the usable KeywordAbility surface
    // (no Annihilator variant); the "defending player sacrifices two
    // permanents" attack trigger is therefore omitted.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{12}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(11)),
        toughness: Some(PtValue::Fixed(11)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::Sacrificed {
                filter: ObjectFilter::permanent()
                    .controlled_by(ControllerConstraint::Opponent)
                    .nontoken(),
            },
            intervening_if: None,
            effect: steal_sacrificed,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn steal_sacrificed(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put THAT card onto the battlefield under your control" — no
    // accessor exposes the just-sacrificed object's id and no effect
    // reanimates a specific known card to the battlefield under another
    // controller from the sacrifice event.
    Vec::new()
}

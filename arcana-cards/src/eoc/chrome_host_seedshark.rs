//! Chrome Host Seedshark — `{2}{U}` 2/4 blue Phyrexian Shark with Flying.
//!
//! * Flying (keyword line). (Incubate / Transform are ability mechanics,
//!   not KeywordAbility-surface keywords.)
//! * Whenever you cast a noncreature spell, incubate X (X = that spell's
//!   mana value). The trigger is wired, but the effect is GAP'd: there is
//!   no PendingTrigger accessor for the cast spell's mana value, and
//!   `Effect::Incubate` requires a literal `n` (dynamic X unreadable here).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Chrome Host Seedshark");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let shark = reg.interner_mut().intern("Shark");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(shark);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: incubate_x,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn incubate_x(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: incubate X where X = the cast spell's mana value — no accessor
    // for the triggering spell's mana value, and Effect::Incubate needs a
    // literal n.
    Vec::new()
}

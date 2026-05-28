//! Mold Demon — `{5}{B}{B}` 6/6 black Fungus Demon. "When this creature
//! enters, sacrifice it unless you sacrifice two Swamps."
//!
//! GAP: OptionalPaymentKind has no Sacrifice variant; "sacrifice two Swamps"
//! cost is not expressible. Emitting sacrifice-self as else_effect on a
//! best-effort mana gate instead — both polarity and cost type are incorrect.
//! Full effect: Vec::new() with gap note.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mold Demon");
    let fungus = reg.interner_mut().intern("Fungus");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    subtypes.0.insert(demon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_unless_sac_swamps,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_unless_sac_swamps(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: OptionalPaymentKind has no Sacrifice variant; "sacrifice two Swamps"
    // cost is not expressible. Emitting sacrifice-self as the fallback.
    vec![Effect::Sacrifice { player: trig.controller, filter: arcana_core::targets::ObjectFilter::new(), count: 1 }]
}

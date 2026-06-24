//! Mold Demon — `{5}{B}{B}` 6/6 black Fungus Demon. "When this creature
//! enters, sacrifice it unless you sacrifice two Swamps."
//!
//! GAP: the payment is "sacrifice two SWAMPS" — both a COUNT > 1 and a
//! Land-SUBTYPE constraint. OptionalPaymentKind::Sacrifice sacrifices exactly
//! ONE permanent and SacrificeFilter has no Land-subtype variant, so neither
//! the count nor the subtype is expressible. Emitting an unconditional
//! sacrifice-self would over-apply the penalty (fabrication), so the whole
//! ETB clause is GAP'd.

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
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice it unless you sacrifice two Swamps." The payment is a
    // COUNT > 1 + Land-SUBTYPE sacrifice (two Swamps); OptionalPaymentKind::
    // Sacrifice handles exactly one permanent with no Land-subtype filter, so
    // it isn't expressible. An unconditional sacrifice-self would over-apply
    // the penalty (fabrication), so the whole clause is GAP'd.
    Vec::new()
}

//! Thurid, Mare of Destiny — `{2}{W}{W}` 2/4 Legendary Pegasus with Flying
//! and Lifelink.
//! "Whenever you cast a Pegasus, Unicorn, or Horse creature spell, copy it.
//!  Other Pegasi, Unicorns, and Horses you control get +1/+1."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thurid, Mare of Destiny");
    let pegasus = reg.interner_mut().intern("Pegasus");
    let unicorn = reg.interner_mut().intern("Unicorn");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pegasus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    let spell_filter = ObjectFilter::new()
        .with_types(TypeLine::CREATURE.into())
        .with_subtypes_any(vec![pegasus, unicorn, horse]);

    // GAP: static "Other Pegasi, Unicorns, and Horses you control get +1/+1" —
    // a continuous tribal anthem is not expressible in this card class.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(spell_filter),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: copy_spell,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn copy_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "copy it" (the triggering spell) — no PendingTrigger accessor
    // exposes the just-cast spell's stack-object id, and the trigger is
    // non-targeted, so Effect::CopySpell cannot be supplied a target.
    Vec::new()
}

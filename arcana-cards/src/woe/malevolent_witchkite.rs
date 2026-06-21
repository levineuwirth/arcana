//! Malevolent Witchkite — `{4}{B}{B}` 5/4 Dragon Warlock.
//!
//! Flying.
//! When this creature enters, sacrifice any number of artifacts,
//! enchantments, and/or tokens, then draw that many cards.
//!
//! Flying is expressible. The ETB sacrifices any number of artifacts or
//! enchantments you control via `ChooseAnyNumberFromZone` /
//! `PickAction::Sacrifice`. Two fidelity GAPs: the "and/or tokens" arm of
//! the disjunction isn't included (a single `ObjectFilter` can't OR
//! "artifact or enchantment or any token"), and "then draw that many
//! cards" is unexpressible because the dynamic count (= number actually
//! sacrificed) isn't visible as a follow-up to the variable-count pick.

use arcana_core::effects::{Effect, KeywordAbility, PickAction};
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
    let name = reg.interner_mut().intern("Malevolent Witchkite");
    let dragon = reg.interner_mut().intern("Dragon");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_sacrifice_any_number,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_sacrifice_any_number(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "and/or tokens" arm omitted (can't OR artifact/enchantment/
    // any-token in one filter); "then draw that many cards" omitted (the
    // dynamic count isn't visible as a follow-up to the variable pick).
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: trig.controller,
        zone: Zone::Battlefield,
        filter: ObjectFilter::permanent()
            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT))
            .controlled_by(ControllerConstraint::You),
        action: PickAction::Sacrifice,
    }]
}

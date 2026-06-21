//! Wingnut, Bat on the Belfry — `{1}{R}` 1/2 Legendary Bat Mutant with
//! Menace.
//! "Alliance — Whenever another creature you control enters, Wingnut
//! gains your choice of flying, menace, or haste until end of turn."
//! "Whenever Wingnut attacks, each other attacking creature gets +1/+0
//! until end of turn."
//!
//! Menace is a base keyword (Alliance is a trigger-naming label, not a
//! standalone keyword). The Alliance trigger fires on another creature
//! you control entering; its "your choice of flying/menace/haste"
//! payload has no choose-among-keywords primitive, so the body is GAP'd.
//! The attack pump targets "each OTHER attacking creature" — the
//! demonstrated ObjectFilter surface can't restrict to attackers, so
//! that trigger's body is GAP'd too.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wingnut, Bat on the Belfry");
    let bat = reg.interner_mut().intern("Bat");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: alliance_gain_keyword,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_other_attackers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn alliance_gain_keyword(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Wingnut gains your choice of flying, menace, or haste until end of
    // turn" — there is no choose-among-keywords primitive; GrantKeyword takes a
    // single fixed keyword, not a modal choice.
    Vec::new()
}

fn pump_other_attackers(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each other attacking creature gets +1/+0 until end of turn" — the
    // demonstrated ObjectFilter surface cannot restrict to attacking creatures,
    // so the affected set is inexpressible.
    Vec::new()
}

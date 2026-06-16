//! Desperate Sentry — `{2}{W}` 1/2 white Human Soldier.
//!
//! Oracle text:
//! * "When this creature dies, create a 3/2 colorless Eldrazi Horror
//!   creature token." — a `SelfDies` triggered ability.
//! * "Delirium — This creature gets +3/+0 as long as there are four or
//!   more card types among cards in your graveyard." — a STATIC
//!   continuous self-pump gated by delirium. This card class has no
//!   primitive for a self-applying conditional continuous P/T buff, so
//!   the static is GAP'd (see below).
//!
//! The Scryfall "Delirium" keyword is ability-word / reminder text, not
//! one of the usable `KeywordAbility` variants, so `keywords` is empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::effects::TokenDefinition;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Desperate Sentry");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    // Pre-intern the token's subtype strings at registration time so the
    // read-only `reg.interner().lookup(..)` in the resolver succeeds.
    let _eldrazi = reg.interner_mut().intern("Eldrazi");
    let _horror = reg.interner_mut().intern("Horror");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static continuous self-pump "Delirium — This creature gets
    // +3/+0 as long as there are four or more card types among cards in
    // your graveyard." There is no triggered/activated/keyword primitive
    // in this card class to express a conditional continuous buff that
    // applies to the source itself, so the static is not emitted.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_make_horror,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "When this creature dies, create a 3/2 colorless Eldrazi Horror
/// creature token."
fn dies_make_horror(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let eldrazi = reg.interner().lookup("Eldrazi").unwrap_or_default();
    let horror = reg.interner().lookup("Horror").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(horror);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: eldrazi,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

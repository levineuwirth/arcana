//! Eldrazi Skyspawner — `{2}{U}` 2/1 colorless (Devoid) Eldrazi Drone
//! with Flying.
//!
//! Oracle:
//! * Devoid (this card has no color).
//! * Flying
//! * When this creature enters, create a 1/1 colorless Eldrazi Scion
//!   creature token. It has "Sacrifice this token: Add {C}."
//!
//! Devoid is rendered as `ColorSet::colorless()`. The Scion token's
//! "Sacrifice this token: Add {C}." ability cannot be baked onto a
//! `TokenDefinition`, so that activated ability is a GAP; the 1/1
//! colorless Eldrazi Scion body is wired.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Eldrazi Skyspawner");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let _scion = reg.interner_mut().intern("Scion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_scion,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_scion(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let scion = match reg.interner().lookup("Scion") {
        Some(sym) => sym,
        None => return Vec::new(),
    };
    let eldrazi = reg.interner().lookup("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    if let Some(e) = eldrazi {
        subtypes.0.insert(e);
    }
    subtypes.0.insert(scion);
    // GAP: the Scion's "Sacrifice this token: Add {C}." activated ability
    // cannot be baked onto a TokenDefinition; the body is wired.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: scion,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

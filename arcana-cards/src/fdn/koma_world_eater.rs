//! Koma, World-Eater — `{3}{G}{G}{U}{U}` 8/12 Legendary green/blue Serpent.
//!
//! * "This spell can't be countered." — a static cast-property; not
//!   expressible with the demonstrated API, GAP'd.
//! * Trample, Ward {4}.
//! * "Whenever Koma deals combat damage to a player, create four 3/3 blue
//!   Serpent creature tokens named Koma's Coil."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Koma, World-Eater");
    let serpent = reg.interner_mut().intern("Serpent");
    // Pre-intern the token name (its primary subtype handle).
    let _coil = reg.interner_mut().intern("Koma's Coil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}{U}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(12)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{4}").expect("valid cost")),
        ],
        ..Default::default()
    };
    // GAP: "This spell can't be countered." — static cast-property,
    // not expressible with the demonstrated API.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: make_coils,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_coils(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let coil = reg.interner().lookup("Koma's Coil").unwrap_or_default();
    let serpent = reg.interner().lookup("Serpent").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);
    let token = TokenDefinition {
        name: coil,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}

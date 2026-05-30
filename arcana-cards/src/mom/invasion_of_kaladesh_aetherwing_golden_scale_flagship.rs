//! Invasion of Kaladesh // Aetherwing, Golden-Scale Flagship
//!
//! Front face: `{U}{R}` Battle — Siege (blue/red) with 4 defense counters.
//! ETB: create a 1/1 colorless Thopter artifact creature token with flying.
//!
//! Back face: Legendary Artifact — Vehicle (blue/red). Flying.
//! "Aetherwing's power is equal to the number of artifacts you control."
//! "Crew 1"
//!
//! # GAPs
//! - Back face power "equal to the number of artifacts you control" — this is a
//!   dynamic characteristic-setting layer effect, not a static P/T. Not expressible;
//!   emitting 0/0 as a placeholder.
//! - Crew keyword is not in the engine keyword surface; not emitted.
//! - GAP: defeat→cast-back-face not auto-wired (CR 310.11 deferred).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Kaladesh");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Pre-intern Thopter for token creation
    let _thopter_sub = reg.interner_mut().intern("Thopter");

    // Back face: Aetherwing, Golden-Scale Flagship
    let back_name = reg.interner_mut().intern("Aetherwing, Golden-Scale Flagship");
    let vehicle_sub = reg.interner_mut().intern("Vehicle");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle_sub);
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes: back_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: power "equal to the number of artifacts you control" — dynamic; using 0 as placeholder
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: Crew 1 — not in keyword surface
        ..Default::default()
    };
    let back = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_thopter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn etb_create_thopter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thopter_sub = reg.interner().lookup("Thopter").expect("Thopter interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(thopter_sub);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: thopter_sub,
            colors: ColorSet::new(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}

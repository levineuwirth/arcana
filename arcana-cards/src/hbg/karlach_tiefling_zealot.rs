//! Karlach, Tiefling Zealot — `{1}{R}{W}` 4/4 Legendary Creature — Tiefling Barbarian.
//! "First strike, haste
//!  When this card specializes from your graveyard, return it from your graveyard
//!  to the battlefield. It perpetually gains 'This creature can't block.'
//!  When this card specializes from any zone, create a 2/2 white Knight creature
//!  token. Creatures you control get +1/+1 and gain haste until end of turn."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karlach, Tiefling Zealot");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(barbarian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: SelfSpecializes doesn't distinguish source zone; "from your
                // graveyard" specifically isn't expressible. Firing on any
                // specialization; effect returns it from graveyard.
                trigger_condition: TriggerCondition::SelfSpecializes,
                intervening_if: None,
                effect: specialize_from_graveyard,
                trigger_zones: vec![Zone::Graveyard(0), Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfSpecializes,
                intervening_if: None,
                effect: specialize_any_zone,
                trigger_zones: vec![Zone::Battlefield, Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn specialize_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "It perpetually gains 'This creature can't block.'" — perpetual
    // grants aren't expressible; only the graveyard return is emitted.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }]
}

fn specialize_any_zone(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let knight = reg.interner().lookup("Knight").unwrap_or_default();
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(knight);

    let mut effects = vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: knight,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }];

    // Creatures you control get +1/+1 and gain haste until end of turn.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    for id in ids {
        effects.push(Effect::Pump {
            target: id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Haste],
        });
    }
    effects
}

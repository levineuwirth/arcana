//! Vincent Valentine // Galian Beast
//!
//! Front: {2}{B}{B} Legendary Creature — Assassin 2/2
//! Whenever a creature an opponent controls dies, put a number of +1/+1 counters
//! on Vincent Valentine equal to that creature's power.
//! GAP: "equal to that creature's power" — power of the dying creature is not
//! accessible via the script API (script::power_of only works on battlefield objects).
//! Whenever Vincent Valentine attacks, you may transform it.
//!
//! Back: Legendary Creature — Werewolf Beast
//! Trample, lifelink.
//! GAP: "When Galian Beast dies, return it to the battlefield tapped (front face up)"
//! — back-face-only triggered ability not modeled (triggered abilities live on
//! the CardDefinition, not the face).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
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
    let name = reg.interner_mut().intern("Vincent Valentine");
    let assassin_sub = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(assassin_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Galian Beast");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let beast_sub = reg.interner_mut().intern("Beast");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub);
    back_subtypes.0.insert(beast_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Trample, KeywordAbility::Lifelink],
            // GAP: "When Galian Beast dies, return it to the battlefield tapped
            // (front face up)" — back-face-only triggered ability not modeled.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: whenever a creature an opponent controls dies, put +1/+1
            // counters equal to that creature's power on Vincent Valentine.
            // GAP: power of dying creature not accessible — emitting Vec::new().
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: opponent_creature_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2: whenever Vincent Valentine attacks, you may transform it.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn opponent_creature_dies(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put a number of +1/+1 counters equal to that creature's power" —
    // power of the dying creature is not accessible via the script API
    // (script::power_of only works on battlefield objects, not dead ones).
    Vec::new()
}

fn attacks_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

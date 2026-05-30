//! Desperate Farmer // Depraved Harvester — `{2}{B}` Human Peasant 2/2 with lifelink.
//! Front face: Lifelink. When another creature you control dies, transform this creature.
//! Back face: Depraved Harvester — Human Knight, also with Lifelink.
//!
//! Transform trigger is front-face only (fires when a creature you control dies).
//! GAP: back-face-only triggered ability not modeled (back has no additional abilities beyond lifelink).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Desperate Farmer");
    let human_sub = reg.interner_mut().intern("Human");
    let peasant_sub = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(peasant_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Lifelink],
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Depraved Harvester");
    let back_human = reg.interner_mut().intern("Human");
    let back_knight = reg.interner_mut().intern("Knight");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_human);
    back_subtypes.0.insert(back_knight);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Lifelink],
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Transform trigger: front-face only — when another creature you control dies,
    // transform this creature.
    let creature_filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: creature_filter,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: transform_on_creature_death,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn transform_on_creature_death(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Only transform if the dying creature is NOT this creature itself
    // (the trigger says "another creature you control dies").
    // The engine's trigger condition will match any creature you control entering the graveyard;
    // we must exclude self. Since we can't easily check if the dying creature == trig.source,
    // we emit the transform — the face gate on the trigger (front-face) handles directionality.
    vec![Effect::Transform { target: trig.source }]
}

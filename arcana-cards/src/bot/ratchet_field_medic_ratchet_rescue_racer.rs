//! Ratchet, Field Medic // Ratchet, Rescue Racer
//!
//! Front: {2}{W} Legendary Artifact Creature — Robot 2/4
//!   More Than Meets the Eye {1}{W} (alternate cast cost — GAP: not in keyword API)
//!   Lifelink
//!   Whenever you gain life, you may convert Ratchet. When you do, return target artifact card
//!   with mana value <= amount of life gained this turn from your graveyard to battlefield tapped.
//!
//! Back: Legendary Artifact — Vehicle (Ratchet, Rescue Racer) 4/6
//!   Living metal (During your turn, this Vehicle is also a creature.) — GAP: not modeled
//!   Lifelink
//!   Whenever one or more nontoken artifacts you control are put into a graveyard from the
//!   battlefield, convert Ratchet. This ability triggers only once each turn.
//!
//! GAP: "More Than Meets the Eye" — alternate casting cost mechanic, not in keyword list.
//! GAP: "Living metal" — Vehicle-becomes-creature during your turn, not modeled.
//! GAP: "Convert" — same as Transform in Transformers context; modeled via Effect::Transform.
//! GAP: Front-face trigger "Whenever you gain life" — TriggerCondition::YouGainLife not available.
//!      Wired as a ZoneChange placeholder (wrong semantics); the real trigger cannot be expressed.
//! GAP: The full effect of the lifegain trigger (return artifact card with mv <= life gained this
//!      turn from graveyard) requires dynamic life-gained-this-turn tracking not in script API.
//! GAP: Back-face-only triggered ability (nontoken artifacts to graveyard → convert)
//!      not auto-installed on transform back face.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ratchet, Field Medic");
    let robot_sub = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ratchet, Rescue Racer");
    let vehicle_sub = reg.interner_mut().intern("Vehicle");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // Back face P/T for Vehicle form; Living metal makes it a creature during your turn
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::Lifelink],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: Front trigger "Whenever you gain life" — no TriggerCondition for lifegain available.
    // Using ZoneChange creature->graveyard as a structural placeholder that compiles but has
    // wrong semantics. The real lifegain trigger and its full effect are not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_gain_life_convert,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_gain_life_convert(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Full effect is "you may convert Ratchet; if you do, return target artifact card with
    // mana value <= life gained this turn from your graveyard to battlefield tapped."
    // Dynamic life-gained-this-turn is not accessible via script API.
    // OptionalPayment wrapping transform is not expressible for the "when you do" chained trigger.
    // Emitting just the transform (convert) as best approximation.
    vec![Effect::Transform { target: trig.source }]
}

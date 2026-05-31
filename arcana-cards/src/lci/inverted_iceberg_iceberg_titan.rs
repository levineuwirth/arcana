//! Inverted Iceberg // Iceberg Titan — `{1}{U}` Artifact (front) that
//! transforms (via Craft) into the back-face Artifact Creature — Golem
//! "Iceberg Titan".
//!
//! Front (Inverted Iceberg): When this artifact enters, mill a card,
//! then draw a card.
//! Craft with artifact {4}{U}{U} — exiles artifacts to return this card
//! transformed. (GAP: the Craft activation's cost shape — exile this +
//! another artifact you control or an artifact card from your graveyard
//! — is not expressible with the current OptionalPaymentKind / activated
//! cost surface, so the craft-to-transform path is unmodeled engine debt.)
//!
//! Back (Iceberg Titan): Whenever this creature attacks, you may tap or
//! untap target artifact or creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inverted Iceberg");

    // FRONT face: Artifact, {1}{U}, blue. No P/T (noncreature artifact).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    // BACK face: Iceberg Titan — Artifact Creature — Golem.
    let back_name = reg.interner_mut().intern("Iceberg Titan");
    let golem_sub = reg.interner_mut().intern("Golem");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(golem_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // FRONT trigger (face 0): ETB mill 1, then draw 1.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: etb_mill_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // BACK trigger (face 1): on attack, may tap or untap target
            // artifact or creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_tap_or_untap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn etb_mill_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::Mill { player: trig.controller, count: 1 },
        Effect::DrawCards { player: trig.controller, count: 1 },
    ]
}

fn on_attack_tap_or_untap(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // "tap or untap" — the may/choice between the two modes is a
    // resolution-time choice; engine resolves a single Tap here.
    // GAP: the tap-OR-untap player choice is not expressible as one
    // Effect; emitting Untap as the typical attacker-friendly mode.
    vec![Effect::Untap { target: *id }]
}

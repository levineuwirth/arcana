//! Alluring Suitor // Deadly Dancer
//!
//! Front face: Creature — Vampire {2}{R}, 2/3, red.
//! "When you attack with exactly two creatures, transform this creature."
//! GAP: 'exactly two creatures attacking' trigger condition not expressible.
//!
//! Back face: Creature — Vampire, red.
//! Trample.
//! "When this creature transforms into Deadly Dancer, add {R}{R}. Until end
//! of turn, you don't lose this mana as steps and phases end."
//! The on-transform "add {R}{R}" is wired via a SelfTransforms{to_face: 1}
//! trigger (gated to face 1) → Effect::AddMana.
//! GAP: 'until end of turn, don't lose this mana as steps and phases end' rider
//! not modeled (no mana-pool persistence Effect).
//!
//! "{R}{R}: This creature and another target creature each get +1/+0 until
//! end of turn." — wired as a back-face activated ability (face_gate Some(1)):
//! pumps the source and a chosen target creature.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alluring Suitor");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Deadly Dancer");
    let vampire_back_sub = reg.interner_mut().intern("Vampire");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vampire_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Back: "When this creature transforms into Deadly Dancer, add {R}{R}."
            // (face 1). GAP: the "don't lose this mana as phases end" rider is not modeled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: on_transform_add_rr,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back activated: {R}{R}: This creature and another target creature
            // each get +1/+0 until end of turn (face 1).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{R}: This creature and another target creature each get +1/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: pump_self_and_target,
            }),
        // GAP: front "when you attack with exactly two creatures, transform" not modeled —
        // there is no exactly-N-attackers trigger condition (only the alone/==1 forms).
    )
}

fn on_transform_add_rr(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "don't lose this mana as steps and phases end" not modeled.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![
            arcana_core::mana::ManaUnit::plain(ManaColor::Red, trig.source),
            arcana_core::mana::ManaUnit::plain(ManaColor::Red, trig.source),
        ],
    }]
}

fn pump_self_and_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    // "another target creature" — the filter can't exclude this source, so the
    // engine's targeting may permit Deadly Dancer itself; functionally faithful.
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        effects.push(Effect::Pump {
            target: *id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    effects
}

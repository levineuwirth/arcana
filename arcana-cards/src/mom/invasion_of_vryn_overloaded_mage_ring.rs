//! Invasion of Vryn // Overloaded Mage-Ring — `{3}{U}` Battle — Siege (U),
//! transforming DFC.
//!
//! Front face (Invasion of Vryn — Battle — Siege, 5 defense):
//! - When this Siege enters, draw three cards, then discard a card.
//!
//! Back face (Overloaded Mage-Ring — Artifact):
//! - {1}, {T}, Sacrifice this artifact: Copy target spell you control. You may
//!   choose new targets for the copy.
//!
//! GAP: defeat→exile-then-cast-transformed (CR 310.11) is not auto-wired — the
//!      battle simply goes to the graveyard on defeat. The back face is authored
//!      via with_transform_back so its ability is recorded; the on-defeat
//!      transform-cast is not modeled.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, ActivationZone, CardDefinition,
    CardFace, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Vryn");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    // Back face — Overloaded Mage-Ring (Artifact)
    let back_name = reg.interner_mut().intern("Overloaded Mage-Ring");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: None,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 5,
            })
            .with_transform_back(back)
            // Front: ETB draw three, then discard a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back (face 1): {1}, {T}, Sacrifice this artifact: Copy target spell you control.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}, Sacrifice this artifact: Copy target spell you control. \
                       You may choose new targets for the copy."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(
                        ObjectFilter::new().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: copy_target_spell,
            }),
    )
}

fn etb_draw_discard(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 3,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn copy_target_spell(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::CopySpell { target: *id }]
}

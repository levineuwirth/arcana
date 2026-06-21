//! Gilt-Leaf Archdruid — `{3}{G}{G}` 3/3 Elf Druid.
//!
//! Oracle:
//! * Whenever you cast a Druid spell, you may draw a card.
//!   (The "may" is a resolution-time choice; modeled as draw a card.)
//! * Tap seven untapped Druids you control: Gain control of all lands
//!   target player controls.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gilt-Leaf Archdruid");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let druid_spell_filter = script::subtype_filter(reg, "Druid");

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(druid_spell_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: may_draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap seven untapped Druids you control: Gain control of all lands \
                       target player controls."
                    .into(),
                cost: ActivationCost {
                    tap_other: Some(script::subtype_filter(reg, "Druid")),
                    tap_other_count: 7,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_control_of_lands,
            }),
    )
}

fn may_draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}

fn gain_control_of_lands(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    let land_filter = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    // Lands controlled by the chosen player (use them as the perspective).
    // ChangeControl is not retargeted by Effect::ForEach, so build one
    // ChangeControl per id and wrap in a Sequence.
    let ids = script::ids_matching(state, &land_filter, *p);
    vec![Effect::Sequence(
        ids.into_iter()
            .map(|id| Effect::ChangeControl {
                target: id,
                new_controller: ctx.controller,
            })
            .collect(),
    )]
}

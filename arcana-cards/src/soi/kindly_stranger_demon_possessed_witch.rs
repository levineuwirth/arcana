//! Kindly Stranger // Demon-Possessed Witch — `{2}{B}` Creature — Human 2/3.
//! Delirium — {2}{B}: Transform this creature. Activate only if there are four or
//! more card types among cards in your graveyard. (Delirium condition is a GAP.)
//! Back: Creature — Human Shaman 2/3.
//! When this creature transforms into Demon-Possessed Witch, you may destroy target creature.
//!
//! # GAPs
//! - Delirium activation condition (4+ card types in graveyard) is not expressible
//!   via `ActivationCost` — there is no "graveyard card-type count" gate field.
//!   The activated ability is authored without that condition check.
//! - Delirium keyword not in KeywordAbility enum — omitted.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kindly Stranger");
    let human_sub = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Back face: Demon-Possessed Witch — Human Shaman 2/3
    let back_name = reg.interner_mut().intern("Demon-Possessed Witch");
    let shaman_sub = reg.interner_mut().intern("Shaman");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub);
    back_subtypes.0.insert(shaman_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Delirium — {2}{B}: Transform.
            // GAP: Delirium gate (4+ card types in graveyard) not expressible via ActivationCost.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Delirium — {2}{B}: Transform this creature. Activate only if there are four or more card types among cards in your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_self,
            })
            // Back: When this creature transforms into Demon-Possessed Witch,
            // you may destroy target creature ("you may" via up-to-one target).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: on_transform_destroy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn on_transform_destroy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

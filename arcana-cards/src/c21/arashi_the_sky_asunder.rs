//! Arashi, the Sky Asunder — `{3}{G}{G}` 5/5 Legendary Spirit.
//! "{X}{G}, {T}: Arashi deals X damage to target creature with flying."
//! "Channel — {X}{G}{G}, Discard this card: It deals X damage to each
//! creature with flying."
//!
//! Channel is not an expressible keyword, so the keyword line is empty;
//! its synthesized "{X}{G}{G}, Discard this card: ..." graveyard/hand
//! activation is wired as an ActivatedAbility from the hand zone with
//! discard_self. Both abilities deal X damage (X = the paid {X},
//! read from ctx.x_value) to flying creatures.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arashi, the Sky Asunder");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{G}, {T}: Arashi deals X damage to target creature with flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: damage_flyer,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Channel — {X}{G}{G}, Discard this card: It deals X damage to each creature with flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{G}{G}").expect("valid cost"),
                    discard_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: channel_damage_all_flyers,
            }),
    )
}

fn damage_flyer(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let x = ctx.x_value.unwrap_or(0);
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: x,
    }]
}

fn channel_damage_all_flyers(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: x,
        }),
    }]
}

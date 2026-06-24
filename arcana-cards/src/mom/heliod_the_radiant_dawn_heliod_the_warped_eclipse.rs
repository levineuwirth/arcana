//! Heliod, the Radiant Dawn // Heliod, the Warped Eclipse — {2}{W}{W}
//!
//! Front: Legendary Enchantment Creature — God 4/4
//! When Heliod enters, return target enchantment card that isn't a God from your graveyard to your hand.
//! {3}{U/P}: Transform Heliod. Activate only as a sorcery.
//!
//! Back: Legendary Enchantment Creature — Phyrexian God
//! You may cast spells as though they had flash.
//! Spells you cast cost {1} less for each card opponents drew this turn.
//!
//! GAP: {U/P} hybrid/Phyrexian mana in activated ability cost — using {U} as approximation.
//! GAP: back-face static "you may cast spells as though they had flash" — there is no
//!      continuous-effect / cast-permission primitive granting instant-speed casting for a
//!      class of spells (cf. Prophet of Kruphix, Leyline of Anticipation); the engine's Flash
//!      support is per-spell keyword / flashback only. Missing primitive: a
//!      ContinuousEffectKind cast-as-though-flash permission. Not wired.
//! GAP: back-face static "spells you cast cost {1} less for each card opponents drew this
//!      turn" — ContinuousEffectKind::SpellCostModifier exists but its generic_delta is a
//!      FIXED i32, not a per-game-state count; the "for each card opponents drew this turn"
//!      scaling is not expressible (no dynamic cost-reduction primitive). Not wired (a fixed
//!      delta would fabricate the amount).
//! NOTE: both back-face abilities are STATICS with no expressible payload, so neither is
//!      wired. The front ETB return and the transform activated ability ARE wired below.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heliod, the Radiant Dawn");
    let god_sub = reg.interner_mut().intern("God");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes: front_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Heliod, the Warped Eclipse");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let back_god_sub = reg.interner_mut().intern("God");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(back_god_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // ETB trigger: return target non-God enchantment card from graveyard to hand
    let enchantment_filter = ObjectFilter::new()
        .with_types(TypeLine::ENCHANTMENT.into())
        .without_subtype_sym(god_sub);

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_return_enchantment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: enchantment_filter,
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // {3}{U}: Transform Heliod (approximation — {U/P} not expressible as mana cost)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: Transform Heliod. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: do_transform,
            })
        // GAP: back-face statics (cast-as-though-flash; per-card-drawn cost reduction)
        // have no expressible payload — see the module doc for the exact missing primitives.
    )
}

fn etb_return_enchantment(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}

fn do_transform(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

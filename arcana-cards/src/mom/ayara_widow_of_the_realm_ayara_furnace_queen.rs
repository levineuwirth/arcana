//! Ayara, Widow of the Realm // Ayara, Furnace Queen
//!
//! Front face: Legendary Creature — Elf Noble, 3/3, {1}{B}{B}
//!   {T}, Sacrifice another creature or artifact: Ayara deals X damage to target opponent
//!     or battle and you gain X life, where X is the sacrificed permanent's mana value.
//!     GAP: sacrifice-other as activation cost — modeled via sacrifice_other on ActivationCost.
//!     GAP: "X is the sacrificed permanent's mana value" — no accessor for sacrificed
//!     permanent's CMC in activation context; modeled as 0 damage (GAP the scaling).
//!     GAP: "target opponent or battle" — TargetFilter::AnyTarget is the closest;
//!     battles not separately modeled.
//!   {5}{R/P}: Transform Ayara. Activate only as a sorcery. ({R/P} can be paid with
//!     either {R} or 2 life.)
//!     GAP: Phyrexian mana {R/P} not representable in ManaCost::parse; modeled as {5}{R}.
//!
//! Back face: Legendary Creature — Phyrexian Elf Noble
//!   At the beginning of combat on your turn, return up to one target artifact or creature
//!   card from your graveyard to the battlefield. It gains haste. Exile it at the beginning
//!   of the next end step.
//!   GAP: back-face-only triggered ability not auto-installed on transform.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ayara, Widow of the Realm");

    let sub_elf = reg.interner_mut().intern("Elf");
    let sub_noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_elf);
    subtypes.0.insert(sub_noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Ayara, Furnace Queen — Legendary Creature — Phyrexian Elf Noble
    let back_name = reg.interner_mut().intern("Ayara, Furnace Queen");
    let back_sub_phyrexian = reg.interner_mut().intern("Phyrexian");
    let back_sub_elf = reg.interner_mut().intern("Elf");
    let back_sub_noble = reg.interner_mut().intern("Noble");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_sub_phyrexian);
    back_subtypes.0.insert(back_sub_elf);
    back_subtypes.0.insert(back_sub_noble);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {T}, Sacrifice another creature or artifact: deal X damage / gain X life
            // GAP: X = sacrificed permanent's mana value — no CMC accessor; emits 0 damage.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice another creature or artifact: Ayara deals X damage to target opponent or battle and you gain X life.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::ARTIFACT))
                            .controlled_by(ControllerConstraint::You),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0), // front face only
                effect: sacrifice_deal_damage,
            })
            // {5}{R}: Transform Ayara. Activate only as a sorcery.
            // GAP: {R/P} Phyrexian mana modeled as {R}.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{R}: Transform Ayara. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_to_queen,
            })
            // GAP: back-face-only triggered ability (beginning of combat on your turn,
            //   return target artifact or creature card from graveyard to battlefield,
            //   gains haste, exile at next end step) not auto-installed on transform.
    )
}

fn sacrifice_deal_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X = sacrificed permanent's mana value — no CMC accessor for sacrificed perm.
    // Emitting 0 damage and 0 life gain as a best-effort stub.
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // X = 0 (GAP: should be mana value of sacrificed permanent)
    vec![
        Effect::DealDamage {
            target: DamageTarget::Player(*p),
            amount: 0,
            source: ctx.source,
        },
        Effect::GainLife {
            player: ctx.controller,
            amount: 0,
        },
    ]
}

fn transform_to_queen(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

//! Oko, Thief of Crowns — `{1}{G}{U}` legendary planeswalker, starting
//! loyalty 4. Subtype Oko; green-blue.
//!
//! Loyalty abilities:
//! * `+2`: Create a Food token. (Functional via
//!   `Effect::CreateCommodityToken`.)
//! * `+1`: Target artifact or creature loses all abilities and becomes a
//!   green Elk creature with base power and toughness 3/3. (Functional for
//!   lose-all-abilities + base 3/3 + green; the Elk subtype overlay /
//!   strip-other-types is omitted — no subtype-set effect in the surface.)
//! * `−5`: Exchange control of an artifact/creature you control and a
//!   small creature an opponent controls. GAP — a two-way control exchange
//!   isn't expressible (`ChangeControl` is one-directional).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oko, Thief of Crowns");
    let oko = reg.interner_mut().intern("Oko");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(oko);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Create a Food token.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_food,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Target artifact or creature loses all abilities \
                       and becomes a green Elk creature with base power and \
                       toughness 3/3."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_elk,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−5: Exchange control of target artifact or creature \
                       you control and target creature an opponent controls \
                       with power 3 or less."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_gap,
            }),
    )
}

fn plus_two_food(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: arcana_core::effects::CommodityToken::Food,
        count: 1,
    }]
}

fn plus_one_elk(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // Functional: lose all abilities, base 3/3, becomes green. The Elk
    // subtype overlay and strip-other-types are omitted (no subtype-set
    // effect in the demonstrated surface).
    vec![
        Effect::LoseAllAbilities { target: *id, duration: Duration::WhileSourceOnBattlefield },
        Effect::SetBasePT {
            target: *id,
            power: 3,
            toughness: 3,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetColor {
            target: *id,
            colors: ColorSet::green(),
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}

fn minus_five_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: two-way control exchange is not expressible (ChangeControl is
    // one-directional).
    Vec::new()
}

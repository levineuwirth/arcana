//! Ugin, the Ineffable — `{6}` legendary planeswalker, starting loyalty 4.
//! Subtype Ugin; colorless. (The static "Colorless spells you cast cost
//! {2} less" is a continuous cost-reduction, not a loyalty ability — not
//! modeled here.)
//!
//! Loyalty abilities:
//! * `+1`: Exile the top card of your library face down and look at it;
//!   create a 2/2 colorless Spirit creature token; when that token leaves,
//!   put the exiled card into your hand. (Token creation is functional; the
//!   face-down exile linked to the token's leaves-trigger is omitted —
//!   bespoke exile-linkage.)
//! * `−3`: Destroy target permanent that's one or more colors. (Functional
//!   via `Effect::DestroyPermanent` over a colored-permanent target.)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ugin, the Ineffable");
    let ugin = reg.interner_mut().intern("Ugin");
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ugin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Exile the top card of your library face down and \
                       look at it. Create a 2/2 colorless Spirit creature \
                       token. When that token leaves the battlefield, put the \
                       exiled card into your hand."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_spirit,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy target permanent that's one or more \
                       colors."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent().with_colors(
                        ColorSet::white()
                            | ColorSet::blue()
                            | ColorSet::black()
                            | ColorSet::red()
                            | ColorSet::green(),
                    )),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_destroy,
            }),
    )
}

fn plus_one_spirit(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // The 2/2 Spirit token is functional; the face-down exile linked to the
    // token's leaves-the-battlefield trigger is omitted (bespoke linkage).
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_three_destroy(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

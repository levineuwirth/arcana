//! A-Teferi, Time Raveler — `{2}{W}{U}` Legendary Planeswalker — Teferi,
//! starting loyalty 4.
//!
//! Static (GAP): Your opponents can't cast spells during your turn. (Not a
//! loyalty ability; a casting-restriction static not in the demonstrated
//! surface — omitted.)
//! +1: Until your next turn, you may cast sorcery spells as though they had
//!     flash. (GAP — timing-permission rider.)
//! −3: Return up to one target artifact, creature, or enchantment to its
//!     owner's hand. Draw a card.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Teferi, Time Raveler");
    let teferi = reg.interner_mut().intern("Teferi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(teferi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, you may cast sorcery spells as though they had flash.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_flash,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Return up to one target artifact, creature, or enchantment to its owner's hand. Draw a card.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter {
                        types_any: Some(TypeLine(
                            TypeLine::ARTIFACT | TypeLine::CREATURE | TypeLine::ENCHANTMENT,
                        )),
                        ..Default::default()
                    }),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_bounce_draw,
            }),
    )
}

fn plus_one_flash(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may cast sorcery spells as though they had flash" — a casting
    //      timing-permission rider; not expressible from the demonstrated
    //      effect surface.
    Vec::new()
}

fn minus_three_bounce_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out = Vec::new();
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        out.push(Effect::ReturnToHand { target: *id });
    }
    out.push(Effect::DrawCards { player: ctx.controller, count: 1 });
    out
}

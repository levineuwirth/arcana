//! Koth of the Hammer — `{2}{R}{R}` Legendary Planeswalker — Koth, starting
//! loyalty 3.
//!
//! +1: Untap target Mountain. It becomes a 4/4 red Elemental creature until
//!   end of turn. It's still a land. The untap is modeled; the land-animation
//!   ("becomes a 4/4 red Elemental until end of turn") has no expressible
//!   primitive — partially GAP'd.
//! −2: Add {R} for each Mountain you control. A dynamic mana amount keyed on
//!   a board count; not expressible from the demonstrated mana surface. GAP
//!   (correct −2 cost shell).
//! −5: You get an emblem with "Mountains you control have '{T}: This land
//!   deals 1 damage to any target.'" The emblem grants an ACTIVATED ability
//!   to a filtered set — an ability-granting static the anthem/keyword/
//!   filtered builders can't express. GAP the emblem (correct −5 cost shell).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Koth of the Hammer");
    let koth = reg.interner_mut().intern("Koth");
    let mountain = reg.interner_mut().intern("Mountain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(koth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    let mountain_filter = ObjectFilter::new().with_subtype_sym(mountain);

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap target Mountain. It becomes a 4/4 red Elemental \
                       creature until end of turn. It's still a land.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(mountain_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap_mountain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Add {R} for each Mountain you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-5: You get an emblem with \"Mountains you control have \
                       '{T}: This land deals 1 damage to any target.'\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_emblem,
            }),
    )
}

fn plus_one_untap_mountain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a 4/4 red Elemental creature until end of turn (still a
    //      land)" — land-animation has no expressible primitive; the untap
    //      is modeled.
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Untap { target: *id }]
}

fn minus_two_mana(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Add {R} for each Mountain you control" — a dynamic mana amount
    //      keyed on a board count; not expressible from the demonstrated
    //      mana surface.
    Vec::new()
}

fn minus_five_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the emblem grants an ACTIVATED mana/damage ability to all
    //      Mountains you control — an ability-granting static the
    //      anthem/keyword/filtered emblem builders can't express.
    Vec::new()
}

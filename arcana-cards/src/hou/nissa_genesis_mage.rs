//! Nissa, Genesis Mage — `{5}{G}{G}` Legendary Planeswalker — Nissa.
//! Starting loyalty 5 (oracle).
//!
//! +2: Untap up to two target creatures and up to two target lands.
//! −3: Target creature gets +5/+5 until end of turn.
//! −10: Look at the top ten cards of your library. You may put any number of
//!     creature and/or land cards from among them onto the battlefield. Put
//!     the rest on the bottom of your library in a random order.
//!     GAP: "look at top N, put ANY NUMBER of matching onto the battlefield"
//!     (a multi-card optional put) is not expressible (DigTopN takes a single
//!     optional pick to hand only).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa, Genesis Mage");
    let nissa = reg.interner_mut().intern("Nissa");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nissa);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Untap up to two target creatures and up to two \
                       target lands.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(2),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent()
                                .with_types(TypeLine::LAND.into()),
                        ),
                        count: TargetCount::UpTo(2),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Target creature gets +5/+5 until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: Look at the top ten cards of your library. You may \
                       put any number of creature and/or land cards onto the \
                       battlefield. Put the rest on the bottom in a random \
                       order.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_gap,
            }),
    )
}

fn plus_two_untap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::Untap { target: *id }),
            _ => None,
        })
        .collect()
}

fn minus_three_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 5,
        toughness: 5,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn minus_ten_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: look at top 10, put any number of creature/land cards onto the
    //      battlefield — a multi-card optional put is not expressible.
    Vec::new()
}

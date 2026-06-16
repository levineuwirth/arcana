//! Gideon, Martial Paragon — `{4}{W}` Legendary Planeswalker — Gideon,
//! starting loyalty (printed) per spec.
//!
//! Loyalty abilities:
//! * `+2`: Untap all creatures you control. Those creatures get +1/+1
//!   until end of turn.
//! * `0`: Until end of turn, Gideon becomes a 5/5 Human Soldier creature
//!   with indestructible that's still a planeswalker. Prevent all damage
//!   that would be dealt to him this turn. (PW ANIMATION — modeled as
//!   AddType(CREATURE) + SetBasePT(5/5) + GrantKeyword(Indestructible) +
//!   PreventDamage on self. The "Human Soldier" subtype grant is not
//!   expressible — GAP'd, the body is otherwise faithful.)
//! * `−10`: Creatures you control get +2/+2 until end of turn. Tap all
//!   creatures your opponents control.
//!
//! # Rules references
//! * CR 606 — loyalty abilities; CR 704.5i — 0-loyalty sacrifice.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gideon, Martial Paragon");
    let gideon = reg.interner_mut().intern("Gideon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gideon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Untap all creatures you control. Those creatures \
                       get +1/+1 until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((arcana_core::types::CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_untap_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Until end of turn, Gideon, Martial Paragon becomes a \
                       5/5 Human Soldier creature with indestructible that's \
                       still a planeswalker. Prevent all damage that would be \
                       dealt to him this turn.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_animate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−10: Creatures you control get +2/+2 until end of turn. \
                       Tap all creatures your opponents control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((arcana_core::types::CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten,
            }),
    )
}

fn plus_two_untap_pump(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    let mut effects = Vec::new();
    for id in ids {
        effects.push(Effect::Untap { target: id });
        effects.push(Effect::Pump {
            target: id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    effects
}

fn zero_animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Human Soldier" creature-subtype grant is not expressible from
    // the demonstrated Effect surface; the rest of the animation is faithful.
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 5,
            toughness: 5,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
        Effect::PreventDamage {
            target: DamageTarget::Object(ctx.source),
            amount: None,
            duration: ReplacementDuration::EndOfTurn,
        },
    ]
}

fn minus_ten(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    let yours = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    for id in script::ids_matching(state, &yours, ctx.controller) {
        effects.push(Effect::Pump {
            target: id,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    let opp = ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
    for id in script::ids_matching(state, &opp, ctx.controller) {
        effects.push(Effect::Tap { target: id });
    }
    effects
}

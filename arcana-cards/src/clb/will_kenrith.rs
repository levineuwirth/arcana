//! Will Kenrith — `{4}{U}{U}` Legendary Planeswalker — Will,
//! starting loyalty 5 — colors U.
//!
//! Oracle text (Partner ignored — not modeled):
//! * `+2`: Until your next turn, up to two target creatures each have base
//!   power and toughness 0/3 and lose all abilities. — `SetBasePT` +
//!   `LoseAllAbilities` per chosen target. "Until your next turn" is
//!   approximated by `Duration::EndOfTurn`.
//! * `−2`: Target player draws two cards. Until your next turn, instant,
//!   sorcery, and planeswalker spells that player casts cost {2} less. —
//!   `DrawCards` is faithful; the cost-reduction rider is GAP'd.
//! * `−8`: Target player gets an emblem "Whenever you cast an instant or
//!   sorcery spell, copy it twice…". — emblem; GAP.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Will Kenrith");
    let will = reg.interner_mut().intern("Will");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(will);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Until your next turn, up to two target creatures \
                       each have base power and toughness 0/3 and lose all \
                       abilities.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_neutralize,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Target player draws two cards. Until your next turn, \
                       instant, sorcery, and planeswalker spells that player \
                       casts cost {2} less.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Target player gets an emblem with \"Whenever you cast \
                       an instant or sorcery spell, copy it twice. You may \
                       choose new targets for the copies.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

/// `+2`: up to two targets become base 0/3 and lose all abilities.
fn plus_two_neutralize(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Until your next turn" approximated by Duration::EndOfTurn.
    let mut out = Vec::new();
    for t in &ctx.targets.targets {
        if let TargetChoice::Object(id) = t {
            out.push(Effect::SetBasePT {
                target: *id,
                power: 0,
                toughness: 3,
                duration: Duration::EndOfTurn,
            });
            out.push(Effect::LoseAllAbilities {
                target: *id,
                duration: Duration::EndOfTurn,
            });
        }
    }
    out
}

/// `−2`: target player draws two (cost-reduction rider GAP'd).
fn minus_two_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "instant/sorcery/PW spells that player casts cost {2} less until
    // your next turn" is not expressible; the draw is faithful.
    let player = match ctx.targets.targets.first() {
        Some(TargetChoice::Player(p)) => *p,
        _ => return Vec::new(),
    };
    vec![Effect::DrawCards { player, count: 2 }]
}

/// `−8`: grant an emblem.
fn minus_eight_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblems (static text granted to a player) are not expressible from
    // the demonstrated Effect surface.
    Vec::new()
}

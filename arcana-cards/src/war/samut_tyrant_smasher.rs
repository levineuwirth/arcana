//! Samut, Tyrant Smasher — `{2}{R/G}{R/G}` legendary planeswalker, starting loyalty 5.
//!
//! Static: Creatures you control have haste (static ability, not a
//!   loyalty ability — GAP; no loyalty cost to attach it to).
//! −1: Target creature gets +2/+1 and gains haste until end of turn. Scry 1.
//!
//! Scope: the −1 ability is fully expressed (pump + haste grant + Scry 1).
//! The "creatures you control have haste" static is a continuous ability
//! with no loyalty cost — not a loyalty ability — and is noted as a GAP.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Samut, Tyrant Smasher");
    let samut = reg.interner_mut().intern("Samut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(samut);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R/G}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    // GAP: "Creatures you control have haste" static ability has no
    // loyalty cost — not a loyalty ability and not attachable here.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Target creature gets +2/+1 and gains haste until end \
                       of turn. Scry 1.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_pump,
            }),
    )
}

/// `−1: +2/+1 and haste until end of turn, then Scry 1.`
fn minus_one_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return vec![Effect::Scry { player: ctx.controller, count: 1 }];
    };
    vec![
        Effect::Pump {
            target: *id,
            power: 2,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Haste],
        },
        Effect::Scry { player: ctx.controller, count: 1 },
    ]
}

//! Tibalt, the Fiend-Blooded — `{R}{R}` Legendary Planeswalker — Tibalt,
//! starting loyalty 2.
//!
//! +1: Draw a card, then discard a card at random.
//! −4: Tibalt deals damage equal to the number of cards in target player's
//!     hand to that player.
//! −6: Gain control of all creatures until end of turn. Untap them. They gain
//!     haste until end of turn.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tibalt, the Fiend-Blooded");
    let tibalt = reg.interner_mut().intern("Tibalt");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tibalt);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(2),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw a card, then discard a card at random.".into(),
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
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-4: Tibalt deals damage equal to the number of cards in \
                       target player's hand to that player.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: Gain control of all creatures until end of turn. \
                       Untap them. They gain haste until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six,
            }),
    )
}

fn plus_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::Random,
        },
    ]
}

fn minus_four(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let amount = state.zone_count(Zone::Hand(*p)) as u32;
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Player(*p),
        amount,
    }]
}

fn minus_six(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Gain control of all creatures until end of turn, untap them, and grant
    // haste until end of turn.
    let filter = ObjectFilter::creature();
    let ids = script::ids_matching(state, &filter, ctx.controller);
    let mut out = Vec::new();
    for id in ids {
        out.push(Effect::ChangeControlEot { target: id, new_controller: ctx.controller });
        out.push(Effect::Untap { target: id });
        out.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        });
    }
    out
}

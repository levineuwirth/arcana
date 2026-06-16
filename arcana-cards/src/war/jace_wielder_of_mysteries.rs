//! Jace, Wielder of Mysteries — `{1}{U}{U}{U}` Legendary Planeswalker —
//! Jace, starting loyalty 5.
//!
//! Static (GAP): If you would draw a card while your library has no cards in
//! it, you win the game instead. (Not a loyalty ability; a draw-replacement
//! alternate-win not in the demonstrated surface — omitted.)
//! +1: Target player mills two cards. Draw a card.
//! −8: Draw seven cards. Then if your library has no cards in it, you win
//!     the game. (Partial — the draw is expressed; the alternate-win
//!     conditional is a GAP.)

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace, Wielder of Mysteries");
    let jace = reg.interner_mut().intern("Jace");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jace);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}{U}").expect("valid cost")),
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
                text: "+1: Target player mills two cards. Draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_mill_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: Draw seven cards. Then if your library has no cards in it, you win the game.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_draw,
            }),
    )
}

fn plus_one_mill_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::Mill { player: *p, count: 2 },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ]
}

fn minus_eight_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // PARTIAL: "Then if your library has no cards in it, you win the game" is
    //          an alternate-win conditional not expressible from the
    //          demonstrated surface. The draw is faithful.
    vec![Effect::DrawCards { player: ctx.controller, count: 7 }]
}

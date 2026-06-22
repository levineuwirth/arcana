//! Lore Weaver — `{3}{U}` 2/2 Creature — Human Wizard. U.
//! "Partner with Ley Weaver" — Partner / Partner with are not in the
//! demonstrated keyword surface, and the ETB tutor-a-named-card-for-a-
//! target-player half has no expressible form (tutor effects act on the
//! controller's own library, not a target player's). GAP both halves.
//! "{5}{U}{U}: Target player draws two cards." — wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lore Weaver");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "Partner with Ley Weaver" — Partner/Partner-with keyword not in
    // surface; the ETB "target player may put Ley Weaver into their hand from
    // their library, then shuffle" tutors a target player's library by name,
    // which has no expressible effect form.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{5}{U}{U}: Target player draws two cards.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}{U}{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: target_player_draws_two,
        }),
    )
}

fn target_player_draws_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::DrawCards { player: *p, count: 2 }]
}

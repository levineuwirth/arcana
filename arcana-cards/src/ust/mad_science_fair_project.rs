//! Mad Science Fair Project — `{3}` artifact.
//! "{T}: Roll a six-sided die. On a 3 or lower, target player adds
//! {C}. Otherwise, that player adds one mana of any color they
//! choose." The d6 "3 or lower" (an exact 50/50) is modeled with
//! Effect::FlipCoin; the any-color branch's resolution-time color
//! choice is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mad Science Fair Project");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Roll a six-sided die. On a 3 or lower, target \
                       player adds {C}. Otherwise, that player adds one \
                       mana of any color they choose."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_player()],
                // Targets a player, so not a mana ability (CR 605).
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: roll_for_mana,
            },
        ),
    )
}

fn roll_for_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: die roll modeled as a coin flip (d6 "3 or lower" is exactly
    // 50/50). The "win" arm is the 3-or-lower {C} branch.
    vec![Effect::FlipCoin {
        player: ctx.controller,
        win: Box::new(Effect::AddMana {
            player: *p,
            mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
        }),
        // GAP: "that player adds one mana of any color they choose" — a
        // resolution-time color choice by the target player is not
        // expressible; this branch no-ops.
        lose: Some(Box::new(Effect::Sequence(vec![]))),
    }]
}

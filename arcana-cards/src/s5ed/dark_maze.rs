//! Dark Maze — `{4}{U}` 4/5 Wall with Defender.
//! "{0}: This creature can attack this turn as though it didn't have
//! defender. Exile it at the beginning of the next end step."
//!
//! Defender wired. The {0} activated ability is wired: the
//! "can attack as though it didn't have defender" override is not
//! expressible in the documented Effect surface (GAP'd), but the
//! "Exile it at the beginning of the next end step" rider IS expressible
//! and is wired via a delayed Exile action.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dark Maze");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{0}: This creature can attack this turn as though it didn't have defender. Exile it at the beginning of the next end step.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{0}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: attack_then_exile,
            }),
    )
}

fn attack_then_exile(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "can attack this turn as though it didn't have defender" — no
    // defender-override effect is available in the demonstrated surface.
    // The "Exile it at the beginning of the next end step" rider is wired.
    vec![Effect::DelayedAction {
        source: ctx.source,
        controller: ctx.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::Exile,
    }]
}

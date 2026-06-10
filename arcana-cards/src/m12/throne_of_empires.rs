//! Throne of Empires — `{4}` artifact (Magic 2012, 2011).
//! "{1}, {T}: Create a 1/1 white Soldier creature token. Create five
//! of those tokens instead if you control artifacts named Crown of
//! Empires and Scepter of Empires."
//! One activated ability; the resolver counts the named artifacts on
//! the battlefield and mints one or five Soldier tokens.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Throne of Empires");
    let _ = reg.interner_mut().intern("Soldier");
    let _ = reg.interner_mut().intern("Crown of Empires");
    let _ = reg.interner_mut().intern("Scepter of Empires");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{1}, {T}: Create a 1/1 white Soldier creature token. Create five of those tokens instead if you control artifacts named Crown of Empires and Scepter of Empires.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_soldiers,
            },
        ),
    )
}

fn make_soldiers(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier").unwrap_or_default();
    let crown = reg.interner().lookup("Crown of Empires");
    let scepter = reg.interner().lookup("Scepter of Empires");
    let has_crown = script::count_matching(
        state,
        &ObjectFilter { name: crown, ..ObjectFilter::default() }
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    ) > 0;
    let has_scepter = script::count_matching(
        state,
        &ObjectFilter { name: scepter, ..ObjectFilter::default() }
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    ) > 0;
    let n = if has_crown && has_scepter { 5 } else { 1 };
    let mut effects = Vec::new();
    for _ in 0..n {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(soldier);
        effects.push(Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: soldier,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    effects
}

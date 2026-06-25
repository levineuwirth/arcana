//! Courtly Provocateur — `{2}{U}` 1/1 Human Wizard.
//! `{T}: Target creature attacks this turn if able.`
//! `{T}: Target creature blocks this turn if able.`
//!
//! Both abilities are tap activations targeting a creature with a
//! must-attack / must-block combat requirement. The must-attack ability
//! installs a must-attack requirement (CR 508.1a) on the target this turn.
//! There is no must-block primitive, so the second ability's effect body
//! is GAP'd (empty).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("Courtly Provocateur");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target creature attacks this turn if able.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: must_attack,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target creature blocks this turn if able.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: must_block,
            }),
    )
}

fn must_attack(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // "Target creature attacks this turn if able." — install a must-attack
    // requirement on the target until end of turn.
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(ctx.source, *id, Duration::EndOfTurn),
    }]
}

fn must_block(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "target creature blocks this turn if able" — no force-block /
    // must-block effect primitive available.
    Vec::new()
}

//! Crookclaw Elder — `{5}{U}` 3/2 Creature — Bird Wizard.
//!
//! Flying.
//! Tap two untapped Birds you control: Draw a card.
//! Tap two untapped Wizards you control: Target creature gains flying
//! until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crookclaw Elder");
    let bird = reg.interner_mut().intern("Bird");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(wizard);

    // Filters for the two tap-other costs.
    let bird_filter = script::subtype_filter(reg, "Bird");
    let wizard_filter = script::subtype_filter(reg, "Wizard");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap two untapped Birds you control: Draw a card.".into(),
                cost: ActivationCost {
                    tap_other: Some(bird_filter),
                    tap_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap two untapped Wizards you control: Target creature gains flying until end of turn.".into(),
                cost: ActivationCost {
                    tap_other: Some(wizard_filter),
                    tap_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_flying,
            }),
    )
}

fn draw_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}

fn grant_flying(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Flying,
        duration: Duration::EndOfTurn,
    }]
}

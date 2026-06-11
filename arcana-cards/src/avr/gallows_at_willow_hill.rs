//! Gallows at Willow Hill — `{3}` artifact.
//! "{3}, {T}, Tap three untapped Humans you control: Destroy target
//! creature. Its controller creates a 1/1 white Spirit creature token
//! with flying."
//!
//! Cost: mana + tap + tapping three untapped Humans (`tap_other` with
//! `tap_other_count: 3`). Resolution destroys the target and mints the
//! Spirit for the target's controller.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gallows at Willow Hill");
    let human = reg.interner_mut().intern("Human");
    // Pre-intern the token subtype so the resolver's read-only lookup
    // finds it.
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}, {T}, Tap three untapped Humans you control: Destroy target creature. Its controller creates a 1/1 white Spirit creature token with flying.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                tap: true,
                tap_other: Some(
                    ObjectFilter::creature().with_subtypes_any(vec![human]),
                ),
                tap_other_count: 3,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: hang_and_haunt,
        }),
    )
}

/// "Destroy target creature. Its controller creates a 1/1 white Spirit
/// creature token with flying."
fn hang_and_haunt(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let controller = script::target_controller(state, *id, ctx.controller);
    let spirit = reg.interner().lookup("Spirit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit.clone());
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateToken {
            controller,
            token: TokenDefinition {
                name: spirit,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        },
    ]
}

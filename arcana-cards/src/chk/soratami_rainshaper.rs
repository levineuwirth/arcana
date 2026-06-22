//! Soratami Rainshaper — `{2}{U}` 2/1 blue Creature — Moonfolk Wizard.
//!
//! Flying
//! {3}, Return a land you control to its owner's hand: Target creature
//!   you control gains shroud until end of turn.
//!
//! Decomposition: Flying → `keywords`; the activated ability → one
//! `ActivatedAbilityDef`. The "Return a land you control to its owner's
//! hand" component of the cost is not expressible with the demonstrated
//! `ActivationCost` fields (there is no return-permanent cost), so it is
//! approximated with the mana cost only and the land-bounce is GAP'd. The
//! effect grants shroud until end of turn to a target creature you
//! control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soratami Rainshaper");
    let moonfolk = reg.interner_mut().intern("Moonfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moonfolk);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}, Return a land you control to its owner's hand: Target creature you control gains shroud until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                // GAP (cost): "Return a land you control to its owner's hand" is
                // not an ActivationCost field (no return-permanent cost);
                // approximated with the mana cost only.
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_shroud,
        }),
    )
}

fn grant_shroud(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Shroud,
        duration: Duration::EndOfTurn,
    }]
}

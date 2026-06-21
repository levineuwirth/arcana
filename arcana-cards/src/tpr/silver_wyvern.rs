//! Silver Wyvern — `{3}{U}{U}` 4/3 blue Drake with Flying.
//! "{U}: Change the target of target spell or ability that targets only
//! this creature. The new target must be a creature."
//!
//! Abilities:
//! 1. Flying (keyword).
//! 2. {U}: change the target of a spell/ability targeting only this
//!    creature — there is no target-redirection Effect in the catalog;
//!    the ability is wired (targeting a spell) but the redirect itself
//!    is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silver Wyvern");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{U}: Change the target of target spell or ability that targets only \
                   this creature. The new target must be a creature."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U}").unwrap(),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Spell(ObjectFilter::default()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: change_target,
        }),
    )
}

fn change_target(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "change the target of target spell or ability" — there is no
    // target-redirection Effect in the catalog (and the "targets only
    // this creature" / "new target must be a creature" constraints have
    // no expressible filter). Emitting nothing.
    Vec::new()
}

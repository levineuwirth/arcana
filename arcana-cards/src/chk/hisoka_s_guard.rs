//! Hisoka's Guard — `{1}{U}` 1/1 Creature — Human Wizard.
//!
//! Oracle:
//! * You may choose not to untap this creature during your untap step.
//! * {1}{U}, {T}: Target creature you control other than this creature has
//!   shroud for as long as this creature remains tapped.
//!
//! Decomposition: the mana+tap activated ability grants shroud to another
//! target creature you control.
//!
//! GAP: "You may choose not to untap this creature during your untap step" is
//!      a static untap-restriction with no expressible primitive; omitted.
//! GAP: the grant's duration "for as long as this creature remains tapped" has
//!      no matching Duration variant — approximated with Duration::EndOfTurn.
//! GAP: "other than this creature" self-exclusion is not an ObjectFilter
//!      predicate — the target filter is creature you control (restriction
//!      relaxed).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hisoka's Guard");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "You may choose not to untap this creature during your untap step"
    //      — static untap restriction, no expressible primitive.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{U}, {T}: Target creature you control other than this creature has shroud \
                   for as long as this creature remains tapped."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                tap: true,
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
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "for as long as this creature remains tapped" — no while-tapped
    //      Duration; approximated as Duration::EndOfTurn.
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Shroud,
        duration: Duration::EndOfTurn,
    }]
}

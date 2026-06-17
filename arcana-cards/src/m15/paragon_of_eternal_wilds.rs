//! Paragon of Eternal Wilds — `{3}{G}` 2/2 Human Druid.
//! "Other green creatures you control get +1/+1."
//! `{G}, {T}: Another target green creature you control gains trample until end of
//! turn.`
//!
//! The "+1/+1 to other green creatures" anthem is a static continuous ability with
//! no trigger/cost — GAP'd. The activated ability targets a green creature you
//! control and grants it trample until end of turn ("another" — self-exclusion on
//! the target filter is not expressible and is GAP'd).

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
    let name = reg.interner_mut().intern("Paragon of Eternal Wilds");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: "Other green creatures you control get +1/+1." — static anthem.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}, {T}: Another target green creature you control gains trample until end of turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You)
                            .with_colors(ColorSet::green()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_trample,
            }),
    )
}

fn grant_trample(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Another" (self-exclusion on the target filter) is not expressible.
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Trample,
        duration: Duration::EndOfTurn,
    }]
}

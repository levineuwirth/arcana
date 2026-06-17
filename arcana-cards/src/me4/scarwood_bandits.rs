//! Scarwood Bandits — `{2}{G}{G}` 2/2 Human Rogue with Forestwalk.
//! "{2}{G}, {T}: Unless an opponent pays {2}, gain control of target artifact for as
//! long as this creature remains on the battlefield." Modeled as an opponent-paid
//! optional payment whose failure hands you control of the artifact. The control
//! grant uses the permanent ChangeControl primitive (GAP: "for as long as this
//! remains on the battlefield" duration is not expressible).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scarwood Bandits");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Landwalk(forest)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{G}, {T}: Unless an opponent pays {2}, gain control of target artifact for as long as this creature remains on the battlefield.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gain_control_unless_paid,
        }),
    )
}

fn gain_control_unless_paid(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let opps = script::opponents(state, ctx.controller);
    let Some(&opp) = opps.first() else { return Vec::new(); };
    // "Unless an opponent pays {2}, gain control" — the opponent is prompted; on no
    // pay, the punishment (you gaining control) fires.
    vec![Effect::OptionalPayment {
        chooser: opp,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::ChangeControl {
            target: *id,
            new_controller: ctx.controller,
        })),
    }]
}

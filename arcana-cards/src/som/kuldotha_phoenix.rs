//! Kuldotha Phoenix — `{2}{R}{R}{R}` 4/4 Creature — Phoenix.
//! Flying, haste. Metalcraft — {4}: Return this card from your graveyard to the
//! battlefield. Activate only during your upkeep and only if you control three
//! or more artifacts.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kuldotha Phoenix");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}: Return this card from your graveyard to the battlefield. Activate only during your upkeep and only if you control three or more artifacts.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                // Gates legality on the Metalcraft / "three or more artifacts" clause.
                // GAP: the "only during your upkeep" timing restriction has no
                // step-based activation-condition helper, so it is omitted.
                activation_condition: Some(control_three_artifacts),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: return_self,
        }),
    )
}

fn control_three_artifacts(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_control_at_least(
        s,
        you,
        &ObjectFilter {
            types: Some(TypeLine::ARTIFACT.into()),
            ..Default::default()
        },
        3,
    )
}

fn return_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}

//! Kavaron Harrier — `{R}` 2/1 red Artifact Creature — Robot Soldier.
//! "Whenever this creature attacks, you may pay {2}. If you do, create a
//! 2/2 colorless Robot artifact creature token that's tapped and attacking.
//! Sacrifice that token at end of combat."
//!
//! GAP: effect — "pay {2} as an optional cost" (may-pay clause) not
//! expressible. Using CreateTokenSacEot as closest approximation and
//! ignoring the optional mana payment.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kavaron Harrier");
    let robot = reg.interner_mut().intern("Robot");
    let soldier = reg.interner_mut().intern("Soldier");
    let _robot_tok = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: create_robot_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_robot_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let robot = reg.interner().lookup("Robot").expect("Robot interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    let token = TokenDefinition {
        name: robot,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: optional mana payment not expressible; always create token
    vec![Effect::CreateTokenSacEot { controller: trig.controller, token }]
}

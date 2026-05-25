//! Goblin Kaboomist — `{1}{R}` 1/2 red Goblin Warrior. "At the beginning of your
//! upkeep, create a colorless artifact token named Land Mine with '{R}, Sacrifice this
//! token: This token deals 2 damage to target attacking creature without flying.' Then
//! flip a coin. If you lose the flip, this creature deals 2 damage to itself."
//!
//! GAP: Land Mine token has an activated ability ({R}, Sacrifice: deal 2 damage to
//! target attacking creature without flying) — token activated abilities are not yet
//! modeled in TokenDefinition. The token is emitted without its ability.
//! GAP: Coin flip conditional — no coin-flip mechanic in the effect catalog. The
//! self-damage on losing the flip is omitted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Kaboomist");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let _land_mine = reg.interner_mut().intern("Land Mine");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: create_land_mine,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_land_mine(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let land_mine = reg.interner().lookup("Land Mine")
        .expect("Land Mine interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(land_mine);
    // GAP: Land Mine activated ability ({R}, Sacrifice: deal 2 damage to target
    // attacking creature without flying) cannot be expressed in TokenDefinition.
    // GAP: Coin flip — no flip mechanic in effect catalog; self-damage on losing
    // the flip is omitted.
    let token = TokenDefinition {
        name: land_mine,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

//! Mad Ratter — `{3}{R}` 1/2 red Goblin. "Whenever you draw your second card
//! each turn, create two 1/1 black Rat creature tokens."
//!
//! GAP: TriggerCondition::CardDrawn has no "second card" filter. Using
//! CardDrawn with OncePerTurn as approximation (fires every draw).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mad Ratter");
    let goblin = reg.interner_mut().intern("Goblin");
    let _rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
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
                // GAP: no "second card" filter; using OncePerTurn as approximation
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let rat = reg.interner().lookup("Rat")
        .expect("Rat interned during register()");
    let make_rat = || {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(rat);
        TokenDefinition {
            name: rat,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        }
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: make_rat() },
        Effect::CreateToken { controller: trig.controller, token: make_rat() },
    ]
}

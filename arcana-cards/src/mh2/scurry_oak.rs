//! Scurry Oak — `{2}{G}` 1/2 Treefolk.
//! Evolve.
//! Whenever one or more +1/+1 counters are put on this creature, you
//! may create a 1/1 green Squirrel creature token.
//!
//! Evolve is a base keyword. The counter-added trigger creates the
//! Squirrel; the optional "you may" is resolved as always-create (the
//! engine has no optional-token gate to model the choice).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
// SubtypeSet imported above for both card subtypes and the token's subtype set.
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scurry Oak");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let _squirrel = reg.interner_mut().intern("Squirrel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Evolve],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::PlusOnePlusOne),
                    chapter: None,
                },
                intervening_if: None,
                effect: make_squirrel,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_squirrel(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let squirrel = match reg.interner().lookup("Squirrel") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: squirrel,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

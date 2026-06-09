//! Nightsquad Commando — `{2}{B}` 2/3 black Human Soldier.
//! "When this creature enters, if you attacked this turn, create a 1/1 white
//! Human Soldier creature token."
//!
//! Intervening-if "if you attacked this turn" wired via
//! `conditions::you_attacked_this_turn` (any creature you control was
//! declared as an attacker this turn).

use arcana_core::conditions;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nightsquad Commando");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // Intervening-if "if you attacked this turn" via
            // conditions::you_attacked_this_turn.
            intervening_if: Some(iif_you_attacked),
            effect: etb_soldier_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn iif_you_attacked(state: &GameState, _source: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::you_attacked_this_turn(state, you)
}

fn etb_soldier_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human = reg
        .interner()
        .lookup("Human")
        .expect("Human interned during register()");
    let soldier = reg
        .interner()
        .lookup("Soldier")
        .expect("Soldier interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let token = TokenDefinition {
        name: soldier,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

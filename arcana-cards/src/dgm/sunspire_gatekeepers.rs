//! Sunspire Gatekeepers — `{3}{W}` 2/4 white Creature — Human Soldier.
//! "When this creature enters, if you control two or more Gates, create a
//! 2/2 white Knight creature token with vigilance."
//! Intervening-if "two or more Gates" modeled via
//! `conditions::you_control_subtype_at_least`.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunspire Gatekeepers");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let _knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: Some(iif_two_or_more_gates),
                effect: etb_knight_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn iif_two_or_more_gates(state: &GameState, _source: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
    conditions::you_control_subtype_at_least(state, reg, you, "Gate", 2)
}

fn etb_knight_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let knight = reg.interner().lookup("Knight")
        .expect("Knight interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(knight);
    let token = TokenDefinition {
        name: knight,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

//! Bat Whisperer — `{3}{B}` 4/2 black creature (Vampire).
//! "When this creature enters, if an opponent lost life this turn,
//! create a 1/1 black Bat creature token with flying."
//!
//! Intervening-if "if an opponent lost life this turn" wired via
//! `conditions::an_opponent_lost_life_this_turn`.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Bat Whisperer");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let _ = reg.interner_mut().intern("Bat");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // Intervening-if "if an opponent lost life this turn" via
                // conditions::an_opponent_lost_life_this_turn.
                intervening_if: Some(iif_opponent_lost_life),
                effect: on_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn iif_opponent_lost_life(state: &GameState, _source: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::an_opponent_lost_life_this_turn(state, you)
}

fn on_enters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let bat_id = reg.interner().lookup("Bat")
        .expect("Bat interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(bat_id);
    let token = TokenDefinition {
        name: bat_id,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    // "if an opponent lost life this turn" enforced via intervening_if at
    // stack-add time (monotone within a turn — no resolution re-check needed).
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

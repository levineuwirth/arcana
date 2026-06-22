//! Thieves' Guild Enforcer — `{B}` 1/1 Human Rogue with Flash.
//! "Whenever this creature or another Rogue you control enters, each
//!  opponent mills two cards.
//!  As long as an opponent has eight or more cards in their graveyard,
//!  this creature gets +2/+1 and has deathtouch."
//!
//! Decomposed as: a keyword line (Flash) plus one enters trigger (any
//! Rogue you control, including this one → each opponent mills two). The
//! graveyard-threshold static buff is a conditional continuous effect
//! with no demonstrated primitive, so it is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thieves' Guild Enforcer");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let rogue_filter = script::subtype_filter(reg, "Rogue").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: static "As long as an opponent has 8+ cards in their graveyard,
    // this gets +2/+1 and has deathtouch" — a conditional continuous
    // buff; no triggered/activated primitive.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: rogue_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: each_opponent_mills_two,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn each_opponent_mills_two(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 2 })
        .collect()
}

//! Jaded Analyst — `{1}{U}` 3/2 Human Detective.
//!
//! Defender.
//! Whenever you draw your second card each turn, this creature loses
//!   defender and gains vigilance until end of turn.
//!
//! The "second card each turn" gate is an intervening-if on a per-draw
//! trigger (fires when exactly two cards have been drawn this turn).
//! Vigilance is granted; "loses defender" has no targeted keyword-
//! removal Effect, so that half is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jaded Analyst");
    let human = reg.interner_mut().intern("Human");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CardDrawn {
                player: ControllerConstraint::You,
            },
            intervening_if: Some(if_second_draw),
            effect: gain_vigilance,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_second_draw(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::cards_drawn_this_turn(s, you) == 2
}

fn gain_vigilance(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses defender until end of turn" — no targeted keyword-
    // removal Effect (LoseAllAbilities would strip everything).
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::Vigilance,
        duration: Duration::EndOfTurn,
    }]
}

//! Rose, Cutthroat Raider — `{2}{R}{R}` 3/2 Legendary Artifact Creature — Robot.
//! First strike.
//! Raid — At end of combat on your turn, if you attacked this turn, create a
//! Junk token for each opponent you attacked.
//! Whenever you sacrifice a Junk, add {R}.
//!
//! First strike is a usable keyword. The Raid trigger is GAP'd: there is no
//! Junk commodity-token primitive, no "opponents you attacked" accessor, and
//! no "you attacked this turn" intervening-if helper. The sacrifice-a-Junk
//! mana trigger IS expressible (`Sacrificed { Junk filter }` → add {R}).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rose, Cutthroat Raider");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let junk_filter = script::subtype_filter(reg, "Junk").controlled_by(ControllerConstraint::You);

    // GAP: Raid — "At end of combat on your turn, if you attacked this turn,
    // create a Junk token for each opponent you attacked" — no Junk
    // commodity-token primitive, no opponents-attacked accessor, no
    // "you attacked this turn" intervening-if helper.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::Sacrificed { filter: junk_filter },
                intervening_if: None,
                effect: add_red,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_red(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
    }]
}

//! Moonstone Harbinger — `{2}{B}` 1/3 Bat Warrior with Flying and Deathtouch.
//! "Whenever you gain or lose life during your turn, Bats you control get +1/+0
//! and gain deathtouch until end of turn. This ability triggers only once each
//! turn."
//!
//! Flying + Deathtouch are base keywords. The trigger fires on a life-GAIN
//! event (`LifeGained`); the "or lose life" half has no `LifeLost`
//! trigger-condition variant (GAP), and the "during your turn" restriction has
//! no demonstrated gate (the trigger may over-fire on the opponent's turn). The
//! once-each-turn clause maps to `TriggerFrequency::OncePerTurn`. The effect
//! pumps each Bat you control +1/+0 and grants deathtouch until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Moonstone Harbinger");
    let bat = reg.interner_mut().intern("Bat");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "or lose life" half — no LifeLost trigger condition; and
            //      "during your turn" has no demonstrated gate.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pump_bats,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_bats(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Bat")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: arcana_core::objects::NULL_OBJECT_ID,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Deathtouch],
        }),
    }]
}

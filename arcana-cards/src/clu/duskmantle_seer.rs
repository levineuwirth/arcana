//! Duskmantle Seer — `{2}{U}{B}` 4/4 Vampire Wizard.
//!
//! * Flying.
//! * At the beginning of your upkeep, each player reveals the top card of
//!   their library, loses life equal to that card's mana value, then puts it
//!   into their hand.
//!   // GAP: the per-card-mana-value life loss and the reveal-then-to-hand
//!   sequence is not expressible — there is no effect that reveals the top
//!   card, loses life equal to its mana value, and draws it. Each player
//!   drawing the revealed card is modeled as a DrawCards for each player;
//!   the variable life loss has no primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duskmantle Seer");
    let vampire = reg.interner_mut().intern("Vampire");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: each_player_draws_top,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn each_player_draws_top(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Each player puts the revealed top card into their hand (a draw).
    // GAP: "loses life equal to that card's mana value" — the life loss is
    // tied to the revealed card's mana value, which has no script accessor;
    // the reveal-then-loss half is unmodeled.
    let mut effects = Vec::new();
    for p in script::all_players(state) {
        effects.push(Effect::DrawCards { player: p, count: 1 });
    }
    if effects.is_empty() {
        effects.push(Effect::DrawCards {
            player: trig.controller,
            count: 1,
        });
    }
    vec![Effect::Sequence(effects)]
}

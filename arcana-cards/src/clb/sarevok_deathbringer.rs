//! Sarevok, Deathbringer — `{3}{B}` 3/4 Legendary Human Knight.
//! At the beginning of each player's end step, if no permanents left the
//! battlefield this turn, that player loses X life, where X is Sarevok's
//! power. (X = Sarevok's power via script::power_of; "that player" is the
//! active player whose end step it is. GAP — the intervening-if "if no
//! permanents left the battlefield this turn" has no conditions predicate,
//! so the gate is dropped and the loss fires each end step.)
//! Choose a Background. (GAP — the Background partner mechanic has no
//! primitive; not a usable KeywordAbility variant.)

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarevok, Deathbringer");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Choose a Background" — the Background second-commander mechanic
    // has no primitive and is not a usable KeywordAbility variant.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::Any,
            },
            // GAP: intervening-if "if no permanents left the battlefield this
            // turn" — no conditions predicate; gate dropped (fires each end step).
            intervening_if: None,
            effect: end_step_lose_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step_lose_life(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "that player" = the active player whose end step this is.
    let them = state.active_player();
    // X = Sarevok's power.
    let x = script::power_of(state, trig.source).max(0) as u32;
    vec![Effect::LoseLife {
        player: them,
        amount: x,
    }]
}

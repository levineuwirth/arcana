//! Vrock — `{3}{B}{B}` 3/3 Creature — Bird Demon.
//!
//! Flying.
//! Toxic Spores — At the beginning of your end step, if a permanent you
//! controlled left the battlefield this turn, each opponent loses 3 life.
//!
//! # Decomposition
//! * Keyword line — `Flying`.
//! * "At the beginning of your end step … each opponent loses 3 life" → an
//!   end-step trigger (id 1) that drains every opponent for 3.
//! * GAP: the intervening-if "if a permanent you controlled left the
//!   battlefield this turn" has no matching `conditions::` predicate, so it is
//!   left as `None` (the trigger fires unconditionally — a documented
//!   over-fire, preferred over GAP-ing the whole ability).

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
    let name = reg.interner_mut().intern("Vrock");
    let bird = reg.interner_mut().intern("Bird");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: intervening-if "if a permanent you controlled left the
            // battlefield this turn" — no matching condition predicate.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: drain_each_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn drain_each_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let losses = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 3 })
        .collect();
    vec![Effect::Sequence(losses)]
}

//! Rishadan Brigand — `{4}{U}` 3/2 Human Pirate.
//! Flying.
//! When this creature enters, each opponent sacrifices a permanent of their
//! choice unless they pay {3}.
//! This creature can block only creatures with flying. (GAP — static blocking
//! restriction; no expressible primitive.)
//!
//! Decomposition:
//! * Keyword line → `KeywordAbility::Flying`.
//! * One ETB trigger: per opponent, an `OptionalPayment` of {3}; if they
//!   decline (the punishment branch), they sacrifice a permanent.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rishadan Brigand");
    let human = reg.interner_mut().intern("Human");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "This creature can block only creatures with flying" — no
    // blocking-restriction primitive in the catalog.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_each_opponent_sac_unless_pay,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_each_opponent_sac_unless_pay(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        // "sacrifices a permanent of their choice unless they pay {3}":
        // prompt that opponent to pay {3}; on decline, they sacrifice one.
        effects.push(Effect::OptionalPayment {
            chooser: opp,
            cost: OptionalPaymentKind::Mana(ManaCost::parse("{3}").expect("valid cost")),
            then: Box::new(Effect::Sequence(vec![])),
            else_effect: Some(Box::new(Effect::Sacrifice {
                player: opp,
                filter: ObjectFilter::permanent(),
                count: 1,
            })),
        });
    }
    effects
}

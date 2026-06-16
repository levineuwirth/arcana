//! Ayara's Oathsworn — `{1}{B}` 2/2 Human Knight with Menace.
//! "Whenever this creature deals combat damage to a player, if it has fewer
//!  than four +1/+1 counters on it, put a +1/+1 counter on it. Then if it has
//!  exactly four +1/+1 counters on it, search your library for a card, put it
//!  into your hand, then shuffle."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ayara's Oathsworn");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: on_combat_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_combat_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let counters = state
        .objects
        .get(trig.source)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    let mut out = Vec::new();
    // "if it has fewer than four +1/+1 counters on it, put a +1/+1 counter on it."
    let post = if counters < 4 {
        out.push(Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
        counters + 1
    } else {
        counters
    };
    // "Then if it has exactly four +1/+1 counters on it, search your library
    // for a card, put it into your hand, then shuffle." Evaluated against the
    // counter count after the conditional increment above.
    if post == 4 {
        out.push(Effect::TutorToHand {
            player: trig.controller,
            filter: ObjectFilter::default(),
            reveal: false,
        });
    }
    out
}

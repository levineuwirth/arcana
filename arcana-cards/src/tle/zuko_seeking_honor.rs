//! Zuko, Seeking Honor — `{2}{B/R}` 2/2 black/red Legendary Creature — Human Noble.
//!
//! Firebending 1 (Whenever this creature attacks, add {R}. ...)
//! Whenever you cast a noncreature spell, Zuko gains first strike until
//!   end of turn.
//! Whenever Zuko deals combat damage to a player, put a +1/+1 counter on him.
//!
//! Decomposition: Firebending 1 is not in the keyword surface (no
//! KeywordAbility variant) and its "add {R} that lasts until end of
//! combat" is not expressible (no combat-scoped mana duration), so it is
//! GAP'd. The two remaining triggers → two `TriggeredAbilityDef`s: a
//! noncreature-spell cast grants first strike EOT; combat damage to a
//! player adds a +1/+1 counter to the source.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zuko, Seeking Honor");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: keyword "Firebending 1" — no KeywordAbility variant; its
        // "add {R} until end of combat" is not expressible (no combat-scoped
        // mana duration).
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: gain_first_strike,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter {
                        name: Some(name),
                        ..ObjectFilter::default()
                    },
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: add_counter_to_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_first_strike(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::FirstStrike,
        duration: Duration::EndOfTurn,
    }]
}

fn add_counter_to_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
